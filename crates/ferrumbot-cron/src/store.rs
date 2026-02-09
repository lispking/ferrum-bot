use std::path::Path;

use anyhow::{Context, Result};

use crate::CronStore;

pub async fn load_store(path: &Path) -> Result<CronStore> {
    if !path.exists() {
        return Ok(CronStore::default());
    }
    let raw = tokio::fs::read_to_string(path).await?;
    serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse cron store: {}", path.display()))
}

pub async fn save_store(path: &Path, store: &CronStore) -> Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let payload = serde_json::to_string_pretty(store)?;
    tokio::fs::write(path, payload).await?;
    Ok(())
}
