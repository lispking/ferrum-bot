use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::{Tool, ToolContext};

use super::common::{Access, parse_path_args, path_parameters};

pub struct ReadFileTool {
    access: Access,
}

impl ReadFileTool {
    pub fn new(allowed_dir: Option<std::path::PathBuf>) -> Self {
        Self {
            access: Access::new(allowed_dir),
        }
    }
}

#[async_trait]
impl Tool for ReadFileTool {
    fn name(&self) -> &'static str {
        "read_file"
    }

    fn description(&self) -> &'static str {
        "Read the contents of a file at the given path."
    }

    fn parameters(&self) -> Value {
        path_parameters()
    }

    async fn execute(&self, args: Value, _ctx: ToolContext) -> Result<String> {
        let req = parse_path_args(&args);
        let path = self.access.resolve(req.path)?;
        if !path.exists() {
            return Ok(format!("Error: File not found: {}", path.display()));
        }
        if !path.is_file() {
            return Ok(format!("Error: Not a file: {}", path.display()));
        }
        Ok(tokio::fs::read_to_string(path).await?)
    }
}
