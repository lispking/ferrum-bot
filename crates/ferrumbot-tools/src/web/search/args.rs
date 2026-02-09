use serde_json::{Value, json};

pub(super) struct SearchRequest {
    pub query: String,
    pub count: usize,
}

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "query": { "type": "string" },
            "count": { "type": "integer", "minimum": 1, "maximum": 10 }
        },
        "required": ["query"]
    })
}

pub(super) fn parse(args: &Value, default_count: usize) -> SearchRequest {
    let query = args
        .get("query")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let count = args
        .get("count")
        .and_then(|v| v.as_u64())
        .map(|v| v as usize)
        .unwrap_or(default_count)
        .clamp(1, 10);

    SearchRequest { query, count }
}
