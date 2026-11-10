use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "sorobanguard",
    version,
    author,
    about = "The open-source testing and quality toolkit for Stellar Soroban smart contracts",
    long_about = None,
    propagate_version = true,
)]
pub struct Cli {
    /// Enable verbose (debug-level) logging.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress terminal color output.
    #[arg(long, global = true, env = "NO_COLOR")]
    pub no_color: bool,

    /// Path to the Soroban project directory (defaults to current directory).
    #[arg(short, long, global = true, value_name = "DIR")]
    pub project: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize SorobanGuard in the current project.
    Init(InitArgs),

    /// Run the full validation pipeline (lint, build, test, inspect).
    Check(CheckArgs),

    /// Build the Soroban contract with `stellar contract build`.
    Build(BuildArgs),

    /// Run the contract test suite with `cargo test`.
    Test(TestArgs),

    /// Inspect the compiled WASM artifact and report metadata.
    Inspect(InspectArgs),

    /// Generate a validation report (JSON + Markdown) from the last run.
    Report(ReportArgs),

    /// Print shell completion script for the given shell.
    Completions(CompletionsArgs),
}

#[derive(Parser, Debug)]
pub struct InitArgs {
    /// Do not copy the starter template into the project.
    #[arg(long)]
    pub no_template: bool,
}

#[derive(Parser, Debug)]
pub struct CheckArgs {
    /// Write JSON and Markdown reports to this directory.
    #[arg(long, value_name = "DIR")]
    pub report_dir: Option<PathBuf>,

    /// Treat WASM size warnings as errors (overrides config).
    #[arg(long)]
    pub strict_size: bool,

    /// Run only the listed check IDs (comma-separated).
    #[arg(long, value_name = "IDS", value_delimiter = ',')]
    pub only: Vec<String>,

    /// Skip the listed check IDs (comma-separated).
    #[arg(long, value_name = "IDS", value_delimiter = ',')]
    pub skip: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct BuildArgs {
    /// Pass extra flags through to `stellar contract build`.
    #[arg(last = true)]
    pub extra_args: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct TestArgs {
    /// Pass extra flags through to `cargo test`.
    #[arg(last = true)]
    pub extra_args: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct InspectArgs {
    /// Path to the WASM file to inspect (defaults to auto-detected artifact).
    #[arg(value_name = "WASM")]
    pub wasm_path: Option<PathBuf>,

    /// Print full export and import lists.
    #[arg(long)]
    pub full: bool,
}

#[derive(Parser, Debug)]
pub struct ReportArgs {
    /// Directory where reports are written.
    #[arg(long, value_name = "DIR")]
    pub output_dir: Option<PathBuf>,

    /// Only generate JSON report.
    #[arg(long, conflicts_with = "markdown_only")]
    pub json_only: bool,

    /// Only generate Markdown report.
    #[arg(long)]
    pub markdown_only: bool,
}

#[derive(Parser, Debug)]
pub struct CompletionsArgs {
    /// Shell to generate completions for.
    #[arg(value_name = "SHELL")]
    pub shell: clap_complete::Shell,
}
