use anyhow::Result;
use ferrumbot_core::{InboundMessage, OutboundMessage};
use ferrumbot_tools::ToolContext;
use serde_json::{Value, json};

use super::AgentLoop;

impl AgentLoop {
    pub(super) async fn process_message_with_session(
        &self,
        msg: InboundMessage,
        session_key: &str,
    ) -> Result<Option<OutboundMessage>> {
        let history = {
            let mut sessions = self.sessions.lock().await;
            let session = sessions.get_or_create(session_key)?;
            session.get_history(50)
        };

        let mut messages = self.context.build_messages(
            history,
            &msg.content,
            Some(&msg.channel),
            Some(&msg.chat_id),
        );

        let tool_defs = self.tools.definitions();
        let mut final_content = None;

        for _ in 0..self.max_iterations {
            let resp = self
                .provider
                .chat(
                    messages.clone(),
                    Some(tool_defs.clone()),
                    Some(&self.model),
                    None,
                    None,
                )
                .await?;

            if resp.has_tool_calls() {
                let tool_calls: Vec<Value> = resp
                    .tool_calls
                    .iter()
                    .map(|tc| {
                        json!({
                            "id": tc.id,
                            "type": "function",
                            "function": {
                                "name": tc.name,
                                "arguments": tc.arguments.to_string(),
                            }
                        })
                    })
                    .collect();

                self.context
                    .add_assistant(&mut messages, resp.content.clone(), Some(tool_calls));

                for call in resp.tool_calls {
                    let result = self
                        .tools
                        .execute(
                            &call.name,
                            call.arguments,
                            ToolContext {
                                workspace: self.workspace.clone(),
                                current_channel: Some(msg.channel.clone()),
                                current_chat_id: Some(msg.chat_id.clone()),
                                bus: Some(self.bus.clone()),
                                cron: self.cron.clone(),
                            },
                        )
                        .await;
                    self.context
                        .add_tool_result(&mut messages, &call.id, &call.name, &result);
                }
            } else {
                final_content = resp.content;
                break;
            }
        }

        let final_content = final_content.unwrap_or_else(|| {
            "I've completed processing but have no response to give.".to_string()
        });

        {
            let mut sessions = self.sessions.lock().await;
            let session = sessions.get_or_create(session_key)?;
            session.add_message("user", &msg.content);
            session.add_message("assistant", &final_content);
            sessions.save(session_key)?;
        }

        Ok(Some(OutboundMessage {
            channel: msg.channel,
            chat_id: msg.chat_id,
            content: final_content,
            reply_to: None,
            media: Vec::new(),
            metadata: Default::default(),
        }))
    }
}
