use std::path::PathBuf;

use ferrumbot_core::MessageBus;
use ferrumbot_cron::CronService;

#[derive(Clone)]
pub struct ToolContext {
    pub workspace: PathBuf,
    pub current_channel: Option<String>,
    pub current_chat_id: Option<String>,
    pub bus: Option<MessageBus>,
    pub cron: Option<CronService>,
}
