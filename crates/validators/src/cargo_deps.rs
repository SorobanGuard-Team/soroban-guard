use std::time::Instant;

use sorobanguard_core::{check::CheckResult, context::ProjectContext, pipeline::Validator};

pub struct CargoDepsValidator;

impl Validator for CargoDepsValidator {
    fn id(&self) -> &str { "cargo_deps" }
    fn name(&self) -> &str { "Cargo dependency check" }
    fn description(&self) -> &str {
        "Runs `cargo check` to verify dependencies resolve without compilation errors"
    }

    fn run(&self, ctx: &ProjectContext) -> CheckResult {
        let start = Instant::now();

        let output = std::process::Command::new("cargo")
            .args(["check", "--message-format=short"])
            .current_dir(&ctx.project.root_dir)
            .envs(&ctx.config.project.env)
            .output();

        let duration = start.elapsed();

        match output {
            Err(e) => CheckResult::failed(
                self.id(), self.name(), self.description(), duration,
                format!("Failed to run cargo check: {e}"),
                Some("Ensure cargo is installed and on PATH".to_string()),
            ),
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                if out.status.success() {
                    CheckResult::passed(self.id(), self.name(), self.description(), duration)
                } else {
                    CheckResult::failed(
                        self.id(), self.name(), self.description(), duration,
                        "cargo check failed — dependency or compilation errors found",
                        Some("Run `cargo check` and fix reported errors".to_string()),
                    )
                    .with_output(stderr)
                }
            }
        }
    }
}
