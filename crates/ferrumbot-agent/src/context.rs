use std::path::PathBuf;

use serde_json::{Value, json};

pub struct ContextBuilder {
    workspace: PathBuf,
}

impl ContextBuilder {
    pub fn new(workspace: PathBuf) -> Self {
        Self { workspace }
    }

    pub fn build_system_prompt(&self, channel: Option<&str>, chat_id: Option<&str>) -> String {
        let now = chrono::Local::now()
            .format("%Y-%m-%d %H:%M (%A)")
            .to_string();
        let mut parts = vec![format!(
            "# ferrum-bot\n\nYou are ferrum-bot, a helpful Rust AI assistant.\n\nCurrent Time: {now}\nWorkspace: {}",
            self.workspace.display()
        )];

        for file in ["AGENTS.md", "SOUL.md", "USER.md", "TOOLS.md", "IDENTITY.md"] {
            let path = self.workspace.join(file);
            if path.exists()
                && let Ok(content) = std::fs::read_to_string(&path)
            {
                parts.push(format!("## {file}\n\n{content}"));
            }
        }

        let memory = self.workspace.join("memory").join("MEMORY.md");
        if memory.exists()
            && let Ok(content) = std::fs::read_to_string(&memory)
        {
            parts.push(format!("## Memory\n\n{content}"));
        }

        let skills = self.workspace.join("skills");
        if skills.exists() {
            let mut summary = Vec::new();
            if let Ok(entries) = std::fs::read_dir(&skills) {
                for entry in entries.flatten() {
                    let skill = entry.path().join("SKILL.md");
                    if skill.exists() {
                        summary.push(format!(
                            "- {} ({})",
                            entry.file_name().to_string_lossy(),
                            skill.display()
                        ));
                    }
                }
            }
            if !summary.is_empty() {
                parts.push(format!(
                    "## Skills\n\nUse read_file to load full skill docs when needed.\n{}",
                    summary.join("\n")
                ));
            }
        }

        if let (Some(channel), Some(chat_id)) = (channel, chat_id) {
            parts.push(format!(
                "## Current Session\nChannel: {channel}\nChat ID: {chat_id}"
            ));
        }

        parts.join("\n\n---\n\n")
    }

    pub fn build_messages(
        &self,
        history: Vec<Value>,
        current_message: &str,
        channel: Option<&str>,
        chat_id: Option<&str>,
    ) -> Vec<Value> {
        let mut messages = vec![json!({
            "role": "system",
            "content": self.build_system_prompt(channel, chat_id),
        })];
        messages.extend(history);
        messages.push(json!({"role": "user", "content": current_message}));
        messages
    }

    pub fn add_assistant(
        &self,
        messages: &mut Vec<Value>,
        content: Option<String>,
        tool_calls: Option<Vec<Value>>,
    ) {
        let mut msg = json!({"role": "assistant", "content": content.unwrap_or_default()});
        if let Some(tool_calls) = tool_calls {
            msg["tool_calls"] = Value::Array(tool_calls);
        }
        messages.push(msg);
    }

    pub fn add_tool_result(
        &self,
        messages: &mut Vec<Value>,
        tool_call_id: &str,
        name: &str,
        result: &str,
    ) {
        messages.push(json!({
            "role": "tool",
            "tool_call_id": tool_call_id,
            "name": name,
            "content": result,
        }));
    }
}
