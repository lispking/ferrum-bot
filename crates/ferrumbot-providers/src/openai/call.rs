use std::collections::BTreeMap;

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::{Value, json};

use crate::{LlmProvider, LlmResponse};

use super::OpenAiCompatibleProvider;
use super::parse::parse_chat_response;

#[async_trait]
impl LlmProvider for OpenAiCompatibleProvider {
    async fn chat(
        &self,
        messages: Vec<Value>,
        tools: Option<Vec<Value>>,
        model: Option<&str>,
        max_tokens: Option<usize>,
        temperature: Option<f32>,
    ) -> Result<LlmResponse> {
        if self.api_key.is_empty() {
            return Ok(LlmResponse {
                content: Some("Error calling LLM: API key not configured".to_string()),
                tool_calls: Vec::new(),
                finish_reason: "error".to_string(),
                usage: BTreeMap::new(),
            });
        }

        let model = self.normalize_model(model.unwrap_or(&self.default_model));
        let mut body = json!({
            "model": model,
            "messages": messages,
            "max_tokens": max_tokens.unwrap_or(4096),
            "temperature": temperature.unwrap_or(0.7),
        });

        if let Some(tools) = tools {
            body["tools"] = Value::Array(tools);
            body["tool_choice"] = Value::String("auto".to_string());
        }

        let url = format!("{}/chat/completions", self.api_base.trim_end_matches('/'));

        let mut req = self
            .client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .header("Content-Type", "application/json");

        for (k, v) in &self.extra_headers {
            req = req.header(k, v);
        }

        let resp = req.send().await.context("failed to call provider")?;
        let success = resp.status().is_success();
        let payload: Value = resp
            .json()
            .await
            .context("provider response is not valid JSON")?;

        Ok(parse_chat_response(payload, success))
    }

    fn get_default_model(&self) -> &str {
        &self.default_model
    }
}
