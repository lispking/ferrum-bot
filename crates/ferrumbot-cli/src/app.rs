use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use ferrumbot_config::VERSION;

use crate::commands;

#[derive(Parser, Debug)]
#[command(name = "ferrum-bot", version = VERSION, about = "ferrum-bot - Rust Personal AI Assistant")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Onboard,
    Status,
    Agent(AgentArgs),
    Gateway(GatewayArgs),
    Channels(ChannelsCommand),
    Cron(CronCommand),
}

#[derive(Args, Debug)]
pub struct AgentArgs {
    #[arg(long, short = 'm')]
    pub message: Option<String>,
    #[arg(long, short = 's', default_value = "cli:default")]
    pub session: String,
}

#[derive(Args, Debug)]
pub struct GatewayArgs {
    #[arg(long, short = 'p')]
    pub port: Option<u16>,
    #[arg(long, short = 'v', default_value_t = false)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum ChannelsAction {
    Status,
}

#[derive(Args, Debug)]
pub struct ChannelsCommand {
    #[command(subcommand)]
    pub action: ChannelsAction,
}

#[derive(Subcommand, Debug)]
pub enum CronAction {
    List {
        #[arg(long, short = 'a', default_value_t = false)]
        all: bool,
    },
    Add {
        #[arg(long, short = 'n')]
        name: String,
        #[arg(long, short = 'm')]
        message: String,
        #[arg(long, short = 'e')]
        every: Option<i64>,
        #[arg(long, short = 'c')]
        cron: Option<String>,
        #[arg(long)]
        at: Option<String>,
        #[arg(long, short = 'd', default_value_t = false)]
        deliver: bool,
        #[arg(long)]
        to: Option<String>,
        #[arg(long)]
        channel: Option<String>,
    },
    Remove {
        job_id: String,
    },
    Enable {
        job_id: String,
        #[arg(long, default_value_t = false)]
        disable: bool,
    },
    Run {
        job_id: String,
        #[arg(long, short = 'f', default_value_t = false)]
        force: bool,
    },
}

#[derive(Args, Debug)]
pub struct CronCommand {
    #[command(subcommand)]
    pub action: CronAction,
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Onboard => commands::onboard::run().await?,
        Commands::Status => commands::status::run().await?,
        Commands::Agent(args) => commands::agent::run(args).await?,
        Commands::Gateway(args) => commands::gateway::run(args).await?,
        Commands::Channels(cmd) => commands::channels::run(cmd).await?,
        Commands::Cron(cmd) => commands::cron::run(cmd).await?,
    }

    Ok(())
}
