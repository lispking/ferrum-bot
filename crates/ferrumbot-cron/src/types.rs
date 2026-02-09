use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CronScheduleKind {
    At,
    Every,
    Cron,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CronScheduleDef {
    pub kind: CronScheduleKind,
    #[serde(rename = "atMs")]
    pub at_ms: Option<i64>,
    #[serde(rename = "everyMs")]
    pub every_ms: Option<i64>,
    pub expr: Option<String>,
    pub tz: Option<String>,
}

impl Default for CronScheduleDef {
    fn default() -> Self {
        Self {
            kind: CronScheduleKind::Every,
            at_ms: None,
            every_ms: None,
            expr: None,
            tz: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CronPayload {
    pub kind: String,
    pub message: String,
    pub deliver: bool,
    pub channel: Option<String>,
    pub to: Option<String>,
}

impl Default for CronPayload {
    fn default() -> Self {
        Self {
            kind: "agent_turn".to_string(),
            message: String::new(),
            deliver: false,
            channel: None,
            to: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CronJobState {
    #[serde(rename = "nextRunAtMs")]
    pub next_run_at_ms: Option<i64>,
    #[serde(rename = "lastRunAtMs")]
    pub last_run_at_ms: Option<i64>,
    #[serde(rename = "lastStatus")]
    pub last_status: Option<String>,
    #[serde(rename = "lastError")]
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CronJob {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub schedule: CronScheduleDef,
    pub payload: CronPayload,
    pub state: CronJobState,
    #[serde(rename = "createdAtMs")]
    pub created_at_ms: i64,
    #[serde(rename = "updatedAtMs")]
    pub updated_at_ms: i64,
    #[serde(rename = "deleteAfterRun")]
    pub delete_after_run: bool,
}

impl Default for CronJob {
    fn default() -> Self {
        let now = crate::now_ms();
        Self {
            id: uuid::Uuid::new_v4().to_string()[..8].to_string(),
            name: String::new(),
            enabled: true,
            schedule: CronScheduleDef::default(),
            payload: CronPayload::default(),
            state: CronJobState::default(),
            created_at_ms: now,
            updated_at_ms: now,
            delete_after_run: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CronStore {
    pub version: i64,
    pub jobs: Vec<CronJob>,
}

impl Default for CronStore {
    fn default() -> Self {
        Self {
            version: 1,
            jobs: Vec::new(),
        }
    }
}
