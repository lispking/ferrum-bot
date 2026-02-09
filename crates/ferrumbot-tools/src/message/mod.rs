mod args;
mod send;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::{Tool, ToolContext};

pub struct MessageTool;

#[async_trait]
impl Tool for MessageTool {
    fn name(&self) -> &'static str {
        "message"
    }

    fn description(&self) -> &'static str {
        "Send a message to current or specified chat channel."
    }

    fn parameters(&self) -> Value {
        args::parameters()
    }

    async fn execute(&self, args: Value, ctx: ToolContext) -> Result<String> {
        let request = args::parse(&args, &ctx);
        send::send_message(request, ctx).await
    }
}
