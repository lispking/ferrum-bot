use std::path::PathBuf;

use anyhow::Result;
use serde_json::{Value, json};

use crate::resolve_path;

#[derive(Clone)]
pub(super) struct Access {
    allowed_dir: Option<PathBuf>,
}

impl Access {
    pub(super) fn new(allowed_dir: Option<PathBuf>) -> Self {
        Self { allowed_dir }
    }

    pub(super) fn resolve(&self, path: &str) -> Result<PathBuf> {
        resolve_path(path, self.allowed_dir.as_deref())
    }
}

pub(super) fn path_parameters() -> Value {
    json!({
        "type": "object",
        "properties": { "path": { "type": "string" } },
        "required": ["path"]
    })
}

pub(super) fn write_parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "path": { "type": "string" },
            "content": { "type": "string" }
        },
        "required": ["path", "content"]
    })
}

pub(super) fn edit_parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "path": { "type": "string" },
            "old_text": { "type": "string" },
            "new_text": { "type": "string" }
        },
        "required": ["path", "old_text", "new_text"]
    })
}

#[derive(Clone, Copy)]
pub(super) struct PathArgs<'a> {
    pub path: &'a str,
}

pub(super) struct WriteArgs<'a> {
    pub path: &'a str,
    pub content: &'a str,
}

pub(super) struct EditArgs<'a> {
    pub path: &'a str,
    pub old_text: &'a str,
    pub new_text: &'a str,
}

pub(super) fn parse_path_args(args: &Value) -> PathArgs<'_> {
    PathArgs {
        path: args
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or_default(),
    }
}

pub(super) fn parse_write_args(args: &Value) -> WriteArgs<'_> {
    WriteArgs {
        path: args
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or_default(),
        content: args
            .get("content")
            .and_then(|v| v.as_str())
            .unwrap_or_default(),
    }
}

pub(super) fn parse_edit_args(args: &Value) -> EditArgs<'_> {
    EditArgs {
        path: args
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or_default(),
        old_text: args
            .get("old_text")
            .and_then(|v| v.as_str())
            .unwrap_or_default(),
        new_text: args
            .get("new_text")
            .and_then(|v| v.as_str())
            .unwrap_or_default(),
    }
}
