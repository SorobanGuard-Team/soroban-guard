use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Top-level SorobanGuard project configuration, loaded from `sorobanguard.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct SorobanGuardConfig {
    #[serde(default)]
    pub project: ProjectConfig,

    #[serde(default)]
    pub checks: ChecksConfig,

    #[serde(default)]
    pub report: ReportConfig,

    #[serde(default)]
    pub wasm: WasmConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectConfig {
    /// Human-readable project name used in reports.
    pub name: Option<String>,

    /// Path to the Soroban contract directory (defaults to current dir).
    pub contract_dir: Option<PathBuf>,

    /// Additional environment variables injected when running cargo commands.
    #[serde(default)]
    pub env: HashMap<String, String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: None,
            contract_dir: None,
            env: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChecksConfig {
    /// Maximum allowed WASM size in bytes (default: 256 KiB).
    #[serde(default = "default_max_wasm_size")]
    pub max_wasm_size: u64,

    /// Whether to treat WASM size limit as an error (true) or warning (false).
    #[serde(default = "default_true")]
    pub fail_on_wasm_size: bool,

    /// Checks to skip entirely.
    #[serde(default)]
    pub skip: Vec<String>,

    /// Whether to run cargo clippy as part of `check`.
    #[serde(default = "default_true")]
    pub run_clippy: bool,

    /// Whether to run cargo fmt --check as part of `check`.
    #[serde(default = "default_true")]
    pub run_fmt_check: bool,
}

impl Default for ChecksConfig {
    fn default() -> Self {
        Self {
            max_wasm_size: default_max_wasm_size(),
            fail_on_wasm_size: true,
            skip: Vec::new(),
            run_clippy: true,
            run_fmt_check: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportConfig {
    /// Directory where reports are written (default: ./sorobanguard-reports).
    pub output_dir: Option<PathBuf>,

    /// Whether to generate a JSON report.
    #[serde(default = "default_true")]
    pub json: bool,

    /// Whether to generate a Markdown report.
    #[serde(default = "default_true")]
    pub markdown: bool,

    /// Whether to suppress terminal color output.
    #[serde(default)]
    pub no_color: bool,
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            output_dir: None,
            json: true,
            markdown: true,
            no_color: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct WasmConfig {
    /// Override the expected WASM artifact path.
    pub artifact_path: Option<PathBuf>,
}

fn default_max_wasm_size() -> u64 {
    256 * 1024 // 256 KiB
}

fn default_true() -> bool {
    true
}

impl SorobanGuardConfig {
    pub fn load(dir: &std::path::Path) -> anyhow::Result<Self> {
        let config_path = dir.join("sorobanguard.toml");
        if config_path.exists() {
            let raw = std::fs::read_to_string(&config_path)?;
            let config: SorobanGuardConfig = toml::from_str(&raw)
                .map_err(|e| anyhow::anyhow!("Invalid sorobanguard.toml: {e}"))?;
            Ok(config)
        } else {
            Ok(SorobanGuardConfig::default())
        }
    }

    pub fn save(&self, dir: &std::path::Path) -> anyhow::Result<()> {
        let config_path = dir.join("sorobanguard.toml");
        let raw = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, raw)?;
        Ok(())
    }
}
