use std::path::Path;

use anyhow::{Context, Result};
use tokio::process::Command;
use tokio::time::{Duration, timeout};

pub(super) async fn run_shell_command(command: &str, cwd: &Path, timeout_s: u64) -> Result<String> {
    let mut cmd = Command::new("/bin/sh");
    cmd.arg("-lc").arg(command).current_dir(cwd);
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    cmd.kill_on_drop(true);

    let child = cmd.spawn().context("failed to spawn command")?;
    let output = match timeout(Duration::from_secs(timeout_s), child.wait_with_output()).await {
        Ok(output) => output?,
        Err(_) => {
            return Ok(format!(
                "Error: Command timed out after {timeout_s} seconds"
            ));
        }
    };

    let mut text = String::new();
    if !output.stdout.is_empty() {
        text.push_str(&String::from_utf8_lossy(&output.stdout));
    }
    if !output.stderr.is_empty() {
        if !text.is_empty() {
            text.push('\n');
        }
        text.push_str("STDERR:\n");
        text.push_str(&String::from_utf8_lossy(&output.stderr));
    }
    if !output.status.success() {
        text.push_str(&format!(
            "\nExit code: {}",
            output.status.code().unwrap_or(-1)
        ));
    }

    if text.is_empty() {
        text = "(no output)".to_string();
    }
    if text.len() > 10_000 {
        text.truncate(10_000);
        text.push_str("\n... (truncated)");
    }

    Ok(text)
}
