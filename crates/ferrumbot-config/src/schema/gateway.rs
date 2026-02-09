use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GatewayConfig {
    pub host: String,
    pub port: u16,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 18790,
        }
    }
}
