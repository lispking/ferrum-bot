use anyhow::Result;

use super::GatewayRuntime;

pub(super) async fn stop_runtime(mut runtime: GatewayRuntime) -> Result<()> {
    runtime.agent.stop().await;
    runtime.cron.stop().await;
    runtime.channels.stop_all().await?;

    if let Some(task) = runtime.agent_task.take() {
        task.abort();
    }

    Ok(())
}
