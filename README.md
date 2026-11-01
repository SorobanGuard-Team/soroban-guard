# SorobanGuard

**The open-source testing and quality toolkit for Stellar Soroban smart contracts.**

[![CI](https://github.com/your-org/sorobanguard/actions/workflows/ci.yml/badge.svg)](https://github.com/your-org/sorobanguard/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

---

## The problem

Soroban developers working on Stellar smart contracts need reliable ways to validate their contracts before deployment. Building, testing, inspecting WASM artifacts and integrating quality checks into CI requires assembling multiple tools and writing custom scripts. SorobanGuard packages these workflows into a single, reusable open-source toolkit.

## The solution

SorobanGuard provides:

- A **command-line interface** (`sorobanguard`) that runs a configurable validation pipeline.
- A modular **validation engine** covering linting, builds, tests, WASM inspection, and size checks.
- Reusable **testing helpers** (`sorobanguard-test-utils`) for Soroban contract test suites.
- **GitHub Actions integration** so the same pipeline runs in every PR.
- **JSON and Markdown reports** you can read locally or attach to CI runs.
- **Starter templates and examples** (counter, token, escrow) you can copy into new projects.

## Who should use this

- Soroban smart contract developers who want automated quality gates.
- Teams adding contract validation to their CI/CD pipeline.
- Open-source contributors who want a standard way to validate contributed contracts.

---

## Requirements

| Tool | Minimum version | Install |
|---|---|---|
| Rust (stable) | 1.84.0 | `rustup update stable` |
| `wasm32v1-none` target | — | `rustup target add wasm32v1-none` |
| stellar-cli | 30.x | `cargo install stellar-cli --features opt` |
| rustfmt | — | `rustup component add rustfmt` |
| clippy | — | `rustup component add clippy` |

> **Why Rust 1.84?** The `wasm32v1-none` target, required by the Soroban runtime, is only available from Rust 1.84+. Older targets (`wasm32-unknown-unknown`) are not supported by the Soroban runtime when using Rust 1.82 or newer.

---

## Installation

### From source (recommended during early development)

```bash
git clone https://github.com/your-org/sorobanguard
cd sorobanguard
cargo install --path crates/cli
```

### Verify installation

```bash
sorobanguard --version
```

---

## Quick start

```bash
# Navigate into an existing Soroban project
cd my-soroban-contract

# 1. Initialise SorobanGuard (creates sorobanguard.toml and updates .gitignore)
sorobanguard init

# 2. Run the full validation pipeline
sorobanguard check

# 3. Build the contract
sorobanguard build

# 4. Run the test suite
sorobanguard test

# 5. Inspect the WASM artifact
sorobanguard inspect

# 6. Regenerate reports from the last run
sorobanguard report
```

---

## Commands

| Command | Description |
|---|---|
| `sorobanguard init` | Initialise SorobanGuard in the current project |
| `sorobanguard check` | Run the full validation pipeline |
| `sorobanguard build` | Build the contract with `stellar contract build` |
| `sorobanguard test` | Run `cargo test` |
| `sorobanguard inspect [WASM]` | Inspect WASM artifact metadata |
| `sorobanguard report` | Regenerate JSON/Markdown reports from last run |
| `sorobanguard completions <SHELL>` | Print shell completion script |

### Common flags

```
-p, --project <DIR>   Path to the project (defaults to current directory)
-v, --verbose         Enable debug logging
    --no-color        Disable terminal colours
    --help            Print help
    --version         Print version
```

### `sorobanguard check` flags

```
--report-dir <DIR>              Output directory for reports
--strict-size                   Treat WASM size warnings as errors
--skip <id,id,...>              Skip named checks
--only <id,id,...>              Run only named checks
```

---

## Validation checks

| ID | Name | What it checks |
|---|---|---|
| `toolchain` | Toolchain | rustc, cargo, stellar-cli, wasm32v1-none target |
| `cargo_config` | Cargo.toml | Valid TOML, `cdylib` crate-type, soroban-sdk present |
| `cargo_deps` | Dependencies | `cargo check` passes |
| `fmt_check` | Formatting | `cargo fmt --check` |
| `clippy` | Clippy lints | `cargo clippy -- -D warnings` |
| `contract_build` | Build | `stellar contract build` succeeds |
| `contract_tests` | Tests | `cargo test` passes |
| `wasm_exists` | WASM exists | Artifact present under `target/wasm32v1-none/release/` |
| `wasm_size` | WASM size | Size within configured limit; binary structurally valid |

Skipping a check: `sorobanguard check --skip fmt_check,clippy`

---

## Configuration

`sorobanguard init` creates `sorobanguard.toml` in your project root:

```toml
[project]
# name = "my-contract"

[checks]
max_wasm_size = 262144   # 256 KiB
fail_on_wasm_size = true
run_clippy = true
run_fmt_check = true
skip = []

[report]
json = true
markdown = true
no_color = false
```

---

## Reports

After `sorobanguard check`, reports land in `sorobanguard-reports/`:

- `sorobanguard-report.json` — machine-readable, suitable for CI artifact upload.
- `sorobanguard-report.md` — human-readable with a results table, failure details, and toolchain information.

---

## GitHub Actions

Copy `.github/workflows/ci.yml` from this repository into your project.

The workflow:
1. Installs Rust + wasm32v1-none target.
2. Installs stellar-cli.
3. Runs `cargo fmt --check` and `cargo clippy`.
4. Runs `cargo test` (Linux, Windows, macOS).
5. Builds WASM artifacts with `stellar contract build`.
6. Runs `sorobanguard check` on the contracts.
7. Uploads reports as CI artifacts.

No funded wallet or mainnet credentials are required for any of these steps.

---

## Project architecture

```
sorobanguard/
  crates/
    cli/            # `sorobanguard` binary — CLI entry point and command dispatch
    core/           # Validation engine types (CheckResult, Pipeline, ProjectInfo, WasmArtifact)
    validators/     # Individual check implementations (toolchain, build, tests, wasm, …)
    reporters/      # Terminal, JSON and Markdown reporters
    test-utils/     # Reusable Soroban test helpers (TestEnv, ledger helpers, event assertions)
  templates/
    basic-contract/ # Starter contract (counter with auth and events)
  examples/
    token-contract/ # Fungible token example
    escrow-contract/# Two-party escrow example
  .github/
    workflows/      # CI workflow
    ISSUE_TEMPLATE/ # Contributor issue templates
  docs/             # Architecture, CLI reference, testing guide, …
```

The host-side crates (`cli`, `core`, `validators`, `reporters`, `test-utils`) live in a Rust workspace and compile to native targets. The contract crates are excluded from the workspace and must be built with `stellar contract build` targeting `wasm32v1-none`.

---

## Limitations

- SorobanGuard validates build quality, test results, and WASM structure. It does not perform security auditing.
- WASM inspection reports structural information (exports, imports, size). It does not decompile or audit contract logic.
- Deployment and on-chain interaction are out of scope.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to set up a development environment, run tests, and submit a pull request.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
