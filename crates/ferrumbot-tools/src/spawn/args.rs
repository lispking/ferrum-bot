use serde_json::{Value, json};

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "task": { "type": "string" }
        },
        "required": ["task"]
    })
}

pub(super) fn parse_task(args: &Value) -> &str {
    args.get("task")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
}
