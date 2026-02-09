use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AgentDefaults {
    pub workspace: String,
    pub model: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub max_tool_iterations: usize,
}

impl Default for AgentDefaults {
    fn default() -> Self {
        Self {
            workspace: "~/.ferrum-bot/workspace".to_string(),
            model: "anthropic/claude-opus-4-5".to_string(),
            max_tokens: 8192,
            temperature: 0.7,
            max_tool_iterations: 20,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AgentsConfig {
    pub defaults: AgentDefaults,
}
