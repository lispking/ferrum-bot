use anyhow::Result;
use ferrumbot_channels::ChannelManager;
use ferrumbot_config::load_config;
use ferrumbot_core::MessageBus;

use crate::app::{ChannelsAction, ChannelsCommand};

pub async fn run(cmd: ChannelsCommand) -> Result<()> {
    let config = load_config(None)?;
    let bus = MessageBus::new(64);
    let manager = ChannelManager::new(&config, bus);

    match cmd.action {
        ChannelsAction::Status => {
            let statuses = manager.status().await;
            if statuses.is_empty() {
                println!("No channels enabled.");
            } else {
                println!("Channel Status");
                for (name, running) in statuses {
                    println!("- {name}: {}", if running { "running" } else { "stopped" });
                }
            }
        }
    }

    Ok(())
}
