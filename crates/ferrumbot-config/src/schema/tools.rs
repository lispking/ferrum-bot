use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WebSearchConfig {
    pub api_key: String,
    pub max_results: u8,
}

impl Default for WebSearchConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            max_results: 5,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WebToolsConfig {
    pub search: WebSearchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ExecToolConfig {
    pub timeout: u64,
}

impl Default for ExecToolConfig {
    fn default() -> Self {
        Self { timeout: 60 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ToolsConfig {
    pub web: WebToolsConfig,
    pub exec: ExecToolConfig,
    pub restrict_to_workspace: bool,
}

impl Default for ToolsConfig {
    fn default() -> Self {
        Self {
            web: WebToolsConfig::default(),
            exec: ExecToolConfig::default(),
            restrict_to_workspace: true,
        }
    }
}
