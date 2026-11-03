use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::error::CoreError;

/// Basic information about the Soroban project being validated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,
    pub root_dir: PathBuf,
    pub cargo_toml_path: PathBuf,
    pub is_workspace: bool,
    pub has_soroban_sdk: bool,
}

impl ProjectInfo {
    pub fn from_dir(dir: &Path) -> Result<Self, CoreError> {
        let cargo_toml_path = dir.join("Cargo.toml");
        if !cargo_toml_path.exists() {
            return Err(CoreError::CargoTomlInvalid(format!(
                "Cargo.toml not found in {}",
                dir.display()
            )));
        }

        let raw = std::fs::read_to_string(&cargo_toml_path)?;
        let manifest: toml::Value = toml::from_str(&raw)
            .map_err(|e| CoreError::CargoTomlInvalid(e.to_string()))?;

        let is_workspace = manifest.get("workspace").is_some();

        let (name, version) = if let Some(pkg) = manifest.get("package") {
            let name = pkg.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            let version = pkg.get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("0.0.0")
                .to_string();
            (name, version)
        } else if is_workspace {
            ("workspace".to_string(), "0.0.0".to_string())
        } else {
            return Err(CoreError::CargoTomlInvalid(
                "Cargo.toml has neither [package] nor [workspace]".to_string(),
            ));
        };

        let has_soroban_sdk = raw.contains("soroban-sdk");

        Ok(Self {
            name,
            version,
            root_dir: dir.to_path_buf(),
            cargo_toml_path,
            is_workspace,
            has_soroban_sdk,
        })
    }

    /// Returns the expected WASM output path under target/wasm32v1-none/release.
    /// stellar contract build uses the wasm32v1-none target (requires Rust 1.84+).
    pub fn expected_wasm_path(&self) -> PathBuf {
        let wasm_name = self.name.replace('-', "_");
        self.root_dir
            .join("target")
            .join("wasm32v1-none")
            .join("release")
            .join(format!("{wasm_name}.wasm"))
    }
}
