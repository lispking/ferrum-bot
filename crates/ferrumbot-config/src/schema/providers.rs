use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ProviderConfig {
    pub api_key: String,
    pub api_base: Option<String>,
    pub extra_headers: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ProvidersConfig {
    pub anthropic: ProviderConfig,
    pub openai: ProviderConfig,
    pub openrouter: ProviderConfig,
    pub deepseek: ProviderConfig,
    pub groq: ProviderConfig,
    pub zhipu: ProviderConfig,
    pub dashscope: ProviderConfig,
    pub vllm: ProviderConfig,
    pub gemini: ProviderConfig,
    pub moonshot: ProviderConfig,
    pub aihubmix: ProviderConfig,
}
