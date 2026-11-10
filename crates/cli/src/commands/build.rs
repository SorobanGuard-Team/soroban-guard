use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;

use crate::cli::BuildArgs;

pub async fn run(project_dir: PathBuf, args: BuildArgs) -> Result<i32> {
    println!("{}", "SorobanGuard — building contract…".bold());

    let stellar_bin = if which::which("stellar").is_ok() {
        "stellar"
    } else if which::which("soroban").is_ok() {
        "soroban"
    } else {
        eprintln!(
            "{} stellar CLI not found. Install with:\n  cargo install stellar-cli --features opt",
            "✗".red()
        );
        return Ok(1);
    };

    let mut cmd = std::process::Command::new(stellar_bin);
    cmd.args(["contract", "build"])
        .current_dir(&project_dir);

    if !args.extra_args.is_empty() {
        cmd.args(&args.extra_args);
    }

    let status = cmd.status()?;
    if status.success() {
        println!("{} Build succeeded.", "✓".green());
        Ok(0)
    } else {
        eprintln!("{} Build failed.", "✗".red());
        Ok(status.code().unwrap_or(1))
    }
}
