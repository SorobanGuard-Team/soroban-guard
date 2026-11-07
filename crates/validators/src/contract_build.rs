use std::time::Instant;

use sorobanguard_core::{check::CheckResult, context::ProjectContext, pipeline::Validator};

pub struct ContractBuildValidator;

impl Validator for ContractBuildValidator {
    fn id(&self) -> &str { "contract_build" }
    fn name(&self) -> &str { "Contract build" }
    fn description(&self) -> &str {
        "Builds the Soroban contract with `stellar contract build` targeting wasm32v1-none"
    }

    fn run(&self, ctx: &ProjectContext) -> CheckResult {
        let start = Instant::now();

        // Prefer `stellar` CLI; fall back to `soroban` for older setups
        let stellar_bin = if which::which("stellar").is_ok() {
            "stellar"
        } else if which::which("soroban").is_ok() {
            "soroban"
        } else {
            return CheckResult::failed(
                self.id(), self.name(), self.description(), start.elapsed(),
                "stellar CLI not found on PATH",
                Some("Install stellar CLI: cargo install stellar-cli --features opt\nSee: https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup".to_string()),
            );
        };

        let output = std::process::Command::new(stellar_bin)
            .args(["contract", "build"])
            .current_dir(&ctx.project.root_dir)
            .envs(&ctx.config.project.env)
            .output();

        let duration = start.elapsed();

        match output {
            Err(e) => CheckResult::failed(
                self.id(), self.name(), self.description(), duration,
                format!("Failed to execute {stellar_bin}: {e}"),
                None,
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
                        "Contract build failed",
                        Some("Run `stellar contract build` and fix reported errors".to_string()),
                    )
                    .with_output(combined)
                }
            }
        }
    }
}
