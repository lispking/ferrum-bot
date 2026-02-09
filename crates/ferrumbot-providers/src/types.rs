use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallRequest {
    pub id: String,
    pub name: String,
    pub arguments: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub content: Option<String>,
    pub tool_calls: Vec<ToolCallRequest>,
    pub finish_reason: String,
    pub usage: BTreeMap<String, i64>,
}

impl LlmResponse {
    pub fn has_tool_calls(&self) -> bool {
        !self.tool_calls.is_empty()
    }
}
