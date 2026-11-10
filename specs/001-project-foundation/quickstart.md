# Foundation Quickstart and Validation Guide

**Status**: Implemented and verified locally on Linux with Rust 1.99.1, including an isolated
clean-copy run without credentials or the local feature selector. All documented runtime scenarios
and four Cargo checks passed. Hosted CI is configured but unverified; no remote is configured.

## Staged Use of This Guide

T013 verifies only the Build and Verify, Run the Foundation, and Validate Failures and Redaction
sections after US1 source exists. It does not require README, AGENTS, architecture/security docs,
or a hosted CI run that later tasks have not delivered. The final T032 walkthrough executes the
README and this complete guide; T027/T033–T035 cover continuity, security, and artifact reviews.
All referenced deliverables now exist; the full walkthrough passed during final verification.

## Prerequisites

Install rustup from the official [Rust installation guide](https://www.rust-lang.org/tools/install).
Use Rust 1.99.1 with rustfmt and Clippy, as the existing rust-toolchain.toml specifies. Initial
installation and crate downloads need network access; no PostgreSQL process, Discord account,
banking access, or production credentials are needed. No .env loading is required.

## Build and Verify

From the repository root after implementation:

```bash
rustup show
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check
```

Expected: all commands succeed; no Clippy warnings. The committed Cargo.lock must remain stable.
CI runs corresponding dependency-resolving commands with --locked. Review the actual hosted run
before claiming CI success.

## Run the Foundation

```bash
env -u FUNDGUARD_LOG_LEVEL cargo run --locked
FUNDGUARD_LOG_LEVEL=debug cargo run --locked
FUNDGUARD_LOG_LEVEL=warn cargo run --locked
```

Expected: all exit successfully. Default and debug show the application_initialized tracing event
on stderr; warn suppresses INFO. Cargo may print its own build status; runtime stdout remains empty.
No integrations are contacted. .env.example documents the optional setting only; do not supply
Discord or database credentials. See [runtime contract](contracts/runtime.md) for exact semantics.

## Validate Failures and Redaction

```bash
FUNDGUARD_LOG_LEVEL=invalid-foundation-marker cargo run --locked
FUNDGUARD_LOG_LEVEL= cargo run --locked
```

Expected: each exits nonzero, identifies FUNDGUARD_LOG_LEVEL and accepted values, and does not echo
the supplied invalid marker. Startup errors use an ERROR tracing event with stable field
application_initialization_failed, not a plain-text fallback. Automated tests assert that event,
empty application stdout, and redaction. They also cover uppercase, whitespace, and non-Unicode
values where supported. Subprocess tests inject synthetic ambient token/database markers and
assert that both streams exclude them; never use real secrets for verification.

## Review Architecture and Continuity

Review the implemented source against [plan.md](plan.md): four cohesive source modules, narrow
library exports, no unused integrations or financial operations, no empty future layers. Inspect
`cargo tree -e features` for the intended minimal dependency features. Follow README and AGENTS.md
to product, architecture, standards, security, active spec, future tasks, and handoff.

Review the preliminary threat model and all spec security/financial constraints. Confirm that the
constitution amendment is adopted and significant choices are recorded in ADRs. Verify every
required documentation link, inspect repository diff and lockfile, and run `git diff --check`.

Record actual pass/fail results and remaining gaps in the authoritative tasks and session handoff.
This guide demonstrates foundation behavior only; it proves no future financial workflows,
production deployment, bank verification, or Discord authorization.
