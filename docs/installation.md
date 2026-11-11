# Installation

## Prerequisites

### All platforms

Install Rust via rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
```

Add the Soroban WASM target (Rust 1.84+ required):

```bash
rustup target add wasm32v1-none
rustup component add rustfmt clippy
```

Install stellar-cli:

```bash
cargo install stellar-cli --features opt --locked
```

Verify:

```bash
rustc --version          # should be ≥ 1.84.0
stellar --version        # should be ≥ 30.x
rustup target list --installed | grep wasm32v1-none
```

### Windows

The instructions above work in PowerShell or Git Bash on Windows 11.
Ensure `%USERPROFILE%\.cargo\bin` is on your PATH (the rustup installer does this automatically).

For stellar-cli on Windows, the same `cargo install` command works.

### Linux

No additional packages needed beyond those above. If running in a Docker container, start from the official `rust:1.84` image.

### macOS

No additional steps. Homebrew is not required.

## Install SorobanGuard

### From source

```bash
git clone https://github.com/your-org/sorobanguard
cd sorobanguard
cargo install --path crates/cli --locked
```

This installs the `sorobanguard` binary to `~/.cargo/bin/`.

### Verify

```bash
sorobanguard --version
sorobanguard --help
```

## Updating

```bash
cd sorobanguard
git pull
cargo install --path crates/cli --locked --force
```

## Uninstall

```bash
cargo uninstall sorobanguard
```
