use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;
use sorobanguard_core::{context::ProjectContext, wasm::WasmArtifact};

use crate::cli::InspectArgs;

pub async fn run(project_dir: PathBuf, args: InspectArgs) -> Result<i32> {
    println!("{}", "SorobanGuard — inspecting WASM artifact…".bold());

    let wasm_path = if let Some(p) = args.wasm_path {
        p
    } else {
        let ctx = ProjectContext::load(project_dir)?;
        ctx.config
            .wasm
            .artifact_path
            .clone()
            .unwrap_or_else(|| ctx.project.expected_wasm_path())
    };

    if !wasm_path.exists() {
        eprintln!(
            "{} WASM artifact not found at {}\n  Run `sorobanguard build` first.",
            "✗".red(),
            wasm_path.display()
        );
        return Ok(1);
    }

    let artifact = WasmArtifact::inspect(&wasm_path)?;

    println!();
    println!("  {} Path     : {}", "•".dimmed(), artifact.path.display());
    println!("  {} Size     : {:.1} KiB ({} bytes)", "•".dimmed(), artifact.size_kb(), artifact.size_bytes);
    println!("  {} Valid    : {}", "•".dimmed(), if artifact.is_valid_wasm { "yes".green().to_string() } else { "no".red().to_string() });
    println!("  {} Has Soroban env : {}", "•".dimmed(), artifact.has_soroban_env_interface);
    println!("  {} Exports  : {}", "•".dimmed(), artifact.exports.len());

    if args.full || !artifact.exports.is_empty() {
        for export in &artifact.exports {
            println!("      - {export}");
        }
    }

    println!("  {} Imports  : {}", "•".dimmed(), artifact.imports.len());
    if args.full {
        for import in &artifact.imports {
            println!("      - {}::{}", import.module, import.name);
        }
    }

    println!("  {} Sections : {}", "•".dimmed(), artifact.section_names.join(", "));
    println!();

    Ok(0)
}
