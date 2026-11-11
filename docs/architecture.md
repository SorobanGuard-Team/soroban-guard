# SorobanGuard — Architecture

## Overview

SorobanGuard is structured as a Rust workspace of independent crates that compose into a CLI tool and a reusable library.

```
┌─────────────────────────────────────────────────────┐
│                 sorobanguard (CLI)                  │
│  crates/cli — clap commands, dispatch, exit codes  │
└────────────────────┬────────────────────────────────┘
                     │
          ┌──────────┼──────────┐
          ▼          ▼          ▼
   ┌────────────┐ ┌──────────┐ ┌────────────────┐
   │  core      │ │validators│ │  reporters     │
   │  types,    │ │  each    │ │  terminal,     │
   │  pipeline, │ │  check   │ │  JSON,         │
   │  config    │ │  impl.   │ │  Markdown      │
   └────────────┘ └──────────┘ └────────────────┘
                                        
   ┌──────────────────────────┐
   │     test-utils           │
   │  helpers for Soroban     │
   │  contract test suites    │
   └──────────────────────────┘
```

## Crate responsibilities

### `sorobanguard-core`

Defines the shared types that every other crate depends on:

- `CheckResult` / `CheckStatus` — structured output from a single check.
- `Pipeline` / `PipelineReport` — runs a sequence of validators and collects results.
- `Validator` trait — the interface every check implements.
- `ProjectContext` — assembled project + toolchain + config passed to validators.
- `ProjectInfo` — extracted from `Cargo.toml`.
- `ToolchainInfo` — detected from the system PATH.
- `WasmArtifact` — inspected from a compiled `.wasm` file.
- `SorobanGuardConfig` — loaded from `sorobanguard.toml`.

### `sorobanguard-validators`

Implements the `Validator` trait for each individual check. Each check lives in its own module. Adding a new check requires creating a new module and registering it in `default_pipeline()`.

### `sorobanguard-reporters`

Implements three output modes:
- `TerminalReporter` — coloured table output to stdout.
- `JsonReporter` — serialises the full `PipelineReport` to `sorobanguard-report.json`.
- `MarkdownReporter` — produces `sorobanguard-report.md`.

### `sorobanguard` (CLI)

Parses command-line arguments with `clap`, loads a `ProjectContext`, wires together validators and reporters, and exits with the appropriate exit code (0 = all checks passed, 1 = one or more failures).

### `sorobanguard-test-utils`

A helper library for Soroban contract test suites. It wraps `soroban_sdk::Env` and provides utilities for address generation, ledger manipulation, and event assertions. It compiles against the host target, not `wasm32v1-none`.

## Contract crates (outside the workspace)

The template and example contracts (`templates/basic-contract`, `examples/token-contract`, `examples/escrow-contract`) are excluded from the workspace and must be built with `stellar contract build`. They depend on `soroban-sdk` v29 and target `wasm32v1-none` for production builds. Their `[dev-dependencies]` enable the `testutils` feature so tests run on the native host.

## Data flow

```
1. User runs `sorobanguard check`
2. CLI loads ProjectContext (config + project info + toolchain)
3. Pipeline runs each Validator sequentially
4. Each Validator returns a CheckResult
5. PipelineReport is assembled
6. TerminalReporter prints to stdout
7. JsonReporter and MarkdownReporter write files
8. CLI exits with code 0 or 1
```

## Extension points

To add a new check:
1. Create `crates/validators/src/my_check.rs` implementing `Validator`.
2. Export it from `crates/validators/src/lib.rs`.
3. Add it to the `validators` vec in `default_pipeline()`.
4. Add tests in the same file.

## Dependency policy

- No network calls are made at runtime.
- No arbitrary code is loaded from configuration.
- External tool execution is limited to `cargo`, `stellar`/`soroban`, `rustfmt`, and `clippy` — all standard Rust/Stellar development tools.
