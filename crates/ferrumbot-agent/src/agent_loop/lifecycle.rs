use anyhow::Result;
use chrono::Utc;
use ferrumbot_core::{InboundMessage, OutboundMessage};
use tracing::{error, info};

use super::AgentLoop;

impl AgentLoop {
    pub async fn run(&self) {
        *self.running.lock().await = true;
        info!("agent loop started");
        while *self.running.lock().await {
            let Some(msg) = self.bus.consume_inbound().await else {
                continue;
            };

            match self.process_message(msg).await {
                Ok(Some(outbound)) => {
                    if let Err(err) = self.bus.publish_outbound(outbound).await {
                        error!("failed to publish outbound: {err:#}");
                    }
                }
                Ok(None) => {}
                Err(err) => {
                    error!("error processing message: {err:#}");
                }
            }
        }
    }

    pub async fn stop(&self) {
        *self.running.lock().await = false;
    }

    pub async fn process_direct(
        &self,
        content: &str,
        session_key: &str,
        channel: &str,
        chat_id: &str,
    ) -> Result<String> {
        let msg = InboundMessage {
            channel: channel.to_string(),
            sender_id: "user".to_string(),
            chat_id: chat_id.to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
            media: Vec::new(),
            metadata: Default::default(),
        };

        let response = self.process_message_with_session(msg, session_key).await?;
        Ok(response.map(|m| m.content).unwrap_or_default())
    }

    async fn process_message(&self, msg: InboundMessage) -> Result<Option<OutboundMessage>> {
        let session_key = msg.session_key();
        self.process_message_with_session(msg, &session_key).await
    }
}
