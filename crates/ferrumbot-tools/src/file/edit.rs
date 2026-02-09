use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::{Tool, ToolContext};

use super::common::{Access, edit_parameters, parse_edit_args};

pub struct EditFileTool {
    access: Access,
}

impl EditFileTool {
    pub fn new(allowed_dir: Option<std::path::PathBuf>) -> Self {
        Self {
            access: Access::new(allowed_dir),
        }
    }
}

#[async_trait]
impl Tool for EditFileTool {
    fn name(&self) -> &'static str {
        "edit_file"
    }

    fn description(&self) -> &'static str {
        "Replace old_text with new_text in a file."
    }

    fn parameters(&self) -> Value {
        edit_parameters()
    }

    async fn execute(&self, args: Value, _ctx: ToolContext) -> Result<String> {
        let req = parse_edit_args(&args);
        let old_text = req.old_text;
        let new_text = req.new_text;

        let path = self.access.resolve(req.path)?;
        let mut content = tokio::fs::read_to_string(&path).await?;

        if !content.contains(old_text) {
            return Ok(
                "Error: old_text not found in file. Make sure it matches exactly.".to_string(),
            );
        }
        if content.matches(old_text).count() > 1 {
            return Ok(format!(
                "Warning: old_text appears {} times. Please provide more context to make it unique.",
                content.matches(old_text).count()
            ));
        }

        content = content.replacen(old_text, new_text, 1);
        tokio::fs::write(&path, content).await?;
        Ok(format!("Successfully edited {}", path.display()))
    }
}
