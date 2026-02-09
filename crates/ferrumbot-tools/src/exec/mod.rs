mod args;
mod guard;
mod run;

use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::{Tool, ToolContext};

pub struct ExecTool {
    timeout_s: u64,
    working_dir: PathBuf,
    restrict_to_workspace: bool,
}

impl ExecTool {
    pub fn new(timeout_s: u64, working_dir: PathBuf, restrict_to_workspace: bool) -> Self {
        Self {
            timeout_s,
            working_dir,
            restrict_to_workspace,
        }
    }
}

#[async_trait]
impl Tool for ExecTool {
    fn name(&self) -> &'static str {
        "exec"
    }

    fn description(&self) -> &'static str {
        "Execute a shell command and return its output."
    }

    fn parameters(&self) -> Value {
        args::parameters()
    }

    async fn execute(&self, args: Value, _ctx: ToolContext) -> Result<String> {
        let request = args::parse(&args, &self.working_dir);
        if let Some(err) = guard::guard_command(
            &request.command,
            &request.cwd,
            &self.working_dir,
            self.restrict_to_workspace,
        )? {
            return Ok(err);
        }
        run::run_shell_command(&request.command, &request.cwd, self.timeout_s).await
    }
}
