use anyhow::Result;
use tokio::time::{Duration, sleep};
use tracing::{error, info, warn};

use super::ChannelManager;

impl ChannelManager {
    pub async fn start_all(&self) -> Result<()> {
        for (name, channel) in &self.channels {
            info!("starting channel: {name}");
            info!("channel ready: {}", channel.name());
            if let Err(err) = channel.start().await {
                error!("failed to start channel {name}: {err:#}");
            }
        }

        let bus = self.bus.clone();
        let channels = self.channels.clone();
        let handle = tokio::spawn(async move {
            loop {
                let Some(msg) = bus.consume_outbound().await else {
                    sleep(Duration::from_millis(200)).await;
                    continue;
                };
                if let Some(channel) = channels.get(&msg.channel) {
                    if let Err(err) = channel.send(msg).await {
                        error!("failed to send outbound message: {err:#}");
                    }
                } else {
                    warn!("unknown outbound channel: {}", msg.channel);
                }
            }
        });

        *self.dispatch_task.write().await = Some(handle);
        Ok(())
    }

    pub async fn stop_all(&self) -> Result<()> {
        if let Some(handle) = self.dispatch_task.write().await.take() {
            handle.abort();
        }
        for (name, channel) in &self.channels {
            if let Err(err) = channel.stop().await {
                error!("failed to stop channel {name}: {err:#}");
            }
        }
        Ok(())
    }
}
