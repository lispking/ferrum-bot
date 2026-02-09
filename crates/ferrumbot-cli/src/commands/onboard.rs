use std::io::{self, Write};

use anyhow::Result;
use ferrumbot_config::{
    APP_NAME, Config, config_path, ensure_workspace_templates, load_config, save_config,
};

pub async fn run() -> Result<()> {
    let path = config_path();
    let mut config = load_config(None).unwrap_or_default();

    if path.exists() {
        println!("Config already exists at {}", path.display());
        print!("Overwrite? [y/N]: ");
        io::stdout().flush()?;
        let mut ans = String::new();
        io::stdin().read_line(&mut ans)?;
        if !matches!(ans.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("Aborted.");
            return Ok(());
        }
        config = Config::default();
    }

    save_config(&config, None)?;
    ensure_workspace_templates(&config.workspace_path())?;
    ferrumbot_agent::ensure_memory_files(&config.workspace_path())?;

    println!("✓ Created config at {}", path.display());
    println!(
        "✓ Created workspace at {}",
        config.workspace_path().display()
    );
    println!("\n{APP_NAME} is ready!");
    println!("Next steps:");
    println!("  1. Add your API key to ~/.ferrum-bot/config.json");
    println!("  2. Chat: ferrum-bot agent -m \"Hello!\"");

    Ok(())
}
