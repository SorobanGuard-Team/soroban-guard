use std::time::Instant;

use sorobanguard_core::{check::CheckResult, context::ProjectContext, pipeline::Validator};

pub struct ContractTestsValidator;

impl Validator for ContractTestsValidator {
    fn id(&self) -> &str { "contract_tests" }
    fn name(&self) -> &str { "Contract test suite" }
    fn description(&self) -> &str {
        "Runs the contract test suite with `cargo test`"
    }

    fn run(&self, ctx: &ProjectContext) -> CheckResult {
        let start = Instant::now();

        let output = std::process::Command::new("cargo")
            .args(["test", "--", "--nocapture"])
            .current_dir(&ctx.project.root_dir)
            .envs(&ctx.config.project.env)
            .output();

        let duration = start.elapsed();

        match output {
            Err(e) => CheckResult::failed(
                self.id(), self.name(), self.description(), duration,
                format!("Failed to run cargo test: {e}"),
                Some("Ensure cargo is installed and on PATH".to_string()),
            ),
            Ok(out) => {
                let combined = format!(
                    "{}\n{}",
                    String::from_utf8_lossy(&out.stdout),
                    String::from_utf8_lossy(&out.stderr)
                );
                if out.status.success() {
                    CheckResult::passed(self.id(), self.name(), self.description(), duration)
                        .with_output(combined)
                } else {
                    CheckResult::failed(
                        self.id(), self.name(), self.description(), duration,
                        "One or more tests failed",
                        Some("Run `cargo test` locally to see test output".to_string()),
                    )
                    .with_output(combined)
                }
            }
        }
    }
}
