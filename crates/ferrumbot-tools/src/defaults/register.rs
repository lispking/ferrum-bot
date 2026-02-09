use std::path::PathBuf;

use crate::cron::CronTool;
use crate::exec::ExecTool;
use crate::file::{EditFileTool, ListDirTool, ReadFileTool, WriteFileTool};
use crate::message::MessageTool;
use crate::registry::ToolRegistry;
use crate::spawn::SpawnTool;
use crate::web::{WebFetchTool, WebSearchTool};

pub(super) fn register_file(reg: &mut ToolRegistry, allowed: Option<PathBuf>) {
    reg.register(ReadFileTool::new(allowed.clone()));
    reg.register(WriteFileTool::new(allowed.clone()));
    reg.register(EditFileTool::new(allowed.clone()));
    reg.register(ListDirTool::new(allowed));
}

pub(super) fn register_runtime(
    reg: &mut ToolRegistry,
    workspace: PathBuf,
    exec_timeout: u64,
    restrict_to_workspace: bool,
) {
    reg.register(ExecTool::new(
        exec_timeout,
        workspace,
        restrict_to_workspace,
    ));
}

pub(super) fn register_web(
    reg: &mut ToolRegistry,
    brave_api_key: Option<String>,
    max_results: usize,
) {
    reg.register(WebSearchTool::new(brave_api_key, max_results));
    reg.register(WebFetchTool::new(50_000));
}

pub(super) fn register_message_and_spawn(reg: &mut ToolRegistry) {
    reg.register(MessageTool);
    reg.register(SpawnTool);
}

pub(super) fn register_cron(reg: &mut ToolRegistry) {
    reg.register(CronTool);
}

pub(super) fn prune_unavailable(reg: &mut ToolRegistry, has_bus: bool, has_cron: bool) {
    if !has_bus {
        reg.unregister("message");
    }
    if !has_cron {
        reg.unregister("cron");
    }
}
