mod args;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::{Tool, ToolContext};

pub struct SpawnTool;

#[async_trait]
impl Tool for SpawnTool {
    fn name(&self) -> &'static str {
        "spawn"
    }

    fn description(&self) -> &'static str {
        "Spawn a background subagent task."
    }

    fn parameters(&self) -> Value {
        args::parameters()
    }

    async fn execute(&self, args: Value, _ctx: ToolContext) -> Result<String> {
        let task = args::parse_task(&args);
        Ok(format!("spawn scheduled (placeholder): {task}"))
    }
}
