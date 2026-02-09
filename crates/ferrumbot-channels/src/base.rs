use anyhow::Result;
use async_trait::async_trait;

use ferrumbot_core::OutboundMessage;

#[async_trait]
pub trait BaseChannel: Send + Sync {
    fn name(&self) -> &'static str;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn send(&self, msg: OutboundMessage) -> Result<()>;
    async fn is_running(&self) -> bool;
}
