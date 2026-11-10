# Project structure

## Implemented files

| Path | Responsibility |
| --- | --- |
| src/main.rs | Tokio runtime, safe structured failure reporting, process exit |
| src/lib.rs | Private module declarations and narrow startup/error exports |
| src/bootstrap.rs | Configuration and diagnostics initialization; isolated collision test |
| src/config.rs | Pure log-level parsing, environment read, colocated unit tests |
| tests/bootstrap.rs | Executable contract tests under controlled child environments |
| tests/support/mod.rs | Shared bounded child-process harness with drained output pipes |
| Cargo.toml / Cargo.lock | One package, minimal declared dependencies and resolved versions |
| rust-toolchain.toml / rustfmt.toml | Pinned compiler/components and stable formatting |
| .github/workflows/ci.yml | Credential-free checks; no deployment |

The test harness is included only under cfg(test) in bootstrap, and as an integration-test module.
It creates no runtime abstraction or additional public API. Test expect/unwrap calls represent
failed test setup/assertions, never runtime-critical error handling.

## Intended future ownership — no placeholder directories

Features are identity, funds, income, payments, ledger, reports, and reminders. When behavior
exists, keep it under src/features/<feature> with domain, application, infrastructure, and
presentation layers only as justified by that feature. Domain models and rules stay together;
provider-independent use cases coordinate them; adapters implement actual dependencies.

Future src/platform/discord, database, and scheduling modules own shared integration lifecycle.
Create src/shared only for cohesive concepts actually used by multiple features. Do not create
empty directories, mod files, unused traits, or global repositories/services/handlers/models trees.

The [architecture guide](../architecture/overview.md) defines dependency direction. The
[feature map](../product/feature-map.md) describes intended ownership. Feature requirements and
implementation progress belong to [Spec Kit](../../specs/001-project-foundation/tasks.md).
