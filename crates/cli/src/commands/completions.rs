use anyhow::Result;
use clap::CommandFactory;
use clap_complete::generate;

use crate::cli::{Cli, CompletionsArgs};

pub fn run(args: CompletionsArgs) -> Result<i32> {
    generate(args.shell, &mut Cli::command(), "sorobanguard", &mut std::io::stdout());
    Ok(0)
}
