use std::time::Instant;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    check::{CheckResult, CheckStatus},
    context::ProjectContext,
    error::CoreError,
    project::ProjectInfo,
    toolchain::ToolchainInfo,
    wasm::WasmArtifact,
};

/// Trait every validator must implement.
pub trait Validator: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;

    fn run(&self, ctx: &ProjectContext) -> CheckResult;
}

/// A configured sequence of validators that produces a PipelineReport.
pub struct Pipeline {
    validators: Vec<Box<dyn Validator>>,
}

impl Pipeline {
    pub fn new(validators: Vec<Box<dyn Validator>>) -> Self {
        Self { validators }
    }

    pub fn run(&self, ctx: &ProjectContext) -> PipelineReport {
        let started_at = Utc::now();
        let timer = Instant::now();

        let skip_ids: std::collections::HashSet<&str> = ctx
            .config
            .checks
            .skip
            .iter()
            .map(String::as_str)
            .collect();

        let results: Vec<CheckResult> = self
            .validators
            .iter()
            .map(|v| {
                if skip_ids.contains(v.id()) {
                    CheckResult::skipped(
                        v.id(),
                        v.name(),
                        v.description(),
                        "Skipped by configuration",
                    )
                } else {
                    v.run(ctx)
                }
            })
            .collect();

        let total_duration = timer.elapsed();

        PipelineReport {
            started_at,
            total_duration_secs: total_duration.as_secs_f64(),
            project: ctx.project.clone(),
            toolchain: ctx.toolchain.clone(),
            results,
            wasm_artifact: None,
        }
    }
}

/// The complete output of a validation run.
#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineReport {
    pub started_at: DateTime<Utc>,
    pub total_duration_secs: f64,
    pub project: ProjectInfo,
    pub toolchain: ToolchainInfo,
    pub results: Vec<CheckResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wasm_artifact: Option<WasmArtifact>,
}

impl PipelineReport {
    pub fn passed(&self) -> Vec<&CheckResult> {
        self.results.iter().filter(|r| r.status == CheckStatus::Passed).collect()
    }

    pub fn failed(&self) -> Vec<&CheckResult> {
        self.results.iter().filter(|r| r.status == CheckStatus::Failed).collect()
    }

    pub fn skipped(&self) -> Vec<&CheckResult> {
        self.results.iter().filter(|r| r.status == CheckStatus::Skipped).collect()
    }

    pub fn warnings(&self) -> Vec<&CheckResult> {
        self.results.iter().filter(|r| r.status == CheckStatus::Warning).collect()
    }

    pub fn overall_passed(&self) -> bool {
        self.failed().is_empty()
    }

    pub fn exit_code(&self) -> i32 {
        if self.overall_passed() { 0 } else { 1 }
    }

    pub fn summary_line(&self) -> String {
        format!(
            "{} passed, {} failed, {} skipped, {} warnings — {:.2}s",
            self.passed().len(),
            self.failed().len(),
            self.skipped().len(),
            self.warnings().len(),
            self.total_duration_secs,
        )
    }
}
