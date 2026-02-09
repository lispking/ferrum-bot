use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::{Tool, ToolContext};

use super::common::{Access, parse_path_args, path_parameters};

pub struct ListDirTool {
    access: Access,
}

impl ListDirTool {
    pub fn new(allowed_dir: Option<std::path::PathBuf>) -> Self {
        Self {
            access: Access::new(allowed_dir),
        }
    }
}

#[async_trait]
impl Tool for ListDirTool {
    fn name(&self) -> &'static str {
        "list_dir"
    }

    fn description(&self) -> &'static str {
        "List the contents of a directory."
    }

    fn parameters(&self) -> Value {
        path_parameters()
    }

    async fn execute(&self, args: Value, _ctx: ToolContext) -> Result<String> {
        let req = parse_path_args(&args);
        let dir = self.access.resolve(req.path)?;

        if !dir.exists() {
            return Ok(format!("Error: Directory not found: {}", dir.display()));
        }
        if !dir.is_dir() {
            return Ok(format!("Error: Not a directory: {}", dir.display()));
        }

        let mut rows = Vec::new();
        let mut rd = tokio::fs::read_dir(&dir).await?;
        while let Some(entry) = rd.next_entry().await? {
            let p = entry.path();
            let name = p
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("(invalid)");
            let prefix = if p.is_dir() { "📁" } else { "📄" };
            rows.push(format!("{prefix} {name}"));
        }
        rows.sort();

        if rows.is_empty() {
            Ok(format!("Directory {} is empty", dir.display()))
        } else {
            Ok(rows.join("\n"))
        }
    }
}
