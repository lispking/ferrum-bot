use std::sync::Arc;

use anyhow::Result;
use ferrumbot_agent::AgentLoop;
use ferrumbot_channels::ChannelManager;
use ferrumbot_config::Config;
use ferrumbot_cron::CronService;
use tracing::info;

use crate::init_tracing;

pub(super) struct GatewayRuntime {
    pub agent: Arc<AgentLoop>,
    pub channels: Arc<ChannelManager>,
    pub cron: CronService,
    pub agent_task: Option<tokio::task::JoinHandle<()>>,
}

mod setup;
mod teardown;

pub async fn run_gateway(config: Config, port: Option<u16>, verbose: bool) -> Result<()> {
    init_tracing(verbose);

    let actual_port = port.unwrap_or(config.gateway.port);
    let mut runtime = setup::build_runtime(config).await?;

    info!("starting ferrum-bot gateway on port {actual_port}");
    if runtime.channels.enabled_channels().is_empty() {
        info!("no channels enabled");
    } else {
        info!(
            "channels enabled: {}",
            runtime.channels.enabled_channels().join(", ")
        );
    }

    setup::start_runtime(&mut runtime).await?;

    tokio::signal::ctrl_c().await?;
    info!("received interrupt, shutting down");

    teardown::stop_runtime(runtime).await
}
