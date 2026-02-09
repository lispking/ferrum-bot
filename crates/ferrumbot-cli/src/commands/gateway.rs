use anyhow::Result;
use ferrumbot_config::load_config;
use ferrumbot_runtime::run_gateway;

use crate::app::GatewayArgs;

pub async fn run(args: GatewayArgs) -> Result<()> {
    let config = load_config(None)?;
    run_gateway(config, args.port, args.verbose).await
}
