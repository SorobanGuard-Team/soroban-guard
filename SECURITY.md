# Security Policy

## Scope

SorobanGuard is a developer tooling project. It runs locally on developers' machines and in CI pipelines. It does not connect to Stellar mainnet, does not handle private keys, and does not deploy contracts.

## Supported versions

| Version | Supported |
|---|---|
| 0.x (pre-release) | Active development — latest commit on `main` |

## Reporting a vulnerability

If you discover a security issue in SorobanGuard:

1. **Do not open a public GitHub issue.**
2. Email the maintainers directly at the address in the repository's GitHub security advisory settings.
3. Include a description of the vulnerability, steps to reproduce, and potential impact.

We will acknowledge receipt within 48 hours and aim to publish a fix within 14 days.

## Security considerations

- SorobanGuard executes `cargo`, `rustc`, `stellar`, and `rustfmt` from the system PATH. Ensure your PATH is not compromised.
- SorobanGuard reads `sorobanguard.toml` from the project directory. It does not execute shell commands defined in that file.
- No private keys or wallet credentials are required or stored.
- Do not place private keys in `.env` files in directories accessible to untrusted processes.
