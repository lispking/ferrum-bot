use std::sync::Arc;

use anyhow::Result;
use ferrumbot_agent::AgentLoop;
use ferrumbot_channels::ChannelManager;
use ferrumbot_config::Config;
use ferrumbot_core::MessageBus;
use ferrumbot_cron::{CronService, boxed_callback};

use super::GatewayRuntime;

pub(super) async fn build_runtime(config: Config) -> Result<GatewayRuntime> {
    let bus = MessageBus::new(256);
    let cron_store_path = ferrumbot_config::data_dir().join("cron").join("jobs.json");
    let cron = CronService::new(cron_store_path).await?;

    let agent = Arc::new(AgentLoop::from_config(
        bus.clone(),
        &config,
        Some(cron.clone()),
    )?);

    let callback_agent = agent.clone();
    cron.set_on_job(boxed_callback(move |job| {
        let agent = callback_agent.clone();
        async move {
            let response = agent
                .process_direct(
                    &job.payload.message,
                    &format!("cron:{}", job.id),
                    job.payload.channel.as_deref().unwrap_or("cli"),
                    job.payload.to.as_deref().unwrap_or("direct"),
                )
                .await?;
            Ok(Some(response))
        }
    }))
    .await;

    let channels = Arc::new(ChannelManager::new(&config, bus));

    Ok(GatewayRuntime {
        agent,
        channels,
        cron,
        agent_task: None,
    })
}

pub(super) async fn start_runtime(runtime: &mut GatewayRuntime) -> Result<()> {
    runtime.cron.start().await?;
    runtime.channels.start_all().await?;

    let agent = runtime.agent.clone();
    runtime.agent_task = Some(tokio::spawn(async move {
        agent.run().await;
    }));

    Ok(())
}
