# Financial Core Validation Guide

**Status**: Verified locally on 2026-11-12. The Financial Core library and its tests are implemented.
Existing user edits were preserved. The existing main remains an offline bootstrap.

## Prerequisites

Use the existing rust-toolchain.toml (Rust 1.99.1, rustfmt, Clippy) and Cargo.lock. No dependency
upgrade, bank/Discord account, PostgreSQL service, token, .env entry or feature flag is required.
Initial compiler/dependency installation may use the network; core validation/tests do not.

## Verification commands

From the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --test financial_core
cargo test
cargo check
```

All commands above passed locally: 15 public-contract tests, two reference unit tests and ten
existing foundation tests (27 total). Clippy reported no warnings. Tests used no services or
credentials; manifest/lockfile stayed unchanged. Existing CI uses --locked equivalents; a hosted
CI run is unverified. [tasks.md](tasks.md) owns detailed verification evidence.

## Scenario walkthrough

Exercise the public contract through automated tests; no console demo or Discord command is needed.

1. Construct VND 1000 and 250; assert exact addition 1250. Reject fractional/currency/malformed
   cases and inspect payload-free errors. MAX and MAX+1 text must follow the documented bounds.
2. Preview a decrease: 1000 → 750. Verify original input unchanged, exact depletion accepted,
   overdraft rejected, zero change rejected, and MAX+1 arithmetic does not wrap/saturate.
3. Construct typed nonblank owner/target/origin/key and manual-confirmation intent. None prior
   yields Eligible without posting anything. Reuse that candidate with current zero: Replay,
   preserving prior provenance/result rather than recalculating balance.
4. Change each semantic field, including basis. Same owner/key yields conflict; owner/key mismatch
   yields context mismatch. No error contains either owner's identifiers or monetary payload.
5. Handoff-only new intent fails as HandoffNotCompletion. No manual case or virtual balance can
   be reported as BankVerified. Verify private candidate fields prevent unchecked construction.

[The contract](contracts/financial-core.md) defines exact API/error semantics and maps all 13 AC.
[The model](data-model.md) defines fields, ownership, and supplied-prior limitations. Use only
synthetic reference markers and amounts; do not test with personal records or credentials.

## Review and final evidence

Inspect enabled dependencies/imports and visibility. Domain has std/thiserror only; application
uses its own domain and has no clock/env/state/provider dependency. Future callers use the feature
facade, not internal paths. Existing bootstrap behavior, CI commands, and user code edits remain
preserved. Verify sensitive nested Debug/pretty Debug/error chains reveal no synthetic markers.

Inspect immutable before/proposed values and replay identity, not merely enum names or call counts.
A prior candidate is never a durable receipt; two None calls may both be Eligible. There must be
no journal, mutable registry, transaction, persistence, bank verifier, allocation, scheduler,
QR/OCR/AI, payment action or deployment added as part of this feature.

Actual commands/results are recorded in [tasks.md](tasks.md) and
[SESSION_HANDOFF.md](../../docs/ai/SESSION_HANDOFF.md). Six external compiler probes also rejected
private candidate/value construction, private module imports and a nonexistent BankVerified
variant. ADR 0003 is accepted. These checks do not establish production security, authenticated
ownership or durable/concurrent exactly-once commits.
