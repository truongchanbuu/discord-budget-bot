# Foundation Research and Decisions

**Checked**: 2026-11-10. Research resolves Phase 0 design choices only. No dependencies were added,
compiler installed, lockfile resolved, or application compiled during planning.

## 1. One Package and Meaningful Modules

**Decision**: One `fundguard` Cargo package with library and executable; initially main, lib,
bootstrap, and config source files. Future feature-first and platform layouts are documented only.

**Rationale**: This is sufficient for process initialization, testable parsing, and a reusable
startup boundary. Feature-specific layers should appear with actual behavior. Rust visibility and
review provide the initial boundaries; a single package does not enforce every architectural rule.

**Alternatives considered**: Multi-crate workspace (release/boundary complexity without current
need); global repositories/services/models layers (weak feature ownership); empty feature skeletons
(no behavior and explicitly forbidden). The user's refined spec resolves this choice.

## 2. Toolchain and Version Evidence

**Decision**: Pin Rust 1.99.1, Edition 2024, minimal rustup profile plus rustfmt/Clippy in
rust-toolchain.toml (now delivered by T003). Use the same toolchain locally and in CI; upgrades are explicit reviewed changes.

**Rationale**: The official stable channel currently reports 1.99.1, channel date 2026-10-15.
Edition 2024 requires at least 1.85. At planning time installed rustc/cargo were 1.98.0; the stable alias was not proof of the latest
release. T003 subsequently installed 1.99.1 and project configuration selects it; the global
default alias remains unchanged.

**Alternatives considered**: Floating stable (local/CI drift); pinning the installed older release
(unnecessary mismatch with the current stable baseline); nightly (not required).

