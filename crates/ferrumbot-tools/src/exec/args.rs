use std::path::{Path, PathBuf};

use serde_json::{Value, json};

pub(super) struct ExecRequest {
    pub command: String,
    pub cwd: PathBuf,
}

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "command": { "type": "string" },
            "working_dir": { "type": "string" }
        },
        "required": ["command"]
    })
}

pub(super) fn parse(args: &Value, default_working_dir: &Path) -> ExecRequest {
    let command = args
        .get("command")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let cwd = args
        .get("working_dir")
        .and_then(|v| v.as_str())
        .map(PathBuf::from)
        .unwrap_or_else(|| default_working_dir.to_path_buf());

    ExecRequest { command, cwd }
}
