use std::time::Instant;

use sorobanguard_core::{check::CheckResult, context::ProjectContext, pipeline::Validator};

pub struct FmtCheckValidator;

impl Validator for FmtCheckValidator {
    fn id(&self) -> &str { "fmt_check" }
    fn name(&self) -> &str { "Code formatting" }
    fn description(&self) -> &str {
        "Runs `cargo fmt --check` to verify code is consistently formatted"
    }

    fn run(&self, ctx: &ProjectContext) -> CheckResult {
        let start = Instant::now();

        if !ctx.config.checks.run_fmt_check {
            return CheckResult::skipped(
                self.id(), self.name(), self.description(),
                "Disabled in sorobanguard.toml (checks.run_fmt_check = false)",
            );
        }

        let output = std::process::Command::new("cargo")
            .args(["fmt", "--check"])
            .current_dir(&ctx.project.root_dir)
            .output();

        let duration = start.elapsed();

        match output {
            Err(e) => CheckResult::failed(
                self.id(), self.name(), self.description(), duration,
                format!("Failed to run cargo fmt: {e}"),
                Some("Install rustfmt: rustup component add rustfmt".to_string()),
            ),
            Ok(out) => {
                if out.status.success() {
                    CheckResult::passed(self.id(), self.name(), self.description(), duration)
                } else {
                    let diff = String::from_utf8_lossy(&out.stdout).to_string();
                    CheckResult::failed(
                        self.id(), self.name(), self.description(), duration,
                        "Code is not formatted — run `cargo fmt` to fix",
                        Some("Run: cargo fmt".to_string()),
                    )
                    .with_output(diff)
                }
            }
        }
    }
}
