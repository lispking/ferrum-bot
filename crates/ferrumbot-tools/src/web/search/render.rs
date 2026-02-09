use serde_json::Value;

pub(super) fn to_text(query: &str, count: usize, resp: &Value) -> String {
    let results = resp
        .get("web")
        .and_then(|v| v.get("results"))
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    if results.is_empty() {
        return format!("No results for: {query}");
    }

    let mut lines = vec![format!("Results for: {query}\n")];
    for (i, item) in results.iter().take(count).enumerate() {
        let title = item.get("title").and_then(|v| v.as_str()).unwrap_or("");
        let url = item.get("url").and_then(|v| v.as_str()).unwrap_or("");
        lines.push(format!("{}. {}\n   {}", i + 1, title, url));
        if let Some(desc) = item.get("description").and_then(|v| v.as_str()) {
            lines.push(format!("   {desc}"));
        }
    }

    lines.join("\n")
}
