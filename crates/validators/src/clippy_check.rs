use std::time::Instant;

use sorobanguard_core::{check::CheckResult, context::ProjectContext, pipeline::Validator};

pub struct ClippyCheckValidator;

impl Validator for ClippyCheckValidator {
    fn id(&self) -> &str { "clippy" }
    fn name(&self) -> &str { "Clippy lints" }
    fn description(&self) -> &str {
        "Runs `cargo clippy -- -D warnings` to catch common Rust mistakes"
    }

    fn run(&self, ctx: &ProjectContext) -> CheckResult {
        let start = Instant::now();

        if !ctx.config.checks.run_clippy {
            return CheckResult::skipped(
                self.id(), self.name(), self.description(),
                "Disabled in sorobanguard.toml (checks.run_clippy = false)",
            );
        }

        let output = std::process::Command::new("cargo")
            .args(["clippy", "--", "-D", "warnings"])
            .current_dir(&ctx.project.root_dir)
            .envs(&ctx.config.project.env)
            .output();

        let duration = start.elapsed();

        match output {
            Err(e) => CheckResult::failed(
                self.id(), self.name(), self.description(), duration,
                format!("Failed to run cargo clippy: {e}"),
                Some("Install clippy: rustup component add clippy".to_string()),
            ),
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                if out.status.success() {
                    CheckResult::passed(self.id(), self.name(), self.description(), duration)
                } else {
                    CheckResult::failed(
                        self.id(), self.name(), self.description(), duration,
                        "Clippy found warnings treated as errors",
                        Some("Run `cargo clippy` and address the reported lints".to_string()),
                    )
                    .with_output(stderr)
                }
            }
        }
    }
}
