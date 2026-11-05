use std::fs;
use tempfile::TempDir;

use sorobanguard_core::{
    check::{CheckResult, CheckStatus},
    config::SorobanGuardConfig,
    project::ProjectInfo,
    toolchain::ToolchainInfo,
};

#[test]
fn test_default_config_loads_without_file() {
    let dir = TempDir::new().unwrap();
    let config = SorobanGuardConfig::load(dir.path()).unwrap();
    assert_eq!(config.checks.max_wasm_size, 262144);
    assert!(config.checks.run_clippy);
}

#[test]
fn test_config_round_trips() {
    let dir = TempDir::new().unwrap();
    let mut config = SorobanGuardConfig::default();
    config.checks.max_wasm_size = 512 * 1024;
    config.checks.run_clippy = false;
    config.save(dir.path()).unwrap();

    let reloaded = SorobanGuardConfig::load(dir.path()).unwrap();
    assert_eq!(reloaded.checks.max_wasm_size, 512 * 1024);
    assert!(!reloaded.checks.run_clippy);
}

#[test]
fn test_config_invalid_toml_errors() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("sorobanguard.toml"), "{ not valid }").unwrap();
    assert!(SorobanGuardConfig::load(dir.path()).is_err());
}

#[test]
fn test_project_info_missing_cargo_toml() {
    let dir = TempDir::new().unwrap();
    assert!(ProjectInfo::from_dir(dir.path()).is_err());
}

#[test]
fn test_project_info_valid() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"my-contract\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nsoroban-sdk = \"29\"\n",
    ).unwrap();

    let info = ProjectInfo::from_dir(dir.path()).unwrap();
    assert_eq!(info.name, "my-contract");
    assert!(info.has_soroban_sdk);
}

#[test]
fn test_wasm_path_uses_wasm32v1_none() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"counter\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    ).unwrap();
    let info = ProjectInfo::from_dir(dir.path()).unwrap();
    let path = info.expected_wasm_path();
    assert!(path.to_str().unwrap().contains("wasm32v1-none"), "expected wasm32v1-none in path: {}", path.display());
}

#[test]
fn test_check_result_passed() {
    use std::time::Duration;
    let r = CheckResult::passed("x", "X", "desc", Duration::from_millis(10));
    assert_eq!(r.status, CheckStatus::Passed);
    assert!(r.is_passed());
    assert!(!r.is_failed());
}

#[test]
fn test_check_result_failed() {
    use std::time::Duration;
    let r = CheckResult::failed("y", "Y", "desc", Duration::ZERO, "oops", Some("fix it".into()));
    assert_eq!(r.status, CheckStatus::Failed);
    assert!(r.is_failed());
}

#[test]
fn test_check_result_json_roundtrip() {
    use std::time::Duration;
    let r = CheckResult::passed("z", "Z", "d", Duration::from_secs(1));
    let json = serde_json::to_string(&r).unwrap();
    assert!(json.contains("\"status\":\"passed\""));
}

#[test]
fn test_toolchain_detect_no_panic() {
    let _ = ToolchainInfo::detect();
}
