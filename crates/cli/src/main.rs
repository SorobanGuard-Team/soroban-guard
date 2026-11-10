mod cli;
mod commands;

use anyhow::Result;
use cli::Cli;
use clap::Parser;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let level = if cli.verbose { "debug" } else { "info" };
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .init();

    let exit_code = commands::dispatch(cli).await?;
    std::process::exit(exit_code);
}
