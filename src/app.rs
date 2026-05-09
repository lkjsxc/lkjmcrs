use crate::config::Config;
use crate::quality;
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

mod probe_cli;

const DEFAULT_LOG_FILTER: &str = "lkjmcrs=info";

#[derive(Parser)]
#[command(name = "lkjmcrs")]
#[command(about = "Rust Minecraft server")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Serve,
    Docs {
        #[command(subcommand)]
        command: DocsCommand,
    },
    Quality {
        #[command(subcommand)]
        command: QualityCommand,
    },
    Probe {
        #[command(subcommand)]
        command: probe_cli::ProbeCommand,
    },
    Fixture {
        #[command(subcommand)]
        command: FixtureCommand,
    },
}

#[derive(Subcommand)]
enum DocsCommand {
    ValidateTopology,
}

#[derive(Subcommand)]
enum QualityCommand {
    CheckLines,
}

#[derive(Subcommand)]
enum FixtureCommand {
    SessionServer {
        #[arg(long, default_value = "0.0.0.0:25566")]
        bind: String,
    },
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_FILTER));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    match Cli::parse().command {
        Command::Serve => crate::net::serve(Config::from_default_path()?).await?,
        Command::Docs { command } => match command {
            DocsCommand::ValidateTopology => quality::validate_docs_topology()?,
        },
        Command::Quality { command } => match command {
            QualityCommand::CheckLines => quality::check_lines()?,
        },
        Command::Probe { command } => probe_cli::run(command).await?,
        Command::Fixture { command } => match command {
            FixtureCommand::SessionServer { bind } => crate::session_fixture::serve(&bind).await?,
        },
    }

    Ok(())
}
