use serde_json::{Value, json};

pub(super) struct FetchRequest {
    pub url: String,
    pub max_chars: usize,
}

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "url": { "type": "string" },
            "maxChars": { "type": "integer", "minimum": 100 }
        },
        "required": ["url"]
    })
}

pub(super) fn parse(args: &Value, default_max_chars: usize) -> FetchRequest {
    let url = args
        .get("url")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let max_chars = args
        .get("maxChars")
        .and_then(|v| v.as_u64())
        .map(|v| v as usize)
        .unwrap_or(default_max_chars);

    FetchRequest { url, max_chars }
}
