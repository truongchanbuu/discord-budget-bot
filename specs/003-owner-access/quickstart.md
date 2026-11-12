# Owner Access Validation Guide

**Status**: Planned validation, not implementation evidence. No identity source or test target exists yet.

## Prerequisites

Use the existing rust-toolchain.toml (Rust 1.99.1 with rustfmt and Clippy) and Cargo.lock.
No new dependency, token, .env entry, provider account, database or feature flag is needed.
Toolchain/dependency installation may require the network; policy tests must not.

## Current baseline inspection

From the repository root, inspect git status before work and preserve existing edits. Read
[spec.md](spec.md), [plan.md](plan.md), [contract](contracts/owner-access.md),
[model](data-model.md), constitution and session handoff. Existing Financial Core verification
is historical; it does not prove this planned identity feature works.

## Verification after implementation

Run from the repository root once the identity test target exists:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --test identity
cargo test
cargo check
```

Expected: new public-contract tests and existing foundation/Financial Core tests pass, zero Clippy
warnings, no services/credentials. Manifest, lockfile and offline startup remain unchanged.
Existing CI's cargo test discovers the new target; hosted CI needs separate actual-run evidence.

## Scenario walkthrough

1. Parse synthetic account 42, build a TrustedActor and an enabled binding to synthetic owner
   owner-test-A through validated constructors. Resolve the exact owner without any lookup.
2. Substitute account 43, missing actor, absent binding and disabled binding independently; verify
   the same Denied result with no bound identity output. Reject every account grammar boundary.
3. Authorize owner-test-A scope; reject owner-test-B and missing scope. A matching submitted owner
   does not rescue an unknown actor. Verify exact case/padding owner equality.
4. After success, evaluate with disabled/absent binding: deny. Change the bound owner: resolve the
   current mapping only. Do not create an account-transfer workflow or persistent cache.
5. Inspect nested/pretty formatting and error sources using synthetic numeric/string markers;
   confirm redaction, unchanged inputs and no content/channel-based authorization bypass.

These are public-library checks, not live Discord end-to-end verification. AC-10 is a review of
the future caller obligations in the contract; AC-12 combines behavior tests and source/API review.

## Completion evidence

During implementation, record actual results in [tasks.md](tasks.md) and summarize in
SESSION_HANDOFF.md. Review imports, private fields, public API, secret exposure and full diff.
Use temporary external rustc probes to confirm invalid construction/privacy boundaries without
new dependencies. Record failed probes as expected compiler rejection, not runtime tests.

Keep accounts and money synthetic. Do not add real auth, binding/environment loading, financial
use cases, provider SDKs, production credentials or deployment to demonstrate policy behavior.
