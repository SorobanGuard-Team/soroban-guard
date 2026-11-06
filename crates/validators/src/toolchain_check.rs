use std::time::Instant;

use sorobanguard_core::{
    check::{CheckOutcome, CheckResult},
    context::ProjectContext,
    pipeline::Validator,
};

pub struct ToolchainValidator;

impl Validator for ToolchainValidator {
    fn id(&self) -> &str { "toolchain" }
    fn name(&self) -> &str { "Toolchain check" }
    fn description(&self) -> &str {
        "Verifies that required tools (rustc, cargo, stellar CLI, wasm32v1-none target) are available"
    }

    fn run(&self, ctx: &ProjectContext) -> CheckResult {
        let start = Instant::now();
        let tc = &ctx.toolchain;
        let mut issues = Vec::new();

        if tc.rust_version.is_none() {
            issues.push("rustc not found. Install Rust from https://rustup.rs");
        }
        if tc.cargo_version.is_none() {
            issues.push("cargo not found. Install Rust from https://rustup.rs");
        }
        if !tc.wasm_target_installed {
            issues.push("wasm32v1-none target not installed. Run: rustup target add wasm32v1-none");
        }

        let stellar_missing = tc.stellar_cli_version.is_none();
        // stellar-cli is recommended but non-fatal for lint-only checks
        let stellar_note = if stellar_missing {
            Some("stellar CLI not found. Install from: https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup")
        } else {
            None
        };

        let duration = start.elapsed();

        if !issues.is_empty() {
            let error = issues.join("; ");
            CheckResult::failed(
                self.id(), self.name(), self.description(), duration,
                error,
                Some("Run: rustup target add wasm32v1-none && cargo install stellar-cli".to_string()),
            )
        } else {
            let mut output = format!(
                "{} | {}",
                tc.rust_version.as_deref().unwrap_or("?"),
                tc.cargo_version.as_deref().unwrap_or("?"),
            );
            if let Some(sv) = &tc.stellar_cli_version {
                output.push_str(&format!(" | {sv}"));
            }
            if let Some(note) = stellar_note {
                output.push_str(&format!("\nNote: {note}"));
            }
            CheckResult::passed(self.id(), self.name(), self.description(), duration)
                .with_output(output)
        }
    }
}
