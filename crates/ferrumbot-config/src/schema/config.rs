use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::expand_tilde;

use super::{
    AgentsConfig, ChannelsConfig, GatewayConfig, ProviderConfig, ProvidersConfig, ToolsConfig,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct Config {
    pub agents: AgentsConfig,
    pub channels: ChannelsConfig,
    pub providers: ProvidersConfig,
    pub gateway: GatewayConfig,
    pub tools: ToolsConfig,
}

impl Config {
    pub fn workspace_path(&self) -> PathBuf {
        expand_tilde(&self.agents.defaults.workspace)
    }

    pub fn get_provider_for_model(&self, model: Option<&str>) -> Option<&ProviderConfig> {
        let model = model.unwrap_or(&self.agents.defaults.model).to_lowercase();
        let p = &self.providers;

        let entries: [(&str, &ProviderConfig); 16] = [
            ("aihubmix", &p.aihubmix),
            ("openrouter", &p.openrouter),
            ("deepseek", &p.deepseek),
            ("anthropic", &p.anthropic),
            ("claude", &p.anthropic),
            ("openai", &p.openai),
            ("gpt", &p.openai),
            ("gemini", &p.gemini),
            ("zhipu", &p.zhipu),
            ("glm", &p.zhipu),
            ("dashscope", &p.dashscope),
            ("qwen", &p.dashscope),
            ("groq", &p.groq),
            ("moonshot", &p.moonshot),
            ("kimi", &p.moonshot),
            ("vllm", &p.vllm),
        ];

        for (kw, provider) in entries {
            if model.contains(kw) && !provider.api_key.is_empty() {
                return Some(provider);
            }
        }

        [
            &p.openrouter,
            &p.aihubmix,
            &p.anthropic,
            &p.openai,
            &p.deepseek,
            &p.gemini,
            &p.zhipu,
            &p.dashscope,
            &p.moonshot,
            &p.vllm,
            &p.groq,
        ]
        .into_iter()
        .find(|provider| !provider.api_key.is_empty())
    }

    pub fn get_api_base(&self, model: Option<&str>) -> Option<String> {
        let provider = self.get_provider_for_model(model)?;
        if let Some(base) = &provider.api_base {
            return Some(base.clone());
        }

        if std::ptr::eq(provider, &self.providers.openrouter) {
            return Some("https://openrouter.ai/api/v1".to_string());
        }
        if std::ptr::eq(provider, &self.providers.aihubmix) {
            return Some("https://aihubmix.com/v1".to_string());
        }
        None
    }
}
