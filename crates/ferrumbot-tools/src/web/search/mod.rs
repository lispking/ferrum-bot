mod args;
mod render;

use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

use crate::{Tool, ToolContext};

pub struct WebSearchTool {
    api_key: Option<String>,
    max_results: usize,
    client: Client,
}

impl WebSearchTool {
    pub fn new(api_key: Option<String>, max_results: usize) -> Self {
        Self {
            api_key,
            max_results,
            client: Client::builder()
                .timeout(Duration::from_secs(20))
                .build()
                .unwrap_or_else(|_| Client::new()),
        }
    }
}

#[async_trait]
impl Tool for WebSearchTool {
    fn name(&self) -> &'static str {
        "web_search"
    }

    fn description(&self) -> &'static str {
        "Search the web. Returns titles, URLs, and snippets."
    }

    fn parameters(&self) -> Value {
        args::parameters()
    }

    async fn execute(&self, args: Value, _ctx: ToolContext) -> Result<String> {
        let request = args::parse(&args, self.max_results);
        let api_key = self.api_key.clone().unwrap_or_default();
        if api_key.is_empty() {
            return Ok("Error: BRAVE_API_KEY not configured".to_string());
        }

        let resp: Value = self
            .client
            .get("https://api.search.brave.com/res/v1/web/search")
            .query(&[
                ("q", request.query.as_str()),
                ("count", &request.count.to_string()),
            ])
            .header("Accept", "application/json")
            .header("X-Subscription-Token", api_key)
            .send()
            .await?
            .json()
            .await?;

        Ok(render::to_text(&request.query, request.count, &resp))
    }
}
