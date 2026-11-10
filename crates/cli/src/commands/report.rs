use std::path::PathBuf;

use anyhow::{bail, Result};
use colored::Colorize;
use sorobanguard_reporters::write_reports;

use crate::cli::ReportArgs;

pub async fn run(project_dir: PathBuf, args: ReportArgs) -> Result<i32> {
    println!("{}", "SorobanGuard — generating reports…".bold());

    let report_dir = args
        .output_dir
        .unwrap_or_else(|| project_dir.join("sorobanguard-reports"));

    // Load latest JSON report to re-serialize into Markdown (or vice-versa)
    let json_path = report_dir.join("sorobanguard-report.json");
    if !json_path.exists() {
        bail!(
            "No existing report found at {}. Run `sorobanguard check` first.",
            json_path.display()
        );
    }

    let raw = std::fs::read_to_string(&json_path)?;
    let report: sorobanguard_core::pipeline::PipelineReport = serde_json::from_str(&raw)?;

    let generate_json = !args.markdown_only;
    let generate_markdown = !args.json_only;

    write_reports(&report, &report_dir, generate_json, generate_markdown)?;

    println!("{} Reports written to: {}", "✓".green(), report_dir.display());
    Ok(0)
}
