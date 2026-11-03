use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Project directory not found: {0}")]
    ProjectNotFound(String),

    #[error("Required tool not found: {tool}. Install it with: {install_hint}")]
    ToolNotFound { tool: String, install_hint: String },

    #[error("Cargo.toml not found or invalid: {0}")]
    CargoTomlInvalid(String),

    #[error("Contract build failed: {0}")]
    BuildFailed(String),

    #[error("Tests failed: {0}")]
    TestsFailed(String),

    #[error("WASM artifact not found at {path}. Run `sorobanguard build` first.")]
    WasmNotFound { path: String },

    #[error("WASM validation error: {0}")]
    WasmInvalid(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Process execution error: {0}")]
    Process(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
