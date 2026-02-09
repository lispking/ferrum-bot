use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

pub fn ensure_workspace_templates(workspace: &Path) -> Result<()> {
    fs::create_dir_all(workspace)
        .with_context(|| format!("failed to create workspace: {}", workspace.display()))?;

    write_if_missing(
        &workspace.join("AGENTS.md"),
        "# Agent Instructions\n\nYou are a helpful AI assistant. Be concise, accurate, and friendly.\n",
    )?;
    write_if_missing(
        &workspace.join("SOUL.md"),
        "# Soul\n\nI am ferrum-bot, a lightweight Rust AI assistant.\n",
    )?;
    write_if_missing(
        &workspace.join("USER.md"),
        "# User\n\nCommunication style: concise\n",
    )?;
    write_if_missing(
        &workspace.join("TOOLS.md"),
        "# Tools\n\nUse tools carefully and explain actions before running them.\n",
    )?;

    let memory_dir = workspace.join("memory");
    fs::create_dir_all(&memory_dir)?;
    write_if_missing(
        &memory_dir.join("MEMORY.md"),
        "# Long-term Memory\n\n(Important facts and preferences)\n",
    )?;

    Ok(())
}

fn write_if_missing(path: &Path, content: &str) -> Result<()> {
    if !path.exists() {
        fs::write(path, content)?;
    }
    Ok(())
}
