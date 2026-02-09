use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WhatsAppCloudApiConfig {
    pub access_token: String,
    pub phone_number_id: String,
    pub verify_token: String,
    pub app_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WhatsAppConfig {
    pub enabled: bool,
    pub bridge_url: String,
    pub allow_from: Vec<String>,
    pub cloud_api: WhatsAppCloudApiConfig,
}

impl Default for WhatsAppConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            bridge_url: "ws://localhost:3001".to_string(),
            allow_from: Vec::new(),
            cloud_api: WhatsAppCloudApiConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct TelegramConfig {
    pub enabled: bool,
    pub token: String,
    pub allow_from: Vec<String>,
    pub proxy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DiscordConfig {
    pub enabled: bool,
    pub token: String,
    pub allow_from: Vec<String>,
    pub gateway_url: String,
    pub intents: i64,
}

impl Default for DiscordConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            token: String::new(),
            allow_from: Vec::new(),
            gateway_url: "wss://gateway.discord.gg/?v=10&encoding=json".to_string(),
            intents: 37377,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct FeishuConfig {
    pub enabled: bool,
    pub app_id: String,
    pub app_secret: String,
    pub encrypt_key: String,
    pub verification_token: String,
    pub allow_from: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ChannelsConfig {
    pub whatsapp: WhatsAppConfig,
    pub telegram: TelegramConfig,
    pub discord: DiscordConfig,
    pub feishu: FeishuConfig,
}
