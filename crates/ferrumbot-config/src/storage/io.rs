use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::{Config, config_path};

use super::keys::{normalize_keys, to_camel_case_keys};

pub fn load_config(path: Option<&Path>) -> Result<Config> {
    let path = path.map(Path::to_path_buf).unwrap_or_else(config_path);
    if !path.exists() {
        return Ok(Config::default());
    }

    let raw = fs::read_to_string(&path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;
    let mut value: Value = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse config file: {}", path.display()))?;

    value = normalize_keys(value);
    migrate_legacy_fields(&mut value);

    serde_json::from_value(value).context("failed to deserialize config")
}

pub fn save_config(config: &Config, path: Option<&Path>) -> Result<()> {
    let path = path.map(Path::to_path_buf).unwrap_or_else(config_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create config directory: {}", parent.display()))?;
    }

    let value = serde_json::to_value(config)?;
    let value = to_camel_case_keys(value);
    let payload = serde_json::to_string_pretty(&value)?;
    fs::write(&path, payload)
        .with_context(|| format!("failed to write config file: {}", path.display()))?;
    Ok(())
}

fn migrate_legacy_fields(value: &mut Value) {
    let Some(obj) = value.as_object_mut() else {
        return;
    };

    if let Some(tools) = obj.get_mut("tools").and_then(Value::as_object_mut)
        && let Some(exec) = tools.get_mut("exec").and_then(Value::as_object_mut)
        && let Some(v) = exec.remove("restrict_to_workspace")
        && !tools.contains_key("restrict_to_workspace")
    {
        tools.insert("restrict_to_workspace".to_string(), v);
    }
}
