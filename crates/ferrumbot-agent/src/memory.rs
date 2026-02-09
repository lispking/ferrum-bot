use std::path::Path;

use anyhow::Result;

pub fn ensure_memory_files(workspace: &Path) -> Result<()> {
    let memory_dir = workspace.join("memory");
    std::fs::create_dir_all(&memory_dir)?;

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let today_file = memory_dir.join(format!("{today}.md"));
    if !today_file.exists() {
        std::fs::write(today_file, format!("# {today}\n\n"))?;
    }

    let memory = memory_dir.join("MEMORY.md");
    if !memory.exists() {
        std::fs::write(memory, "# Long-term Memory\n\n")?;
    }
    Ok(())
}
