use std::path::PathBuf;

use anyhow::Result;
use sorobanguard_core::pipeline::PipelineReport;

use crate::Reporter;

pub struct JsonReporter {
    path: PathBuf,
}

impl JsonReporter {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Reporter for JsonReporter {
    fn report(&self, report: &PipelineReport) -> Result<()> {
        let json = serde_json::to_string_pretty(report)?;
        std::fs::write(&self.path, json)?;
        println!("  JSON report written to: {}", self.path.display());
        Ok(())
    }
}
