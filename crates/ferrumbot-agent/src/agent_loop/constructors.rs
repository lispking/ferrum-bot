use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use ferrumbot_core::{MessageBus, SessionManager};
use ferrumbot_cron::CronService;
use ferrumbot_providers::{LlmProvider, OpenAiCompatibleProvider};
use ferrumbot_tools::default_registry;
use tokio::sync::Mutex;

use super::AgentLoop;
use crate::context::ContextBuilder;

pub struct ToolingConfig {
    pub brave_api_key: Option<String>,
    pub web_max_results: usize,
    pub exec_timeout: u64,
    pub restrict_to_workspace: bool,
}

impl AgentLoop {
    pub fn new(
        bus: MessageBus,
        provider: Arc<dyn LlmProvider>,
        workspace: PathBuf,
        model: String,
        max_iterations: usize,
        cron: Option<CronService>,
        tooling: ToolingConfig,
    ) -> Result<Self> {
        let context = ContextBuilder::new(workspace.clone());
        let sessions = SessionManager::new()?;
        let tools = default_registry(
            workspace.clone(),
            Some(bus.clone()),
            cron.clone(),
            tooling.brave_api_key,
            tooling.web_max_results,
            tooling.exec_timeout,
            tooling.restrict_to_workspace,
        );

        Ok(Self {
            bus,
            provider,
            workspace,
            model,
            max_iterations,
            context,
            sessions: Mutex::new(sessions),
            tools,
            cron,
            running: Mutex::new(false),
        })
    }

    pub fn from_config(
        bus: MessageBus,
        config: &ferrumbot_config::Config,
        cron: Option<CronService>,
    ) -> Result<Self> {
        let provider = Arc::new(OpenAiCompatibleProvider::from_config(config)?);
        Self::new(
            bus,
            provider,
            config.workspace_path(),
            config.agents.defaults.model.clone(),
            config.agents.defaults.max_tool_iterations,
            cron,
            ToolingConfig {
                brave_api_key: Some(config.tools.web.search.api_key.clone())
                    .filter(|x| !x.is_empty()),
                web_max_results: config.tools.web.search.max_results as usize,
                exec_timeout: config.tools.exec.timeout,
                restrict_to_workspace: config.tools.restrict_to_workspace,
            },
        )
    }
}
