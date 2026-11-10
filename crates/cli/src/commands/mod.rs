mod init;
mod check;
mod build;
mod test;
mod inspect;
mod report;
mod completions;

use anyhow::Result;
use crate::cli::{Cli, Commands};

pub async fn dispatch(cli: Cli) -> Result<i32> {
    let project_dir = cli
        .project
        .clone()
        .unwrap_or_else(|| std::env::current_dir().expect("cannot determine current directory"));

    match cli.command {
        Commands::Init(args)        => init::run(project_dir, args).await,
        Commands::Check(args)       => check::run(project_dir, args, cli.no_color).await,
        Commands::Build(args)       => build::run(project_dir, args).await,
        Commands::Test(args)        => test::run(project_dir, args).await,
        Commands::Inspect(args)     => inspect::run(project_dir, args).await,
        Commands::Report(args)      => report::run(project_dir, args).await,
        Commands::Completions(args) => completions::run(args),
    }
}
