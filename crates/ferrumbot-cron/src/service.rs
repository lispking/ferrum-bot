use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep};
use tracing::{error, info};

use crate::{
    CronJob, CronJobState, CronPayload, CronScheduleDef, CronScheduleKind, CronStore, JobCallback,
    compute_next_run, load_store, now_ms, save_store,
};

#[derive(Clone)]
pub struct CronService {
    store_path: PathBuf,
    store: Arc<Mutex<CronStore>>,
    running: Arc<Mutex<bool>>,
    callback: Arc<Mutex<Option<JobCallback>>>,
    timer_task: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl CronService {
    pub async fn new(store_path: PathBuf) -> Result<Self> {
        let store = load_store(&store_path).await?;
        Ok(Self {
            store_path,
            store: Arc::new(Mutex::new(store)),
            running: Arc::new(Mutex::new(false)),
            callback: Arc::new(Mutex::new(None)),
            timer_task: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn set_on_job(&self, callback: JobCallback) {
        *self.callback.lock().await = Some(callback);
    }

    pub async fn start(&self) -> Result<()> {
        {
            let mut running = self.running.lock().await;
            if *running {
                return Ok(());
            }
            *running = true;
        }

        self.recompute_next_runs().await;
        self.save().await?;

        let cloned = self.clone();
        let task = tokio::spawn(async move {
            loop {
                if !*cloned.running.lock().await {
                    break;
                }
                if let Err(err) = cloned.tick().await {
                    error!("cron tick failed: {err:#}");
                }
                sleep(Duration::from_secs(1)).await;
            }
        });

        *self.timer_task.lock().await = Some(task);
        Ok(())
    }

    pub async fn stop(&self) {
        *self.running.lock().await = false;
        if let Some(task) = self.timer_task.lock().await.take() {
            task.abort();
        }
    }

    pub async fn list_jobs(&self, include_disabled: bool) -> Vec<CronJob> {
        let store = self.store.lock().await;
        let mut jobs: Vec<CronJob> = if include_disabled {
            store.jobs.clone()
        } else {
            store.jobs.iter().filter(|j| j.enabled).cloned().collect()
        };
        jobs.sort_by_key(|j| j.state.next_run_at_ms.unwrap_or(i64::MAX));
        jobs
    }

    pub async fn add_job(
        &self,
        name: String,
        schedule: CronScheduleDef,
        payload: CronPayload,
        delete_after_run: bool,
    ) -> Result<CronJob> {
        let now = now_ms();
        let mut job = CronJob {
            id: uuid::Uuid::new_v4().to_string()[..8].to_string(),
            name,
            enabled: true,
            schedule,
            payload,
            state: CronJobState::default(),
            created_at_ms: now,
            updated_at_ms: now,
            delete_after_run,
        };
        job.state.next_run_at_ms = compute_next_run(&job.schedule, now);

        let mut store = self.store.lock().await;
        store.jobs.push(job.clone());
        drop(store);

        self.save().await?;
        Ok(job)
    }

    pub async fn remove_job(&self, job_id: &str) -> Result<bool> {
        let mut store = self.store.lock().await;
        let before = store.jobs.len();
        store.jobs.retain(|job| job.id != job_id);
        let removed = store.jobs.len() < before;
        drop(store);
        if removed {
            self.save().await?;
        }
        Ok(removed)
    }

    pub async fn enable_job(&self, job_id: &str, enabled: bool) -> Result<Option<CronJob>> {
        let mut store = self.store.lock().await;
        let mut out = None;
        for job in &mut store.jobs {
            if job.id == job_id {
                job.enabled = enabled;
                job.updated_at_ms = now_ms();
                job.state.next_run_at_ms = if enabled {
                    compute_next_run(&job.schedule, now_ms())
                } else {
                    None
                };
                out = Some(job.clone());
                break;
            }
        }
        drop(store);

        if out.is_some() {
            self.save().await?;
        }
        Ok(out)
    }

    pub async fn run_job(&self, job_id: &str, force: bool) -> Result<bool> {
        let job = {
            let store = self.store.lock().await;
            store.jobs.iter().find(|job| job.id == job_id).cloned()
        };

        let Some(job) = job else {
            return Ok(false);
        };

        if !force && !job.enabled {
            return Ok(false);
        }

        let changed = self.execute_job(job).await?;
        if changed {
            self.save().await?;
        }
        Ok(true)
    }

    pub async fn status(&self) -> serde_json::Value {
        let store = self.store.lock().await;
        let next_wake = store
            .jobs
            .iter()
            .filter(|job| job.enabled)
            .filter_map(|job| job.state.next_run_at_ms)
            .min();

        serde_json::json!({
            "enabled": *self.running.lock().await,
            "jobs": store.jobs.len(),
            "next_wake_at_ms": next_wake,
        })
    }

    async fn tick(&self) -> Result<()> {
        let due: Vec<CronJob> = {
            let store = self.store.lock().await;
            let now = now_ms();
            store
                .jobs
                .iter()
                .filter(|job| job.enabled)
                .filter(|job| job.state.next_run_at_ms.unwrap_or(i64::MAX) <= now)
                .cloned()
                .collect()
        };

        let mut changed = false;
        for job in due {
            changed |= self.execute_job(job).await?;
        }

        if changed {
            self.save().await?;
        }
        Ok(())
    }

    async fn execute_job(&self, mut job: CronJob) -> Result<bool> {
        info!("cron executing job {} ({})", job.name, job.id);
        let callback = self.callback.lock().await.clone();
        let started = now_ms();

        let mut status = "ok".to_string();
        let mut last_error = None;

        if let Some(callback) = callback
            && let Err(err) = callback(job.clone()).await
        {
            status = "error".to_string();
            last_error = Some(err.to_string());
        }

        let mut changed = false;
        {
            let mut store = self.store.lock().await;
            if let Some(found) = store.jobs.iter_mut().find(|x| x.id == job.id) {
                changed = true;
                found.state.last_run_at_ms = Some(started);
                found.state.last_status = Some(status);
                found.state.last_error = last_error;
                found.updated_at_ms = now_ms();

                if matches!(found.schedule.kind, CronScheduleKind::At) {
                    if found.delete_after_run {
                        job = found.clone();
                    } else {
                        found.enabled = false;
                        found.state.next_run_at_ms = None;
                    }
                } else {
                    found.state.next_run_at_ms = compute_next_run(&found.schedule, now_ms());
                }
            }

            if job.delete_after_run && matches!(job.schedule.kind, CronScheduleKind::At) {
                store.jobs.retain(|x| x.id != job.id);
                changed = true;
            }
        }

        Ok(changed)
    }

    async fn recompute_next_runs(&self) {
        let mut store = self.store.lock().await;
        let now = now_ms();
        for job in &mut store.jobs {
            if job.enabled {
                job.state.next_run_at_ms = compute_next_run(&job.schedule, now);
            }
        }
    }

    async fn save(&self) -> Result<()> {
        let store = self.store.lock().await.clone();
        save_store(&self.store_path, &store).await
    }
}
