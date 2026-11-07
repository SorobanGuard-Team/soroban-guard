use std::time::Instant;

use sorobanguard_core::{
    check::{CheckResult, CheckStatus},
    context::ProjectContext,
    pipeline::Validator,
    wasm::WasmArtifact,
};

pub struct WasmSizeValidator;

impl Validator for WasmSizeValidator {
    fn id(&self) -> &str { "wasm_size" }
    fn name(&self) -> &str { "WASM artifact size and validity" }
    fn description(&self) -> &str {
        "Checks WASM file size against configured limit and validates the binary structure"
    }

    fn run(&self, ctx: &ProjectContext) -> CheckResult {
        let start = Instant::now();

        let wasm_path = ctx
            .config
            .wasm
            .artifact_path
            .clone()
            .unwrap_or_else(|| ctx.project.expected_wasm_path());

        if !wasm_path.exists() {
            return CheckResult::skipped(
                self.id(), self.name(), self.description(),
                "WASM artifact not found — run wasm_exists check first",
            );
        }

        let artifact = match WasmArtifact::inspect(&wasm_path) {
            Ok(a) => a,
            Err(e) => {
                return CheckResult::failed(
                    self.id(), self.name(), self.description(), start.elapsed(),
                    format!("Failed to inspect WASM: {e}"),
                    None,
                );
            }
        };

        let duration = start.elapsed();
        let limit = ctx.config.checks.max_wasm_size;

        let output = format!(
            "Size: {:.1} KiB | Valid WASM: {} | Exports: {} | Has soroban env: {}",
            artifact.size_kb(),
            artifact.is_valid_wasm,
            artifact.exports.len(),
            artifact.has_soroban_env_interface,
        );

        if !artifact.is_valid_wasm {
            return CheckResult::failed(
                self.id(), self.name(), self.description(), duration,
                "WASM binary failed structural validation",
                Some("Rebuild with `stellar contract build`".to_string()),
            )
            .with_output(output);
        }

        if artifact.exceeds_size_limit(limit) {
            let msg = format!(
                "WASM size {:.1} KiB exceeds limit {:.1} KiB",
                artifact.size_kb(),
                limit as f64 / 1024.0
            );
            if ctx.config.checks.fail_on_wasm_size {
                return CheckResult::failed(
                    self.id(), self.name(), self.description(), duration,
                    msg,
                    Some("Enable LTO and strip symbols in Cargo.toml [profile.release]".to_string()),
                )
                .with_output(output);
            } else {
                let mut result = CheckResult::passed(self.id(), self.name(), self.description(), duration)
                    .with_output(output);
                result.status = CheckStatus::Warning;
                result.error = Some(msg);
                return result;
            }
        }

        CheckResult::passed(self.id(), self.name(), self.description(), duration)
            .with_output(output)
    }
}
