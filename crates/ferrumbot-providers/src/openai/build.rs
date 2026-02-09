use std::collections::BTreeMap;

use anyhow::Result;
use reqwest::Client;

use ferrumbot_config::{Config, ProviderConfig};

use super::OpenAiCompatibleProvider;

impl OpenAiCompatibleProvider {
    pub fn from_config(config: &Config) -> Result<Self> {
        let model = config.agents.defaults.model.clone();
        let provider = config.get_provider_for_model(Some(&model));

        let (api_key, api_base, extra_headers) = if let Some(provider) = provider {
            (
                provider.api_key.clone(),
                config
                    .get_api_base(Some(&model))
                    .unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
                provider.extra_headers.clone().unwrap_or_default(),
            )
        } else {
            (
                String::new(),
                "https://api.openai.com/v1".to_string(),
                BTreeMap::new(),
            )
        };

        Ok(Self {
            client: Client::new(),
            api_key,
            api_base,
            default_model: model,
            extra_headers,
        })
    }

    pub fn from_provider(
        model: String,
        provider: &ProviderConfig,
        api_base: Option<String>,
    ) -> Self {
        Self {
            client: Client::new(),
            api_key: provider.api_key.clone(),
            api_base: api_base.unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            default_model: model,
            extra_headers: provider.extra_headers.clone().unwrap_or_default(),
        }
    }
}
