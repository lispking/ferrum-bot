use serde_json::{Value, json};

use crate::ToolContext;

pub(super) struct MessageRequest {
    pub content: String,
    pub channel: String,
    pub chat_id: String,
}

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "content": { "type": "string" },
            "channel": { "type": "string" },
            "chat_id": { "type": "string" }
        },
        "required": ["content"]
    })
}

pub(super) fn parse(args: &Value, ctx: &ToolContext) -> MessageRequest {
    let content = args
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let channel = args
        .get("channel")
        .and_then(|v| v.as_str())
        .map(ToString::to_string)
        .or_else(|| ctx.current_channel.clone())
        .unwrap_or_else(|| "cli".to_string());
    let chat_id = args
        .get("chat_id")
        .and_then(|v| v.as_str())
        .map(ToString::to_string)
        .or_else(|| ctx.current_chat_id.clone())
        .unwrap_or_else(|| "direct".to_string());

    MessageRequest {
        content,
        channel,
        chat_id,
    }
}
