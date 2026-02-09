use anyhow::Result;
use ferrumbot_cron::{CronPayload, CronScheduleDef, CronScheduleKind, CronService};

use crate::app::{CronAction, CronCommand};

pub async fn run(cmd: CronCommand) -> Result<()> {
    let store_path = ferrumbot_config::data_dir().join("cron").join("jobs.json");
    let service = CronService::new(store_path).await?;

    match cmd.action {
        CronAction::List { all } => {
            let jobs = service.list_jobs(all).await;
            if jobs.is_empty() {
                println!("No scheduled jobs.");
            } else {
                println!("Scheduled Jobs");
                for job in jobs {
                    let schedule = match job.schedule.kind {
                        CronScheduleKind::Every => {
                            format!("every {}s", job.schedule.every_ms.unwrap_or(0) / 1000)
                        }
                        CronScheduleKind::Cron => job.schedule.expr.unwrap_or_default(),
                        CronScheduleKind::At => {
                            let ts = job.schedule.at_ms.unwrap_or(0);
                            chrono::DateTime::from_timestamp_millis(ts)
                                .map(|d| d.to_rfc3339())
                                .unwrap_or_else(|| "invalid".to_string())
                        }
                    };
                    println!(
                        "- {} | {} | {} | {}",
                        job.id,
                        job.name,
                        schedule,
                        if job.enabled { "enabled" } else { "disabled" }
                    );
                }
            }
        }
        CronAction::Add {
            name,
            message,
            every,
            cron,
            at,
            deliver,
            to,
            channel,
        } => {
            let schedule = if let Some(every) = every {
                CronScheduleDef {
                    kind: CronScheduleKind::Every,
                    every_ms: Some(every * 1000),
                    ..Default::default()
                }
            } else if let Some(expr) = cron {
                CronScheduleDef {
                    kind: CronScheduleKind::Cron,
                    expr: Some(expr),
                    ..Default::default()
                }
            } else if let Some(at) = at {
                let dt = chrono::DateTime::parse_from_rfc3339(&at)?;
                CronScheduleDef {
                    kind: CronScheduleKind::At,
                    at_ms: Some(dt.timestamp_millis()),
                    ..Default::default()
                }
            } else {
                anyhow::bail!("Must specify --every, --cron, or --at");
            };

            let payload = CronPayload {
                kind: "agent_turn".to_string(),
                message,
                deliver,
                channel,
                to,
            };

            let job = service.add_job(name, schedule, payload, false).await?;
            println!("✓ Added job '{}' ({})", job.name, job.id);
        }
        CronAction::Remove { job_id } => {
            if service.remove_job(&job_id).await? {
                println!("✓ Removed job {job_id}");
            } else {
                println!("Job {job_id} not found");
            }
        }
        CronAction::Enable { job_id, disable } => {
            if let Some(job) = service.enable_job(&job_id, !disable).await? {
                println!(
                    "✓ Job '{}' {}",
                    job.name,
                    if disable { "disabled" } else { "enabled" }
                );
            } else {
                println!("Job {job_id} not found");
            }
        }
        CronAction::Run { job_id, force } => {
            if service.run_job(&job_id, force).await? {
                println!("✓ Job executed");
            } else {
                println!("Failed to run job {job_id}");
            }
        }
    }

    Ok(())
}
