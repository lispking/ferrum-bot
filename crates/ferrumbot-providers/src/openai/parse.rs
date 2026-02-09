use std::collections::BTreeMap;

use serde_json::{Value, json};

use crate::{LlmResponse, ToolCallRequest};

pub(super) fn parse_chat_response(payload: Value, success: bool) -> LlmResponse {
    if !success {
        return LlmResponse {
            content: Some(format!("Error calling LLM: {}", payload)),
            tool_calls: Vec::new(),
            finish_reason: "error".to_string(),
            usage: BTreeMap::new(),
        };
    }

    let choice = payload
        .get("choices")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .cloned()
        .unwrap_or_else(|| json!({}));

    let message = choice.get("message").cloned().unwrap_or_else(|| json!({}));
    let content = message
        .get("content")
        .and_then(|v| v.as_str())
        .map(ToString::to_string);

    let mut tool_calls = Vec::new();
    if let Some(items) = message.get("tool_calls").and_then(|v| v.as_array()) {
        for item in items {
            let id = item
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let name = item
                .get("function")
                .and_then(|v| v.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let arguments = item
                .get("function")
                .and_then(|v| v.get("arguments"))
                .cloned()
                .unwrap_or_else(|| Value::Object(Default::default()));
            let parsed_arguments = if let Some(raw) = arguments.as_str() {
                serde_json::from_str(raw).unwrap_or_else(|_| json!({ "raw": raw }))
            } else {
                arguments
            };
            tool_calls.push(ToolCallRequest {
                id,
                name,
                arguments: parsed_arguments,
            });
        }
    }

    let mut usage = BTreeMap::new();
    if let Some(obj) = payload.get("usage").and_then(|v| v.as_object()) {
        for key in ["prompt_tokens", "completion_tokens", "total_tokens"] {
            if let Some(v) = obj.get(key).and_then(|x| x.as_i64()) {
                usage.insert(key.to_string(), v);
            }
        }
    }

    LlmResponse {
        content,
        tool_calls,
        finish_reason: choice
            .get("finish_reason")
            .and_then(|v| v.as_str())
            .unwrap_or("stop")
            .to_string(),
        usage,
    }
}
