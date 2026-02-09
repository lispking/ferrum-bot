use std::collections::HashMap;

use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMessage {
    pub role: String,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub key: String,
    pub messages: Vec<SessionMessage>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Session {
    pub fn new(key: String) -> Self {
        let now = Utc::now();
        Self {
            key,
            messages: Vec::new(),
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
        }
    }

    pub fn add_message(&mut self, role: &str, content: &str) {
        self.messages.push(SessionMessage {
            role: role.to_string(),
            content: content.to_string(),
            timestamp: Local::now().to_rfc3339(),
        });
        self.updated_at = Utc::now();
    }

    pub fn get_history(&self, max_messages: usize) -> Vec<serde_json::Value> {
        let start = self.messages.len().saturating_sub(max_messages);
        self.messages[start..]
            .iter()
            .map(|m| json!({"role": m.role, "content": m.content}))
            .collect()
    }
}
