use serde_json::json;

use crate::ToolContext;

use super::default_registry;

#[tokio::test]
async fn registry_returns_validation_error() {
    let workspace = std::env::temp_dir().join("ferrumbot-tools-test");
    let reg = default_registry(workspace.clone(), None, None, None, 5, 5, false);
    let out = reg
        .execute(
            "write_file",
            json!({ "path": "x.txt" }),
            ToolContext {
                workspace,
                current_channel: None,
                current_chat_id: None,
                bus: None,
                cron: None,
            },
        )
        .await;
    assert!(out.contains("Invalid parameters"));
}
