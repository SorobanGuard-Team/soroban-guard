pub mod terminal;
pub mod json_reporter;
pub mod markdown_reporter;

pub use terminal::TerminalReporter;
pub use json_reporter::JsonReporter;
pub use markdown_reporter::MarkdownReporter;

use anyhow::Result;
use sorobanguard_core::pipeline::PipelineReport;
use std::path::Path;

pub trait Reporter {
    fn report(&self, report: &PipelineReport) -> Result<()>;
}

pub fn write_reports(
    report: &PipelineReport,
    output_dir: &Path,
    json: bool,
    markdown: bool,
) -> Result<()> {
    std::fs::create_dir_all(output_dir)?;

    if json {
        let path = output_dir.join("sorobanguard-report.json");
        let reporter = JsonReporter::new(path);
        reporter.report(report)?;
    }

    if markdown {
        let path = output_dir.join("sorobanguard-report.md");
        let reporter = MarkdownReporter::new(path);
        reporter.report(report)?;
    }

    Ok(())
}
