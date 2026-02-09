use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::{Tool, ToolContext};

use super::common::{Access, parse_write_args, write_parameters};

pub struct WriteFileTool {
    access: Access,
}

impl WriteFileTool {
    pub fn new(allowed_dir: Option<std::path::PathBuf>) -> Self {
        Self {
            access: Access::new(allowed_dir),
        }
    }
}

#[async_trait]
impl Tool for WriteFileTool {
    fn name(&self) -> &'static str {
        "write_file"
    }

    fn description(&self) -> &'static str {
        "Write content to a file at the given path."
    }

    fn parameters(&self) -> Value {
        write_parameters()
    }

    async fn execute(&self, args: Value, _ctx: ToolContext) -> Result<String> {
        let req = parse_write_args(&args);
        let content = req.content;
        let path = self.access.resolve(req.path)?;
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(&path, content).await?;
        Ok(format!(
            "Successfully wrote {} bytes to {}",
            content.len(),
            path.display()
        ))
    }
}
