mod actions;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

use crate::{Tool, ToolContext};

pub struct CronTool;

#[async_trait]
impl Tool for CronTool {
    fn name(&self) -> &'static str {
        "cron"
    }

    fn description(&self) -> &'static str {
        "Manage scheduled tasks."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": { "type": "string", "enum": ["add", "remove", "list"] },
                "name": { "type": "string" },
                "message": { "type": "string" },
                "every": { "type": "integer" },
                "cron": { "type": "string" },
                "at": { "type": "integer" },
                "id": { "type": "string" }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, args: Value, ctx: ToolContext) -> Result<String> {
        let action = args
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let Some(cron) = ctx.cron else {
            return Ok("Error: cron service not configured".to_string());
        };

        actions::handle(action, &args, &cron).await
    }
}
