# Contributing to SorobanGuard

Thank you for your interest in contributing. This document explains how to set up a development environment, run the tests, and submit a pull request.

## Development prerequisites

| Tool | Version | Install |
|---|---|---|
| Rust stable | ≥ 1.84 | `rustup update stable` |
| wasm32v1-none | — | `rustup target add wasm32v1-none` |
| stellar-cli | ≥ 30 | `cargo install stellar-cli --features opt` |
| rustfmt | — | `rustup component add rustfmt` |
| clippy | — | `rustup component add clippy` |

## Getting the source

```bash
git clone https://github.com/your-org/sorobanguard
cd sorobanguard
```

## Running the workspace tests

```bash
# Lint
cargo fmt --all --check
cargo clippy --workspace -- -D warnings

# Host-side unit and integration tests
cargo test --workspace

# Contract crate tests (each has its own Cargo.toml outside the workspace)
cd templates/basic-contract && cargo test && cd ../..
cd examples/token-contract  && cargo test && cd ../..
cd examples/escrow-contract && cargo test && cd ../..
```

## Building the CLI

```bash
cargo build -p sorobanguard
./target/debug/sorobanguard --help
```

## Building a contract

```bash
cd templates/basic-contract
stellar contract build
# Artifact: target/wasm32v1-none/release/basic_contract.wasm
```

## Running SorobanGuard against the template contract

```bash
# From the workspace root:
./target/debug/sorobanguard check --project templates/basic-contract
```

## Project structure

See the [Architecture](docs/architecture.md) document.

## Submitting a pull request

1. Fork the repository.
2. Create a branch: `git checkout -b feat/your-feature`.
3. Make your changes, add tests for new behaviour.
4. Run `cargo fmt --all` and `cargo clippy --workspace -- -D warnings`.
5. Run all tests (see above).
6. Open a pull request against `main`.

## Code style

- Follow standard Rust style enforced by `rustfmt`.
- Treat clippy warnings as errors (the CI workflow does).
- Keep modules small and independently testable.
- Do not add dependencies without discussion.

## Reporting issues

Use the GitHub issue templates provided in `.github/ISSUE_TEMPLATE/`.

## Code of conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md).
