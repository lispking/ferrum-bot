mod args;
mod validate;

use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{Value, json};

use crate::{Tool, ToolContext};

pub struct WebFetchTool {
    max_chars: usize,
    client: Client,
}

impl WebFetchTool {
    pub fn new(max_chars: usize) -> Self {
        Self {
            max_chars,
            client: Client::builder()
                .timeout(Duration::from_secs(20))
                .build()
                .unwrap_or_else(|_| Client::new()),
        }
    }
}

#[async_trait]
impl Tool for WebFetchTool {
    fn name(&self) -> &'static str {
        "web_fetch"
    }

    fn description(&self) -> &'static str {
        "Fetch URL and extract readable content."
    }

    fn parameters(&self) -> Value {
        args::parameters()
    }

    async fn execute(&self, args: Value, _ctx: ToolContext) -> Result<String> {
        let request = args::parse(&args, self.max_chars);

        if let Err(reason) = validate::validate_url(&request.url) {
            return Ok(
                json!({"error": format!("URL validation failed: {reason}"), "url": request.url})
                    .to_string(),
            );
        }

        let resp = self
            .client
            .get(&request.url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_7_2) AppleWebKit/537.36",
            )
            .send()
            .await?;

        let final_url = resp.url().to_string();
        let status = resp.status().as_u16();
        let mut text = resp.text().await?;
        let truncated = text.len() > request.max_chars;
        if truncated {
            text.truncate(request.max_chars);
        }

        Ok(json!({
            "url": request.url,
            "finalUrl": final_url,
            "status": status,
            "extractor": "raw",
            "truncated": truncated,
            "length": text.len(),
            "text": text,
        })
        .to_string())
    }
}
