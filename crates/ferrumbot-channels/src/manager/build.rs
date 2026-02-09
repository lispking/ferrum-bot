use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use ferrumbot_config::Config;
use ferrumbot_core::MessageBus;

use crate::{StubChannel, WhatsAppCloudChannel};

use super::ChannelManager;

impl ChannelManager {
    pub fn new(config: &Config, bus: MessageBus) -> Self {
        let mut channels = HashMap::new();

        if config.channels.telegram.enabled {
            channels.insert(
                "telegram".to_string(),
                Arc::new(StubChannel::new(
                    "telegram",
                    "Telegram channel not implemented yet",
                )) as Arc<dyn crate::BaseChannel>,
            );
        }

        if config.channels.discord.enabled {
            channels.insert(
                "discord".to_string(),
                Arc::new(StubChannel::new(
                    "discord",
                    "Discord channel not implemented yet",
                )) as Arc<dyn crate::BaseChannel>,
            );
        }

        if config.channels.feishu.enabled {
            channels.insert(
                "feishu".to_string(),
                Arc::new(StubChannel::new(
                    "feishu",
                    "Feishu channel not implemented yet",
                )) as Arc<dyn crate::BaseChannel>,
            );
        }

        if config.channels.whatsapp.enabled {
            channels.insert(
                "whatsapp".to_string(),
                Arc::new(WhatsAppCloudChannel::new(config.channels.clone()))
                    as Arc<dyn crate::BaseChannel>,
            );
        }

        Self {
            bus,
            channels,
            dispatch_task: Arc::new(RwLock::new(None)),
        }
    }

    pub fn enabled_channels(&self) -> Vec<String> {
        self.channels.keys().cloned().collect()
    }
}
