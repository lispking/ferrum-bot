use std::path::{Path, PathBuf};

use anyhow::Result;
use regex::Regex;

pub(super) fn guard_command(
    command: &str,
    cwd: &Path,
    workspace_root: &Path,
    restrict_to_workspace: bool,
) -> Result<Option<String>> {
    let lower = command.trim().to_lowercase();
    let deny_patterns = [
        r"\brm\s+-[rf]{1,2}\b",
        r"\bdel\s+/[fq]\b",
        r"\brmdir\s+/s\b",
        r"\b(format|mkfs|diskpart)\b",
        r"\bdd\s+if=",
        r">\s*/dev/sd",
        r"\b(shutdown|reboot|poweroff)\b",
        r":\(\)\s*\{.*\};\s*:",
    ];

    for pattern in deny_patterns {
        if Regex::new(pattern)?.is_match(&lower) {
            return Ok(Some(
                "Error: Command blocked by safety guard (dangerous pattern detected)".to_string(),
            ));
        }
    }

    if restrict_to_workspace && (command.contains("../") || command.contains("..\\")) {
        return Ok(Some(
            "Error: Command blocked by safety guard (path traversal detected)".to_string(),
        ));
    }

    if restrict_to_workspace {
        let workspace = workspace_root
            .canonicalize()
            .unwrap_or_else(|_| workspace_root.to_path_buf());
        let cwd = cwd.canonicalize().unwrap_or_else(|_| {
            if cwd.is_absolute() {
                cwd.to_path_buf()
            } else {
                std::env::current_dir()
                    .unwrap_or_else(|_| PathBuf::from("."))
                    .join(cwd)
            }
        });

        if !cwd.starts_with(workspace) {
            return Ok(Some(
                "Error: Command blocked by safety guard (path outside working dir)".to_string(),
            ));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::guard_command;

    fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock drift")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{}-{now}", std::process::id()))
    }

    #[test]
    fn blocks_cwd_outside_workspace() {
        let root = unique_temp_dir("ferrum-guard-test");
        let workspace = root.join("workspace");
        let outside = root.join("outside");
        fs::create_dir_all(&workspace).expect("create workspace");
        fs::create_dir_all(&outside).expect("create outside dir");

        let out = guard_command("echo ok", outside.as_path(), workspace.as_path(), true)
            .expect("guard should execute");

        assert!(
            out.expect("should block")
                .contains("path outside working dir")
        );
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn allows_cwd_inside_workspace() {
        let root = unique_temp_dir("ferrum-guard-test");
        let workspace = root.join("workspace");
        let inside = workspace.join("sub");
        fs::create_dir_all(&inside).expect("create inside dir");

        let out = guard_command("echo ok", inside.as_path(), workspace.as_path(), true)
            .expect("guard should execute");

        assert!(out.is_none());
        let _ = fs::remove_dir_all(&root);
    }
}
