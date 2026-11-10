use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;

use crate::cli::TestArgs;

pub async fn run(project_dir: PathBuf, args: TestArgs) -> Result<i32> {
    println!("{}", "SorobanGuard — running tests…".bold());

    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("test").current_dir(&project_dir);

    if !args.extra_args.is_empty() {
        cmd.arg("--").args(&args.extra_args);
    }

    let status = cmd.status()?;
    if status.success() {
        println!("{} All tests passed.", "✓".green());
        Ok(0)
    } else {
        eprintln!("{} Tests failed.", "✗".red());
        Ok(status.code().unwrap_or(1))
    }
}
