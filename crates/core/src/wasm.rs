use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use wasmparser::{Parser, Payload};

use crate::error::CoreError;

/// Information extracted from a compiled WASM contract artifact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmArtifact {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub is_valid_wasm: bool,
    pub exports: Vec<String>,
    pub imports: Vec<WasmImport>,
    pub section_names: Vec<String>,
    pub has_soroban_env_interface: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmImport {
    pub module: String,
    pub name: String,
}

impl WasmArtifact {
    pub fn inspect(path: &Path) -> Result<Self, CoreError> {
        if !path.exists() {
            return Err(CoreError::WasmNotFound {
                path: path.display().to_string(),
            });
        }

        let bytes = std::fs::read(path)?;
        let size_bytes = bytes.len() as u64;

        let (is_valid, exports, imports, section_names) = parse_wasm(&bytes);

        let has_soroban_env_interface = imports
            .iter()
            .any(|i| i.module == "env" || i.module == "v1");

        Ok(Self {
            path: path.to_path_buf(),
            size_bytes,
            is_valid_wasm: is_valid,
            exports,
            imports,
            section_names,
            has_soroban_env_interface,
        })
    }

    pub fn size_kb(&self) -> f64 {
        self.size_bytes as f64 / 1024.0
    }

    pub fn exceeds_size_limit(&self, limit_bytes: u64) -> bool {
        self.size_bytes > limit_bytes
    }
}

fn parse_wasm(bytes: &[u8]) -> (bool, Vec<String>, Vec<WasmImport>, Vec<String>) {
    let mut exports = Vec::new();
    let mut imports = Vec::new();
    let mut section_names = Vec::new();
    let mut is_valid = true;

    let parser = Parser::new(0);
    for payload in parser.parse_all(bytes) {
        match payload {
            Ok(Payload::ExportSection(reader)) => {
                for export in reader {
                    if let Ok(e) = export {
                        exports.push(e.name.to_string());
                    }
                }
                section_names.push("export".to_string());
            }
            Ok(Payload::ImportSection(reader)) => {
                for import in reader {
                    if let Ok(i) = import {
                        imports.push(WasmImport {
                            module: i.module.to_string(),
                            name: i.name.to_string(),
                        });
                    }
                }
                section_names.push("import".to_string());
            }
            Ok(Payload::CustomSection(c)) => {
                section_names.push(format!("custom:{}", c.name()));
            }
            Ok(Payload::CodeSectionStart { .. }) => {
                section_names.push("code".to_string());
            }
            Ok(Payload::DataSection(_)) => {
                section_names.push("data".to_string());
            }
            Ok(Payload::TypeSection(_)) => {
                section_names.push("type".to_string());
            }
            Ok(Payload::FunctionSection(_)) => {
                section_names.push("function".to_string());
            }
            Err(_) => {
                is_valid = false;
                break;
            }
            _ => {}
        }
    }

    // Deduplicate section names
    section_names.dedup();

    (is_valid, exports, imports, section_names)
}
