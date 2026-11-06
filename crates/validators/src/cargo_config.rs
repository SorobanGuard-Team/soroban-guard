use std::time::Instant;

use sorobanguard_core::{check::CheckResult, context::ProjectContext, pipeline::Validator};

pub struct CargoConfigValidator;

impl Validator for CargoConfigValidator {
    fn id(&self) -> &str { "cargo_config" }
    fn name(&self) -> &str { "Cargo.toml configuration" }
    fn description(&self) -> &str {
        "Checks that Cargo.toml is valid and includes required Soroban contract settings"
    }

    fn run(&self, ctx: &ProjectContext) -> CheckResult {
        let start = Instant::now();
        let path = &ctx.project.cargo_toml_path;

        let raw = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                return CheckResult::failed(
                    self.id(), self.name(), self.description(), start.elapsed(),
                    format!("Cannot read Cargo.toml: {e}"),
                    None,
                );
            }
        };

        let manifest: toml::Value = match toml::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                return CheckResult::failed(
                    self.id(), self.name(), self.description(), start.elapsed(),
                    format!("Invalid TOML: {e}"),
                    Some("Fix syntax errors in Cargo.toml".to_string()),
                );
            }
        };

        let mut warnings = Vec::new();

        // For non-workspace contracts, check [lib] crate-type includes cdylib
        if !ctx.project.is_workspace {
            let has_cdylib = manifest
                .get("lib")
                .and_then(|l| l.get("crate-type"))
                .and_then(|ct| ct.as_array())
                .map(|arr| arr.iter().any(|v| v.as_str() == Some("cdylib")))
                .unwrap_or(false);

            if !has_cdylib {
                warnings.push(
                    "[lib] crate-type does not include \"cdylib\". \
                     Soroban contracts must include cdylib to produce a WASM artifact.",
                );
            }
        }

        if !ctx.project.has_soroban_sdk {
            warnings.push("soroban-sdk not found in Cargo.toml dependencies.");
        }

        let duration = start.elapsed();

        if warnings.is_empty() {
            CheckResult::passed(self.id(), self.name(), self.description(), duration)
        } else {
            CheckResult::failed(
                self.id(), self.name(), self.description(), duration,
                warnings.join("; "),
                Some("See https://developers.stellar.org/docs/build/smart-contracts/getting-started".to_string()),
            )
        }
    }
}
