use std::time::Instant;

use sorobanguard_core::{check::CheckResult, context::ProjectContext, pipeline::Validator};

pub struct WasmExistsValidator;

impl Validator for WasmExistsValidator {
    fn id(&self) -> &str { "wasm_exists" }
    fn name(&self) -> &str { "WASM artifact existence" }
    fn description(&self) -> &str {
        "Verifies that the compiled WASM artifact is present under target/wasm32v1-none/release/"
    }

    fn run(&self, ctx: &ProjectContext) -> CheckResult {
        let start = Instant::now();

        let wasm_path = ctx
            .config
            .wasm
            .artifact_path
            .clone()
            .unwrap_or_else(|| ctx.project.expected_wasm_path());

        let duration = start.elapsed();

        if wasm_path.exists() {
            CheckResult::passed(self.id(), self.name(), self.description(), duration)
                .with_output(format!("Found: {}", wasm_path.display()))
        } else {
            CheckResult::failed(
                self.id(), self.name(), self.description(), duration,
                format!("WASM artifact not found at {}", wasm_path.display()),
                Some("Run `sorobanguard build` or `stellar contract build` first".to_string()),
            )
        }
    }
}
