use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde_json::json;

use super::{Session, SessionMessage};

pub fn load_session(path: &Path, key: &str) -> Result<Option<Session>> {
    if !path.exists() {
        return Ok(None);
    }

    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);
    let mut session = Session::new(key.to_string());

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let value: serde_json::Value = serde_json::from_str(&line)?;
        if value.get("_type") == Some(&serde_json::Value::String("metadata".to_string())) {
            hydrate_metadata(&mut session, &value)?;
            continue;
        }

        let msg: SessionMessage = serde_json::from_value(value)?;
        session.messages.push(msg);
    }

    Ok(Some(session))
}

pub fn save_session(path: &Path, session: &Session) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .with_context(|| format!("failed to open session file: {}", path.display()))?;

    let metadata = json!({
        "_type": "metadata",
        "created_at": session.created_at.to_rfc3339(),
        "updated_at": session.updated_at.to_rfc3339(),
        "metadata": session.metadata,
    });
    writeln!(file, "{}", serde_json::to_string(&metadata)?)?;

    for message in &session.messages {
        writeln!(file, "{}", serde_json::to_string(message)?)?;
    }

    Ok(())
}

fn hydrate_metadata(session: &mut Session, value: &serde_json::Value) -> Result<()> {
    if let Some(created_at) = value.get("created_at").and_then(|v| v.as_str()) {
        session.created_at = DateTime::parse_from_rfc3339(created_at)?.with_timezone(&Utc);
    }
    if let Some(updated_at) = value.get("updated_at").and_then(|v| v.as_str()) {
        session.updated_at = DateTime::parse_from_rfc3339(updated_at)?.with_timezone(&Utc);
    }
    if let Some(metadata) = value.get("metadata").and_then(|v| v.as_object()) {
        session.metadata = to_hash_map(metadata);
    }
    Ok(())
}

fn to_hash_map(
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> HashMap<String, serde_json::Value> {
    metadata.clone().into_iter().collect()
}
