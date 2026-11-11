#!/usr/bin/env bash
# Run this after: git push -u origin main
# Requires: gh auth login

REPO="YOUR_ORG/sorobanguard"   # <-- change this

gh issue create --repo "$REPO" \
  --title "Add --json flag to sorobanguard inspect" \
  --label "good first issue,enhancement,cli" \
  --body "## Problem
The \`inspect\` command prints human-readable output only. Tooling and scripts need machine-readable output.

## Implementation
Add a \`--json\` flag to \`InspectArgs\` in \`crates/cli/src/cli.rs\`. When set, serialise the \`WasmArtifact\` struct to JSON and print it instead of the formatted table.

## Acceptance criteria
- \`sorobanguard inspect --json\` outputs valid JSON
- Output parses as \`WasmArtifact\`
- Tests added in \`crates/cli/tests/\`

## Files
\`crates/cli/src/cli.rs\`, \`crates/cli/src/commands/inspect.rs\`

**Complexity:** Low | **Category:** Good first issue"

gh issue create --repo "$REPO" \
  --title "Add sorobanguard.toml JSON Schema" \
  --label "good first issue,enhancement,documentation" \
  --body "## Problem
There is no schema for \`sorobanguard.toml\`, so editors give no autocomplete or validation.

## Implementation
Generate a JSON Schema from the \`SorobanGuardConfig\` struct using \`schemars\`. Publish it at \`docs/sorobanguard-schema.json\` and document the \`\$schema\` key in the README.

## Acceptance criteria
- Schema is valid JSON Schema draft-07
- The basic-contract template's \`sorobanguard.toml\` validates against it

## Files
\`crates/core/src/config.rs\`, \`docs/\`

**Complexity:** Low | **Category:** Good first issue"

gh issue create --repo "$REPO" \
  --title "Improve error messages when stellar-cli is missing" \
  --label "good first issue,bug,ux" \
  --body "## Problem
When \`stellar\` is not on PATH, the error message mentions \`stellar\` and \`soroban\` but does not link to the installation guide.

## Implementation
In \`crates/validators/src/contract_build.rs\` and \`crates/validators/src/toolchain_check.rs\`, include the installation URL \`https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup\` in the error message and \`fix_hint\`.

## Acceptance criteria
- Error message includes the installation URL
- Test that the error is returned when neither \`stellar\` nor \`soroban\` is found (use a temp PATH in tests)

## Files
\`crates/validators/src/contract_build.rs\`, \`crates/validators/src/toolchain_check.rs\`

**Complexity:** Low | **Category:** Good first issue"

gh issue create --repo "$REPO" \
  --title "Add --quiet flag that suppresses passing checks in terminal output" \
  --label "good first issue,enhancement,cli" \
  --body "## Problem
On large pipelines the terminal output is verbose. A \`--quiet\` mode should show only failures and warnings.

## Implementation
Add \`--quiet\` to \`CheckArgs\`. Pass it to \`TerminalReporter\` and suppress rows where \`status == Passed\`.

## Files
\`crates/cli/src/cli.rs\`, \`crates/reporters/src/terminal.rs\`

**Complexity:** Low | **Category:** Good first issue"

gh issue create --repo "$REPO" \
  --title "Implement --only check filtering in sorobanguard check" \
  --label "enhancement,cli" \
  --body "## Problem
The \`--only <id,id,...>\` flag is accepted by the CLI but not yet wired into the pipeline. All checks always run.

## Implementation
In \`crates/cli/src/commands/check.rs\`, filter the validator list to only include validators whose \`id()\` is in the \`args.only\` list when it is non-empty. This requires either exposing individual validators from \`sorobanguard-validators\` or building a filtered pipeline.

## Acceptance criteria
- \`sorobanguard check --only toolchain,wasm_exists\` runs exactly those two checks
- Integration test covers this behaviour

## Files
\`crates/cli/src/commands/check.rs\`, \`crates/validators/src/lib.rs\`

**Complexity:** Medium | **Category:** CLI development"

gh issue create --repo "$REPO" \
  --title "Add sorobanguard check --fail-fast flag" \
  --label "enhancement,cli" \
  --body "## Problem
The pipeline always runs all checks even after a critical failure (e.g., build fails). A \`--fail-fast\` flag would stop after the first failure, saving time.

## Implementation
Add \`--fail-fast\` to \`CheckArgs\`. Thread it through the pipeline \`run()\` method so it returns early after the first failed \`CheckResult\`.

## Files
\`crates/cli/src/cli.rs\`, \`crates/core/src/pipeline.rs\`

**Complexity:** Medium | **Category:** CLI development"

gh issue create --repo "$REPO" \
  --title "Detect and report Rust MSRV from rust-toolchain.toml" \
  --label "enhancement,soroban-integration" \
  --body "## Problem
A project may specify a Rust version in \`rust-toolchain.toml\`. SorobanGuard should detect this and include it in \`ToolchainInfo\` and the report.

## Implementation
In \`crates/core/src/toolchain.rs\`, read \`rust-toolchain.toml\` from the project root if present and extract the \`channel\` field. Surface it in \`ToolchainInfo.rust_toolchain_file\`.

## Files
\`crates/core/src/toolchain.rs\`, \`crates/core/src/project.rs\`

**Complexity:** Low | **Category:** Rust and Soroban integration"

gh issue create --repo "$REPO" \
  --title "Add validator that checks [profile.release] for Soroban-recommended settings" \
  --label "enhancement,soroban-integration,validator" \
  --body "## Problem
A correctly configured Soroban contract should have \`opt-level = \"z\"\`, \`lto = true\`, \`panic = \"abort\"\`, and \`strip = \"symbols\"\` in \`[profile.release]\`. Missing these produces larger, slower WASM.

## Implementation
Create \`crates/validators/src/release_profile.rs\` implementing \`Validator\`. Parse the \`[profile.release]\` table from \`Cargo.toml\` and check for the recommended keys.

## Acceptance criteria
- Check passes when all four keys are present with recommended values
- Check emits a warning (not error) when keys are missing
- Tests with both a compliant and non-compliant \`Cargo.toml\`

## Files
\`crates/validators/src/release_profile.rs\`, \`crates/validators/src/lib.rs\`

**Complexity:** Medium | **Category:** Rust and Soroban integration"

gh issue create --repo "$REPO" \
  --title "Add MockTokenHelper to sorobanguard-test-utils" \
  --label "enhancement,testing" \
  --body "## Problem
Test suites that need mock token balances must write their own setup code. A reusable \`MockTokenHelper\` would reduce boilerplate.

## Implementation
In \`crates/test-utils/src/\`, add a \`token.rs\` module that provides a \`MockTokenHelper\` struct. It should wrap a registered test token contract and expose \`mint()\`, \`balance()\`, and \`transfer()\` methods backed by the basic \`token-contract\` example.

## Acceptance criteria
- \`MockTokenHelper::new(&env)\` deploys the token contract
- \`mint\`, \`balance\`, \`transfer\` work in a test
- Example test in \`crates/test-utils/tests/\`

## Files
\`crates/test-utils/src/token.rs\`, \`crates/test-utils/src/lib.rs\`

**Complexity:** Medium | **Category:** Testing utilities | **Depends on:** examples/token-contract"

gh issue create --repo "$REPO" \
  --title "Add snapshot_test! macro for contract state snapshots" \
  --label "enhancement,testing" \
  --body "## Problem
Verifying exact contract state across multiple operations is verbose. A macro that serialises state to JSON and compares against a saved snapshot would simplify regression testing.

## Implementation
Add \`crates/test-utils/src/snapshot.rs\`. The macro serialises a value to JSON, reads an optional fixture file, and either asserts equality or writes a new fixture (when \`UPDATE_SNAPSHOTS=1\`).

## Files
\`crates/test-utils/src/snapshot.rs\`

**Complexity:** Medium | **Category:** Testing utilities"

gh issue create --repo "$REPO" \
  --title "Report contractspecv0 custom section metadata in sorobanguard inspect" \
  --label "enhancement,wasm-inspection" \
  --body "## Problem
Soroban contracts compiled with \`soroban-sdk\` include a \`contractspecv0\` custom WASM section with the contract's ABI. \`sorobanguard inspect\` does not surface this.

## Implementation
In \`crates/core/src/wasm.rs\`, detect the \`contractspecv0\` custom section in the WASM parser loop. If present, include it in \`WasmArtifact.section_names\` and add an \`has_contract_spec: bool\` field. Display it in \`sorobanguard inspect\`.

## Files
\`crates/core/src/wasm.rs\`, \`crates/cli/src/commands/inspect.rs\`

**Complexity:** Medium | **Category:** WASM inspection"

gh issue create --repo "$REPO" \
  --title "Add reusable composite GitHub Action for SorobanGuard" \
  --label "enhancement,github-actions" \
  --body "## Problem
Projects that want to use SorobanGuard in CI must copy the full workflow. A reusable composite action at \`actions/sorobanguard/action.yml\` would let them call it with one \`uses:\` step.

## Implementation
Create \`actions/sorobanguard/action.yml\` as a composite action that installs Rust, installs stellar-cli, builds sorobanguard from source, and runs \`sorobanguard check\`. Accept inputs for \`project-dir\`, \`report-dir\`, and \`rust-version\`.

## Acceptance criteria
- Action runs successfully in the CI workflow using \`uses: ./actions/sorobanguard\`
- README documents the action's inputs

## Files
\`actions/sorobanguard/action.yml\`

**Complexity:** Medium | **Category:** GitHub Actions"

gh issue create --repo "$REPO" \
  --title "Add HTML report output format" \
  --label "enhancement,reporting" \
  --body "## Problem
JSON and Markdown reports are produced but no interactive HTML report exists. An HTML report with a sortable table would be more useful for local review.

## Implementation
Add \`crates/reporters/src/html_reporter.rs\`. Use inline CSS (no external dependencies). Sort rows: failures first, then warnings, then passed, then skipped. Add a \`--html\` flag to \`sorobanguard check\` and \`sorobanguard report\`.

## Files
\`crates/reporters/src/html_reporter.rs\`, \`crates/reporters/src/lib.rs\`, \`crates/cli/src/cli.rs\`

**Complexity:** Medium | **Category:** Reporting"

gh issue create --repo "$REPO" \
  --title "Write CLI reference documentation" \
  --label "documentation,good first issue" \
  --body "## Problem
\`docs/cli-reference.md\` is listed in the architecture document but does not yet exist.

## Implementation
Write \`docs/cli-reference.md\` covering every command, all flags, exit codes, configuration keys, check IDs, and example invocations. Keep it in sync with the CLI's \`--help\` output.

## Files
\`docs/cli-reference.md\`

**Complexity:** Low | **Category:** Documentation"

gh issue create --repo "$REPO" \
  --title "Add validator that detects unsafe code blocks in contract source" \
  --label "enhancement,security,validator" \
  --body "## Problem
Soroban contracts should avoid \`unsafe\` code. SorobanGuard could warn when \`unsafe\` blocks are present.

## Implementation
Create \`crates/validators/src/unsafe_check.rs\`. Walk \`.rs\` files in the contract source tree using \`walkdir\`. Search for the string \`unsafe \` using a simple scan. Emit a warning (not an error) for each file containing \`unsafe\`.

## Acceptance criteria
- Check passes when no \`unsafe\` blocks are found
- Check emits a \`Warning\` result (not \`Failed\`) when \`unsafe\` is found, with the file names listed
- Tests with a fixture containing \`unsafe\` and one without

## Files
\`crates/validators/src/unsafe_check.rs\`, \`crates/validators/src/lib.rs\`

**Complexity:** Low | **Category:** Security and performance"

echo ""
echo "All 15 issues created successfully."
