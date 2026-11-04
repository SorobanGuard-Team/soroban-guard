use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::error::CoreError;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolchainInfo {
    pub rust_version: Option<String>,
    pub cargo_version: Option<String>,
    pub stellar_cli_version: Option<String>,
    pub wasm_target_installed: bool,
    pub rust_toolchain_file: Option<String>,
}

impl ToolchainInfo {
    pub fn detect() -> Self {
        Self {
            rust_version: run_version_command("rustc", &["--version"]),
            cargo_version: run_version_command("cargo", &["--version"]),
            stellar_cli_version: detect_stellar_cli(),
            wasm_target_installed: check_wasm_target(),
            rust_toolchain_file: None,
        }
    }

    pub fn stellar_cli_available(&self) -> bool {
        self.stellar_cli_version.is_some()
    }

    pub fn wasm_target_ok(&self) -> bool {
        self.wasm_target_installed
    }
}

fn run_version_command(program: &str, args: &[&str]) -> Option<String> {
    Command::new(program)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

fn detect_stellar_cli() -> Option<String> {
    // `stellar --version` is the current command (stellar-cli 20+)
    run_version_command("stellar", &["--version"])
        .or_else(|| run_version_command("soroban", &["--version"]))
}

fn check_wasm_target() -> bool {
    Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .ok()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .contains("wasm32-unknown-unknown")
        })
        .unwrap_or(false)
}

pub fn require_tool(program: &str, install_hint: &str) -> Result<(), CoreError> {
    if which::which(program).is_err() {
        return Err(CoreError::ToolNotFound {
            tool: program.to_string(),
            install_hint: install_hint.to_string(),
        });
    }
    Ok(())
}
