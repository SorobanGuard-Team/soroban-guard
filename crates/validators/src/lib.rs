pub mod cargo_config;
pub mod cargo_deps;
pub mod contract_build;
pub mod contract_tests;
pub mod wasm_exists;
pub mod wasm_size;
pub mod toolchain_check;
pub mod fmt_check;
pub mod clippy_check;

pub use cargo_config::CargoConfigValidator;
pub use cargo_deps::CargoDepsValidator;
pub use contract_build::ContractBuildValidator;
pub use contract_tests::ContractTestsValidator;
pub use wasm_exists::WasmExistsValidator;
pub use wasm_size::WasmSizeValidator;
pub use toolchain_check::ToolchainValidator;
pub use fmt_check::FmtCheckValidator;
pub use clippy_check::ClippyCheckValidator;

use sorobanguard_core::pipeline::{Pipeline, Validator};

/// Build the default validation pipeline with all built-in checks enabled.
pub fn default_pipeline() -> Pipeline {
    let validators: Vec<Box<dyn Validator>> = vec![
        Box::new(ToolchainValidator),
        Box::new(CargoConfigValidator),
        Box::new(CargoDepsValidator),
        Box::new(FmtCheckValidator),
        Box::new(ClippyCheckValidator),
        Box::new(ContractBuildValidator),
        Box::new(ContractTestsValidator),
        Box::new(WasmExistsValidator),
        Box::new(WasmSizeValidator),
    ];
    Pipeline::new(validators)
}
