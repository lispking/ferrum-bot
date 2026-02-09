mod register;

use std::path::PathBuf;

use ferrumbot_core::MessageBus;
use ferrumbot_cron::CronService;

use crate::registry::ToolRegistry;

pub fn default_registry(
    workspace: PathBuf,
    bus: Option<MessageBus>,
    cron: Option<CronService>,
    brave_api_key: Option<String>,
    web_max_results: usize,
    exec_timeout: u64,
    restrict_to_workspace: bool,
) -> ToolRegistry {
    let mut reg = ToolRegistry::default();
    let allowed = restrict_to_workspace.then(|| workspace.clone());

    register::register_file(&mut reg, allowed);
    register::register_runtime(&mut reg, workspace, exec_timeout, restrict_to_workspace);
    register::register_web(&mut reg, brave_api_key, web_max_results);
    register::register_message_and_spawn(&mut reg);
    register::register_cron(&mut reg);
    register::prune_unavailable(&mut reg, bus.is_some(), cron.is_some());

    reg
}

#[cfg(test)]
mod tests;