Sources: [official stable manifest](https://static.rust-lang.org/dist/channel-rust-stable.toml),
[Edition 2024 guide](https://doc.rust-lang.org/edition-guide/rust-2024/index.html),
[rustup toolchain configuration](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file).

## 3. Minimal Bootstrap Dependencies

**Decision**: Declare only the following dependencies, now configured by T004. They are now
resolved in Cargo.lock and compiled by the completed US1 source tasks; actual features were reviewed.

| Crate | Verified stable, non-yanked release | Declared minimum Rust | Enabled features |
| --- | --- | --- | --- |
| Tokio | 1.54.1 | 1.85 | defaults off; macros, rt |
| tracing | 0.1.44 | 1.65 | defaults off; std |
| tracing-subscriber | 0.3.23 | 1.65 | defaults off; fmt, std |
| thiserror | 2.0.21 | 1.77 | normal defaults |

**Rationale**: Tokio hosts the entrypoint runtime, tracing/subscriber implement actual diagnostics,
and thiserror represents initialization/configuration failures. The current-thread Tokio macro
needs macros and rt; default multithread flavor would require rt-multi-thread. No network, timer,
signal, filesystem, tracing attribute, JSON, env-filter, or ANSI feature is needed. A simple typed
log level avoids a filter language and its extra dependencies. fmt still has transitive registry
support; this is not a claim that transitive dependencies disappear.

Use ordinary compatible version requirements starting at these verified versions and commit
Cargo.lock. Review the actual resolved graph and license/security implications when generated.
Direct MSRV checks are preliminary compatibility evidence, not proof of lockfile compilation.

**Alternatives considered**: Tokio full (unused capabilities); env-filter (unnecessary filtering
language); anyhow/config/dotenv/test-helper crates (standard library or typed errors suffice);
no Tokio (would omit the selected asynchronous execution foundation).

Primary sources: [Tokio metadata](https://crates.io/api/v1/crates/tokio/1.54.1),
[Tokio runtime macro](https://docs.rs/tokio/1.54.1/tokio/attr.main.html),
[tracing metadata](https://crates.io/api/v1/crates/tracing/0.1.44),
[subscriber metadata](https://crates.io/api/v1/crates/tracing-subscriber/0.3.23),
[thiserror metadata](https://crates.io/api/v1/crates/thiserror/2.0.21).

Research retrieved official registry JSON directly over HTTPS after browser retrieval could not
read those API resources. It did not substitute a Cargo mirror for primary release evidence.

## 4. Future Integration Compatibility — No Phase 0 Installation

**Decision**: Keep Poise/Serenity, PostgreSQL/SQLx, and Serde as selected future technologies.
Reverify releases and needed features in the first owning integration specification.

| Technology | Verified current stable | Declared minimum Rust | Current Phase 0 use |
| --- | --- | --- | --- |
| Poise | 0.7.1 | 1.82 | None |
| Serenity | 0.12.5 | 1.74 | None |
| SQLx | 0.9.0 | 1.94 | None |
| Serde | 1.0.229 | 1.56 | None |
| PostgreSQL | 18.6 (stable major 18) | Not applicable | None |

Poise 0.7.1's normal requirements include Serenity ^0.12.5, Tokio ^1.25.1, and tracing ^0.1.40;
the researched versions satisfy those declared ranges. Poise's Tokio rt-multi-thread feature
entry in registry dependencies is dev-only; do not infer a current production requirement from it.
Future client runtime/TLS/intent features require their own implementation validation. SQLx needs
an explicitly chosen runtime, PostgreSQL driver, and TLS policy when persistence is implemented.
No server version, migration tooling, or offline SQL metadata is required now. Major 18 support
currently ends 2030-11-14; select a supported server release at actual integration time.

**Rationale**: Unused frameworks add compile and security surface without proving any boundary.

**Alternatives considered**: Installing the entire preferred stack (unused); fake providers and
placeholder repository traits (no present use case).

Sources: [Poise metadata](https://crates.io/api/v1/crates/poise/0.7.1),
[Poise dependency metadata](https://crates.io/api/v1/crates/poise/0.7.1/dependencies),
[Serenity metadata](https://crates.io/api/v1/crates/serenity/0.12.5),
[SQLx metadata](https://crates.io/api/v1/crates/sqlx/0.9.0),
[SQLx runtime/TLS documentation](https://docs.rs/sqlx/0.9.0/sqlx/),
[Serde metadata](https://crates.io/api/v1/crates/serde/1.0.229),
[PostgreSQL support policy](https://www.postgresql.org/support/versioning/).

## 5. Runtime Configuration and Safe Diagnostics

**Decision**: Support only FUNDGUARD_LOG_LEVEL, default info, with exact lowercase
trace/debug/info/warn/error values. Reject empty, padded, uppercase, malformed, or non-Unicode
input. Store typed levels and safe error categories, never raw invalid values in errors.
Structured initialization events go to stderr; stdout remains empty. Startup failures use a minimal scoped ERROR tracing subscriber on stderr if normal subscriber
setup failed, with stable field event=application_initialization_failed at ERROR. Executable
negative-path tests assert that event as well as redaction and exit status; no plain-text
production diagnostic fallback is permitted. Logging does not imply persistence or readiness
of external services. See [runtime contract](contracts/runtime.md).

**Rationale**: This provides a real validation/error boundary without secret configuration or
implicit connections. Log-level filtering has predictable behavior, including deliberate INFO
suppression for warn/error. Environment examples can remain entirely non-sensitive.

**Alternatives considered**: RUST_LOG/env-filter (wider syntax without need); .env autoloading
(extra dependency/implicit behavior); requiring bot tokens or database URLs (contradicts offline
foundation); logging at elevated severity simply to bypass configured thresholds (misleading).

## 6. Testing and CI

**Decision**: Pure parser unit tests and std::process executable tests; no test dependencies.
Children receive controlled environment variables. Never mutate parent process environment in
parallel tests. Use the pinned compiler, read-only GitHub token, official SHA-pinned checkout,
and direct Cargo commands. Resolve the checkout release SHA when writing the workflow, as a
supply-chain pin rather than a new architecture decision. Do not add caching or service containers.

**Rationale**: Child processes isolate environment and global subscribers. Edition 2024 makes
process-global environment mutation unsafe; pure inputs avoid it. Observable exit/output tests
provide stronger evidence than tests mirroring implementation syntax.

**Alternatives considered**: Global env mutation and serialized test framework (avoidable coupling);
real integration accounts (forbidden); expansive task runners, cache actions, or secret scanners
(no demonstrated Phase 0 requirement).

Sources: [Edition guide on newly unsafe functions](https://doc.rust-lang.org/edition-guide/rust-2024/newly-unsafe-functions.html),
[Cargo lint configuration](https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section),
[GitHub token permissions](https://docs.github.com/en/actions/security-for-github-actions/security-guides/automatic-token-authentication),
[GitHub security hardening](https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions).

## 7. Adopted Constitution Alignment and Deferred Product Decisions

**Decision**: Adopted constitution 1.1.0 through the constitution workflow under the user's
explicit T001–T006 implementation authorization. Security principles prohibit banking passwords,
OTPs, card PINs, CVVs, banking-session credentials, and direct real-money transfers in the MVP;
they require input validation, identity/authorization boundaries, token protection, and awareness
before handoff. All origin/auditability/idempotency/no-float/privacy rules are retained.

**Rationale**: FR-006 required explicit wording, now incorporated. This is an additive MINOR
amendment with no runtime/data migration because no source or financial data existed at amendment time. Updated
artifacts: constitution, spec/plan consistency notes, runtime contract, tasks, and handoff.

**Alternatives considered**: Weakening the existing structured-tracing rule was rejected.
Startup failure now uses a scoped fallback tracing subscriber, preserving the rule rather than
adopting the previous plain-text exception. The initial approval covered Setup and Foundational phases; the latest maintainer instruction
authorizes all Phase 0 tasks. No business-feature implementation or deployment is included.

Identity binding, currency/amount representation, multi-user isolation, financial transactions,
retention, and production operations remain future feature decisions. Their deferral is scoped,
not an unresolved Phase 0 architecture question. Financial Core must precede payment features.
All technical choices needed for the foundation design are resolved. Constitution adoption and
review authorization now covers all Phase 0 tasks; business features remain deferred.
