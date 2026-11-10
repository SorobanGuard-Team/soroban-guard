use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;
use sorobanguard_core::config::SorobanGuardConfig;

use crate::cli::InitArgs;

const DEFAULT_CONFIG: &str = r#"# SorobanGuard configuration
# See: https://github.com/your-org/sorobanguard/docs/configuration.md

[project]
# name = "my-contract"      # defaults to [package].name from Cargo.toml
# contract_dir = "."        # path to the contract crate root

[checks]
max_wasm_size = 262144       # 256 KiB — set to 0 to disable
fail_on_wasm_size = true
run_clippy = true
run_fmt_check = true
skip = []                    # e.g. ["fmt_check", "clippy"]

[report]
# output_dir = "sorobanguard-reports"
json = true
markdown = true
no_color = false
"#;

pub async fn run(project_dir: PathBuf, args: InitArgs) -> Result<i32> {
    println!("{}", "SorobanGuard — initialising project…".bold());

    let config_path = project_dir.join("sorobanguard.toml");
    if config_path.exists() {
        println!("  {} sorobanguard.toml already exists — skipping.", "⚠".yellow());
    } else {
        std::fs::write(&config_path, DEFAULT_CONFIG)?;
        println!("  {} Created sorobanguard.toml", "✓".green());
    }

    // Add sorobanguard-reports to .gitignore if present
    let gitignore_path = project_dir.join(".gitignore");
    let entry = "sorobanguard-reports/\n";
    if gitignore_path.exists() {
        let content = std::fs::read_to_string(&gitignore_path)?;
        if !content.contains("sorobanguard-reports") {
            let mut updated = content;
            updated.push_str(entry);
            std::fs::write(&gitignore_path, updated)?;
            println!("  {} Added sorobanguard-reports/ to .gitignore", "✓".green());
        }
    }

    println!();
    println!("{}", "Setup complete. Next steps:".bold());
    println!("  1. Run {} to validate your contract", "`sorobanguard check`".cyan());
    println!("  2. See {} for all available options", "`sorobanguard --help`".cyan());

    Ok(0)
}
