mod app;
mod commands;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    app::run().await
}
