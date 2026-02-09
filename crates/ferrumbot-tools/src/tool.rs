use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::ToolContext;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn parameters(&self) -> Value;
    async fn execute(&self, args: Value, ctx: ToolContext) -> Result<String>;
}
