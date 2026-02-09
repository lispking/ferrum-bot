use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use ferrumbot_config::{ChannelsConfig, WhatsAppCloudApiConfig};
use ferrumbot_core::OutboundMessage;

use crate::BaseChannel;

pub struct WhatsAppCloudChannel {
    cloud: WhatsAppCloudApiConfig,
    running: Arc<RwLock<bool>>,
    client: Client,
}

impl WhatsAppCloudChannel {
    pub fn new(channels: ChannelsConfig) -> Self {
        Self {
            cloud: channels.whatsapp.cloud_api,
            running: Arc::new(RwLock::new(false)),
            client: Client::new(),
        }
    }

    fn api_url(&self) -> String {
        format!(
            "https://graph.facebook.com/v20.0/{}/messages",
            self.cloud.phone_number_id
        )
    }
}

#[async_trait]
impl BaseChannel for WhatsAppCloudChannel {
    fn name(&self) -> &'static str {
        "whatsapp"
    }

    async fn start(&self) -> Result<()> {
        *self.running.write().await = true;
        if self.cloud.access_token.is_empty() || self.cloud.phone_number_id.is_empty() {
            warn!("whatsapp cloud api enabled but credentials are missing");
        } else {
            info!("whatsapp cloud channel ready");
        }
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        *self.running.write().await = false;
        Ok(())
    }

    async fn send(&self, msg: OutboundMessage) -> Result<()> {
        if self.cloud.access_token.is_empty() || self.cloud.phone_number_id.is_empty() {
            warn!("skip whatsapp send: credentials are missing");
            return Ok(());
        }

        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "to": msg.chat_id,
            "type": "text",
            "text": {
                "body": msg.content
            }
        });

        let resp = self
            .client
            .post(self.api_url())
            .bearer_auth(&self.cloud.access_token)
            .json(&payload)
            .send()
            .await?;

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            error!("whatsapp cloud send failed: {}", body);
        }

        Ok(())
    }

    async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}
