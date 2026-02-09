use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::LlmResponse;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn chat(
        &self,
        messages: Vec<Value>,
        tools: Option<Vec<Value>>,
        model: Option<&str>,
        max_tokens: Option<usize>,
        temperature: Option<f32>,
    ) -> Result<LlmResponse>;

    fn get_default_model(&self) -> &str;
}
