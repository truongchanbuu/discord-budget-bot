# Owner Access Validation Guide

**Status**: Verified locally on 2026-11-12. Source and the identity test target are implemented;
[tasks.md](tasks.md) owns actual commands/results and acceptance-review evidence.

## Prerequisites

Use the existing rust-toolchain.toml (Rust 1.99.1 with rustfmt and Clippy) and Cargo.lock.
No new dependency, token, .env entry, provider account, database or feature flag is needed.
Toolchain/dependency installation may require the network; policy tests must not.

## Before making further changes

From the repository root, inspect git status before work and preserve existing edits. Read
[spec.md](spec.md), [plan.md](plan.md), [contract](contracts/owner-access.md),
[model](data-model.md), constitution and session handoff. Use current task evidence rather than assuming earlier feature results verify later edits.

## Verification commands

Run from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --test identity
cargo test
cargo check
```

All commands above passed locally: 10 identity tests and all 27 existing tests (37 total), zero
Clippy warnings, no services/credentials. Manifest, lockfile and offline startup remain unchanged.
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

Actual results are in [tasks.md](tasks.md) and SESSION_HANDOFF.md. Source/API/import review found
no out-of-scope workflow. Five temporary external rustc probes rejected raw account/actor
construction, binding field mutation and private module imports. These were expected compiler
rejections, not runtime tests; no compile-test dependency or repository harness was added.

Keep accounts and money synthetic. Do not add real auth, binding/environment loading, financial
use cases, provider SDKs, production credentials or deployment to demonstrate policy behavior.
