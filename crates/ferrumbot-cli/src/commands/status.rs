use anyhow::Result;
use ferrumbot_config::{config_path, load_config};

pub async fn run() -> Result<()> {
    let config = load_config(None)?;
    let path = config_path();
    let workspace = config.workspace_path();

    println!("ferrum-bot Status\n");
    println!(
        "Config: {} {}",
        path.display(),
        if path.exists() { "✓" } else { "✗" }
    );
    println!(
        "Workspace: {} {}",
        workspace.display(),
        if workspace.exists() { "✓" } else { "✗" }
    );

    if path.exists() {
        println!("Model: {}", config.agents.defaults.model);
        println!(
            "OpenRouter API: {}",
            if config.providers.openrouter.api_key.is_empty() {
                "not set"
            } else {
                "✓"
            }
        );
        println!(
            "Anthropic API: {}",
            if config.providers.anthropic.api_key.is_empty() {
                "not set"
            } else {
                "✓"
            }
        );
        println!(
            "OpenAI API: {}",
            if config.providers.openai.api_key.is_empty() {
                "not set"
            } else {
                "✓"
            }
        );
        println!(
            "Gemini API: {}",
            if config.providers.gemini.api_key.is_empty() {
                "not set"
            } else {
                "✓"
            }
        );
        println!(
            "Zhipu AI API: {}",
            if config.providers.zhipu.api_key.is_empty() {
                "not set"
            } else {
                "✓"
            }
        );
        println!(
            "AiHubMix API: {}",
            if config.providers.aihubmix.api_key.is_empty() {
                "not set"
            } else {
                "✓"
            }
        );
        println!(
            "vLLM/Local: {}",
            config
                .providers
                .vllm
                .api_base
                .clone()
                .unwrap_or_else(|| "not set".to_string())
        );
    }

    Ok(())
}
