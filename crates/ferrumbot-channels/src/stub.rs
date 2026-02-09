use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::RwLock;
use tracing::warn;

use ferrumbot_core::OutboundMessage;

use crate::BaseChannel;

pub struct StubChannel {
    name: &'static str,
    reason: &'static str,
    running: Arc<RwLock<bool>>,
}

impl StubChannel {
    pub fn new(name: &'static str, reason: &'static str) -> Self {
        Self {
            name,
            reason,
            running: Arc::new(RwLock::new(false)),
        }
    }
}

#[async_trait]
impl BaseChannel for StubChannel {
    fn name(&self) -> &'static str {
        self.name
    }

    async fn start(&self) -> Result<()> {
        *self.running.write().await = true;
        warn!("{}", self.reason);
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        *self.running.write().await = false;
        Ok(())
    }

    async fn send(&self, msg: OutboundMessage) -> Result<()> {
        warn!("{} outbound (stub): {}", self.name, msg.content);
        Ok(())
    }

    async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}
