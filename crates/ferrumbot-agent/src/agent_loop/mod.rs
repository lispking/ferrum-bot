use std::path::PathBuf;
use std::sync::Arc;

use ferrumbot_core::{MessageBus, SessionManager};
use ferrumbot_cron::CronService;
use ferrumbot_providers::LlmProvider;
use ferrumbot_tools::ToolRegistry;
use tokio::sync::Mutex;

use crate::context::ContextBuilder;

pub struct AgentLoop {
    pub(super) bus: MessageBus,
    pub(super) provider: Arc<dyn LlmProvider>,
    pub(super) workspace: PathBuf,
    pub(super) model: String,
    pub(super) max_iterations: usize,
    pub(super) context: ContextBuilder,
    pub(super) sessions: Mutex<SessionManager>,
    pub(super) tools: ToolRegistry,
    pub(super) cron: Option<CronService>,
    pub(super) running: Mutex<bool>,
}

mod constructors;
mod lifecycle;
mod processing;
