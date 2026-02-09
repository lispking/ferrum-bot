use std::collections::HashMap;

use anyhow::Result;
use ferrumbot_core::OutboundMessage;

use crate::ToolContext;

use super::args::MessageRequest;

pub(super) async fn send_message(request: MessageRequest, ctx: ToolContext) -> Result<String> {
    let Some(bus) = ctx.bus else {
        return Ok("Error: message bus is not configured".to_string());
    };

    bus.publish_outbound(OutboundMessage {
        channel: request.channel.clone(),
        chat_id: request.chat_id.clone(),
        content: request.content,
        reply_to: None,
        media: Vec::new(),
        metadata: HashMap::new(),
    })
    .await?;

    Ok(format!(
        "message sent to {}:{}",
        request.channel, request.chat_id
    ))
}
