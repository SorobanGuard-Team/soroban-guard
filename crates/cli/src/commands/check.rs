use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;
use sorobanguard_core::{context::ProjectContext, wasm::WasmArtifact};
use sorobanguard_reporters::{Reporter, TerminalReporter, write_reports};
use sorobanguard_validators::default_pipeline;

use crate::cli::CheckArgs;

pub async fn run(project_dir: PathBuf, args: CheckArgs, no_color: bool) -> Result<i32> {
    println!("{}", "SorobanGuard — running validation pipeline…".bold());
    println!("  Project: {}", project_dir.display());
    println!();

    let mut ctx = ProjectContext::load(project_dir)?;

    // Apply CLI overrides to config
    for id in &args.skip {
        ctx.config.checks.skip.push(id.clone());
    }
    if args.strict_size {
        ctx.config.checks.fail_on_wasm_size = true;
    }

    // Build pipeline — filter to `--only` list if provided
    let pipeline = if args.only.is_empty() {
        default_pipeline()
    } else {
        // Build default then whittle down based on IDs
        // For now run default; `only` filtering would require exposing validators
        default_pipeline()
    };

    let mut report = pipeline.run(&ctx);

    // Attach WASM artifact metadata if the artifact exists
    let wasm_path = ctx
        .config
        .wasm
        .artifact_path
        .clone()
        .unwrap_or_else(|| ctx.project.expected_wasm_path());

    if wasm_path.exists() {
        if let Ok(artifact) = WasmArtifact::inspect(&wasm_path) {
            report.wasm_artifact = Some(artifact);
        }
    }

    // Terminal output
    let terminal = TerminalReporter::new(no_color);
    terminal.report(&report)?;

    // File reports
    let report_dir = args
        .report_dir
        .or_else(|| ctx.config.report.output_dir.clone())
        .unwrap_or_else(|| ctx.project_dir.join("sorobanguard-reports"));

    write_reports(
        &report,
        &report_dir,
        ctx.config.report.json,
        ctx.config.report.markdown,
    )?;

    Ok(report.exit_code())
}
