use colored::Colorize;
use comfy_table::{Cell, Color, Table};
use sorobanguard_core::{check::CheckStatus, pipeline::PipelineReport};
use anyhow::Result;

use crate::Reporter;

pub struct TerminalReporter {
    pub no_color: bool,
}

impl TerminalReporter {
    pub fn new(no_color: bool) -> Self {
        if no_color {
            colored::control::set_override(false);
        }
        Self { no_color }
    }

    fn status_colored(&self, status: &CheckStatus) -> String {
        match status {
            CheckStatus::Passed  => "✓ PASSED".green().to_string(),
            CheckStatus::Failed  => "✗ FAILED".red().bold().to_string(),
            CheckStatus::Skipped => "- SKIPPED".dimmed().to_string(),
            CheckStatus::Warning => "⚠ WARNING".yellow().to_string(),
        }
    }
}

impl Reporter for TerminalReporter {
    fn report(&self, report: &PipelineReport) -> Result<()> {
        println!();
        println!("{}", "━━━ SorobanGuard Validation Report ━━━".bold());
        println!("  Project : {}", report.project.name);
        println!("  Version : {}", report.project.version);
        println!("  Run at  : {}", report.started_at.format("%Y-%m-%d %H:%M:%S UTC"));
        println!();

        // Build table
        let mut table = Table::new();
        table.set_header(vec!["Check", "Status", "Duration", "Detail"]);

        for result in &report.results {
            let status_cell = Cell::new(self.status_colored(&result.status));
            let detail = result
                .error
                .as_deref()
                .or(result.output.as_deref())
                .unwrap_or("")
                .lines()
                .next()
                .unwrap_or("")
                .chars()
                .take(80)
                .collect::<String>();

            table.add_row(vec![
                Cell::new(&result.name),
                status_cell,
                Cell::new(format!("{:.2}s", result.duration.as_secs_f64())),
                Cell::new(detail),
            ]);
        }

        println!("{table}");
        println!();

        // Print fix hints for failures
        let failures = report.failed();
        if !failures.is_empty() {
            println!("{}", "Required actions:".red().bold());
            for f in &failures {
                println!("  • {} — {}", f.name.bold(), f.error.as_deref().unwrap_or("see above"));
                if let Some(hint) = &f.fix_hint {
                    println!("    → {}", hint.dimmed());
                }
            }
            println!();
        }

        // Summary line
        let summary = report.summary_line();
        if report.overall_passed() {
            println!("{}", format!("✓ All checks passed. {summary}").green().bold());
        } else {
            println!("{}", format!("✗ Validation failed. {summary}").red().bold());
        }
        println!();

        Ok(())
    }
}
