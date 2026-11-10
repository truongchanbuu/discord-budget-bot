# Implementation Plan: Phase 0 Project Foundation

**Branch**: `001-project-foundation` | **Date**: 2026-11-10 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/001-project-foundation/spec.md`.

**Status**: All Phase 0 tasks are complete and locally verified; hosted CI execution is unverified. Constitution 1.1.0 applies. Earlier T001–T006-only authorization is superseded by
the maintainer's instruction to finish the approved Phase 0 tasks. Business features remain out of scope.

## Summary

Build a single Rust package with a library and executable. The executable starts a minimal Tokio
runtime, validates one non-sensitive logging setting, installs structured tracing, reports
initialization, and exits. No financial behavior, network clients, database connections, scheduler,
or Discord interaction is included.

The architecture is a Feature-First Modular Monolith with inward-facing dependencies. Only four
source files are needed now. Documentation defines feature ownership, integration boundaries,
financial constraints, a preliminary threat model, coding standards, and session continuity.
Selected dependencies and rejected alternatives are recorded in [research.md](research.md).

## Technical Context

**Language/Version**: Rust stable release 1.99.1, Edition 2024; pin the compiler in
`rust-toolchain.toml` using the minimal profile plus rustfmt and Clippy. T003 installed and verified
this project-pinned compiler. The earlier 1.98.0 compiler observation is historical; no compiler
change is required by this planning review.

**Primary Dependencies**: Tokio 1.54.1 (`macros`, `rt`), tracing 0.1.44 (`std`),
tracing-subscriber 0.3.23 (`fmt`, `std`), thiserror 2.0.21. Disable default features for the first
three. Use compatible Cargo version requirements and commit the resolved lockfile. Poise/Serenity,
SQLx/PostgreSQL, and Serde remain future selections, not Phase 0 dependencies.

**Storage**: None; no business persistence, migrations, file-backed settings, or secret storage.

**Testing**: `cargo test`; pure configuration unit tests and subprocess integration tests of the
actual executable. No new test dependencies are necessary; use the standard library.

**Target Platform**: Local developer machines supported by stable Rust; Linux is the initial CI
verification platform. Do not claim unexecuted operating-system compatibility checks.

**Project Type**: One Cargo package named `fundguard`, library plus executable; no workspace or API.

**Performance Goals**: No financial throughput or service availability target. Initialization
terminates without external I/O or waiting for input; avoid speculative performance infrastructure.

**Constraints**: Tests and runtime need no credentials or running services. Initial installation
and crate downloads may use the network. No unsafe code, live clients, implicit .env loading,
financial behavior, or deployment.

**Scale/Scope**: One local process, initially single-owner product assumptions; framework-independent
business modules can support additional users and interfaces when their own features are specified.

## Constitution Check

*Initial research used constitution 1.0.0; the post-remediation design check uses constitution 1.1.0.*

| Principle / constraint | Before research | After design | Evidence / required action |
| --- | --- | --- | --- |
| Human-readable, idiomatic Rust | Pass | Pass | Four cohesive modules, typed errors, no speculative traits or service hierarchy |
| Feature-first boundaries | Pass | Pass | Inward dependency matrix below; no empty feature implementations |
| Financial integrity | Pass | Pass | No financial source; all future invariants retained in spec and planned governance/docs |
| Minimal dependencies and replaceable integrations | Pass | Pass | Four used crates; adapters deferred; future reuse documented |
| Verified delivery and persistent context | Pass | Pass for design | Verification matrix and authoritative artifact links; implementation results remain unverified |
| Safe technology/security baseline | Pass | Pass for design | Credential-free runtime, redacted config, no network stack, minimal CI permissions |
| Review gates and generated tooling | Pass | Pass | Planning explicitly requested; scripts/templates/skills unchanged; T001–T006 explicitly authorized |

No constitutional exception is requested. Constitution 1.1.0 now explicitly includes FR-006's
security additions. Analysis C1 is resolved by requiring a scoped fallback tracing subscriber
for startup failures; no plain-text production diagnostic exception is introduced. Design review
confirms all existing invariants and least-privilege rules remain intact. All Phase 0 tasks are authorized by the latest instruction; no future business feature is included.

## Project Structure

### Documentation (this feature)

```text
specs/001-project-foundation/
├── spec.md
├── checklists/requirements.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/runtime.md
```

[tasks.md](tasks.md) is the generated authoritative checklist. All 36 tasks are verified; final evidence is
recorded there. No substitute checklist is maintained here or in
the handoff. Readiness of the remaining tasks does not authorize their implementation.

### Source Code (repository root)

The tree below is implemented. Runtime compilation/tests and an isolated clean-copy walkthrough
have passed; final documentation and artifact review evidence is recorded in tasks.md:

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml
rustfmt.toml
.gitignore
.env.example
.github/workflows/ci.yml
src/
├── main.rs
├── lib.rs
├── bootstrap.rs
└── config.rs
tests/
├── bootstrap.rs
└── support/mod.rs
```

**Structure Decision**: Keep configuration and lifecycle at the root until integrations need
platform modules. `main.rs` owns the Tokio runtime and process exit. `lib.rs` declares private
modules and exposes only the bootstrap entrypoint and its error type. `bootstrap.rs` owns startup
sequence and tracing setup; `config.rs` owns environment reading and pure validation. Tests of
private parsing live alongside configuration; integration tests exercise the executable.
No empty `features`, `platform`, or `shared` directories are created.

The project-structure guide will document this future layout, explicitly labeled unimplemented:

```text
src/features/{identity,funds,income,payments,ledger,reports,reminders}/
    domain/          # only when business behavior exists
    application/     # only when use cases exist
    infrastructure/  # only when an adapter is required
    presentation/    # only when interactions exist
src/platform/{discord,database,scheduling}/
src/shared/          # only cohesive concepts genuinely shared by features
```

This is a map of intended ownership, not a directory creation checklist.

### Required project documents

Deliver every path listed by FR-008. README is the practical entrypoint and links to dedicated
product, architecture, development, security, and session documents. Feature requirements remain
in Spec Kit specifications. `FEATURE_INDEX.md` points to the authoritative active feature and
future Financial Core work. `SESSION_HANDOFF.md` links to the authoritative tasks rather than duplicating
them. Important technical choices become ADRs during implementation:

- `docs/architecture/decisions/0001-feature-first-modular-monolith.md`: one package, future
  boundaries, framework replacement, and triggers for revisiting packaging.
- `docs/architecture/decisions/0002-offline-foundation-bootstrap.md`: minimal dependencies,
  configuration, logging, and deferred integrations.

Both ADRs are accepted and delivered; source and the shared cfg(test)-only child harness are implemented.

## Architecture and Dependency Direction

| Area | Responsibility | May depend on | Must not depend on |
| --- | --- | --- | --- |
| Domain | Entities, value objects, invariants, domain errors | Standard library, justified domain-only libraries | Application, adapters, Discord, SQLx, HTTP, AI |
| Application | Use cases and workflow; genuinely required ports | Own domain; explicitly approved application APIs | Concrete providers, Discord/SQLx types, adapter implementations |
| Infrastructure | Implement ports, persistence, external integration | Own application/domain; external libraries | Presentation or unrelated feature internals |
| Presentation | Validate/map transport input and responses | Own application APIs, Discord transport types | Direct financial mutation/persistence or other feature internals |
| Platform | Runtime clients, pools, scheduler lifecycle | Integration libraries, neutral configuration | Financial rules or feature orchestration |
| Composition root | Construct adapters and wire use cases; lifecycle | All concrete components it composes | Business policy embedded in startup |
| Shared | Small cohesive domain-neutral concepts | Standard library or explicitly justified pure dependencies | Feature internals or framework-specific convenience APIs |

Future Discord commands live with their owning feature's presentation. `platform/discord`
initializes clients and registers commands supplied by the composition root, keeping feature
orchestration out of platform lifecycle code; Serenity contexts and identifiers do not leak into
application/domain APIs. A second Discord application or Web/mobile adapter reuses application
use cases through explicit input/output types. It may use another composition root later; no
HTTP API or duplicate bot is required now. A shared package enforces logical boundaries through
privacy and review; this plan does not claim Cargo mechanically prevents every sibling-module
import. Revisit crate separation only when actual boundary enforcement or release needs justify it.

Scheduling and database lifecycle remain platform concerns; reminder policy and transactions
belong to their owning features. Cross-feature orchestration uses declared application contracts;
review the dependency graph before introducing a new edge. Do not create a generic event bus,
repository framework, dependency-injection container, or provider trait without a concrete use case.

## Bootstrap and Configuration Design

The exact external behavior is defined in [contracts/runtime.md](contracts/runtime.md).
Read only `FUNDGUARD_LOG_LEVEL`. Default to `info`;
accept lowercase `trace`, `debug`, `info`, `warn`, `error`. Reject empty, malformed, whitespace-padded,
uppercase, or non-Unicode values with an error naming the setting without storing or echoing its
raw value. Use a typed tracing level internally. No generic configuration framework is needed.

The startup sequence is: enter Tokio's current-thread runtime; load and validate configuration;
install a fallible tracing subscriber; emit `event = "application_initialized"` at the configured
level threshold; return and exit. The initialization event is an INFO event and may be intentionally
suppressed by `warn` or `error`; default startup always records it. No async service trait or fake
async I/O is introduced just to populate the runtime.

Use `thiserror` for configuration/startup errors and `Result`/`?` for propagation. Raw environment
values, secrets, and arbitrary subscriber error text must not enter either Display or Debug
representations. On startup failure, the executable uses a minimal scoped fallback tracing subscriber at ERROR
level to emit a safe structured error event to stderr, regardless of invalid configured level or
global subscriber installation failure. The scoped subscriber does not install/replace the global
subscriber and retains no raw settings/error strings. Emit an ERROR event with stable field
`event = "application_initialization_failed"`; validation failures also name FUNDGUARD_LOG_LEVEL
and its accepted values through safe typed error formatting. Test this observable fallback path
with invalid configuration, not merely the exit code or plain message text. All production
diagnostics use tracing. Treat subscriber installation
failure as startup failure, without panicking. Tests that initialize global tracing run in child
processes or use local scoped subscribers so parallel tests do not compete for global state.

Keep `bootstrap` and `config` modules private. Expose a synchronous startup function returning a
typed startup error to the executable, invoked inside `#[tokio::main(flavor = "current_thread")]`.
Configuration stays internal; pure parsing accepts supplied values so unit tests do not mutate
global process environment. No dependency injection abstraction is needed for this boundary.

## Security and Documentation Design

The security baseline will distinguish future assets (tokens, owner bindings, financial records,
audit trails) from Phase 0 assets (source, dependency resolution, local configuration, CI trust).
Its threat table covers token leakage, malicious Discord input/attachments, wrong-owner access,
forged/replayed confirmations, sensitive diagnostics, and dependency/CI compromise. For each,
record trust boundary, mitigation, current implementation state, and future owner. No actual
identity or payment control is represented as implemented during this phase.

Document future fail-closed identity binding and authorization before use-case execution;
personal-use does not imply all senders are trusted. Do not invent an owner ID configuration
setting before the identity feature. Bank passwords, OTPs, PINs, CVVs, and bank-session credentials
must never be collected or stored; no direct real-money transfers in the MVP. Preserve every
financial invariant from the spec, including origin, auditability, atomic idempotency, and the
separate meanings of virtual balances, handoff, manual confirmation, and bank verification.

`.env.example` contains only the supported non-secret log level. `.gitignore` excludes target,
real .env variants, logs, and editor/OS debris while allowing `.env.example` and Cargo.lock.
Never include example token-shaped strings or personal records. No secret manager, security
scanner, policy engine, or bank provider is added speculatively.

The coding guide covers every FR-005 topic with practical acceptable/unacceptable examples,
including borrowed inputs versus unnecessary clones, guards versus nesting, explicit errors,
private APIs, and meaningful tests. Future value-object/transaction examples are clearly
illustrative, not implementations: no floats for money, validate at construction, and commit
financial mutation plus idempotency/audit state atomically when that feature is specified.

## Tooling and CI Design

Use Cargo manifest lint configuration to forbid unsafe code and warn on Clippy `all`; CI promotes
warnings to errors. Use stable rustfmt with Edition 2024 and default idiomatic formatting; avoid
blanket pedantic restrictions and arbitrary function length thresholds. No separate Clippy config
file is required unless a justified non-default option exists.

CI runs on pull requests and pushes, with `contents: read`, no service containers, deployment,
secrets, or privileged pull-request target events. Use official checkout pinned to a full commit
SHA resolved from a maintained release at implementation time. Install/select the same pinned
compiler and components as local development using rustup. Run all four required commands, with
`--locked` for Cargo dependency-resolving CI steps; formatting uses the exact requested command.
Run executable integration tests through cargo test. No cache action or wrapper command tool is
needed initially. Set a finite job timeout, disable checkout credential persistence, and keep
unnecessary token permissions absent.

## Verification Strategy and Acceptance Coverage

| Scope | Method | Requirements / scenarios |
| --- | --- | --- |
| Package/toolchain and required checks | Four local commands; review CI equivalents | FR-007, FR-011, FR-012; AC-01, AC-04, AC-13; SC-002 |
| Defaults and all supported levels | Pure table-based parser tests | FR-001, FR-002; AC-10, AC-11; SC-001 |
| Malformed, empty, non-Unicode values | Parser tests, Unix-specific byte case where supported | FR-002; AC-11; SC-001 |
| Runtime success and redaction | Subprocess tests of exit, stderr/stdout and marker presence/absence | FR-001, FR-002; AC-10–AC-12; SC-001 |
| Boundary and visibility review | Inspect modules, imports, dependency tree, documented future adapter example | FR-003, FR-004, FR-013; AC-02, AC-08, AC-09; SC-005, SC-007 |
| Standards and security review | Topic coverage, preliminary threat table, no-secret diff review | FR-005, FR-006, FR-014; AC-03, AC-07, AC-14; SC-006 |
| Onboarding and continuity | Follow README/AGENTS links and fresh-session discovery path | FR-008, FR-009; AC-05, AC-06; SC-004 |
| Delivery and roadmap | Verify authoritative spec links, planned vs actual state and Financial Core sequencing | FR-010, FR-015; AC-15; SC-003, SC-004 |

Subprocess tests clear their child environment, then set only needed synthetic values. Every
child, including subscriber-collision tests, has a 10-second test-only deadline. Use bounded
polling with std::process::Child::try_wait; on timeout kill and reap the child and report failure.
Capture/drain output without allowing pipe capacity to block the bounded wait. This deadline is
a test harness safeguard, not a production startup latency requirement. Do not use
unsafe process-global environment mutation under Edition 2024. Test invalid-input and ambient
credential markers are absent from both output streams. Tests require no real credentials or
network service. Inspect enabled Cargo features for accidental networking and review bootstrap
source; do not claim a substring test alone proves absence of network calls.

After implementation, run exactly the four requested commands, plus the documented startup
success/failure scenarios. Inspect lockfile changes, `cargo tree -e features`, module visibility,
all required document links, security constraints, and `git diff --check`/repository diff. A hosted
CI run is additional evidence only when one actually exists. US1's T013 checkpoint exercises only runtime/build sections of quickstart.md; the architecture,
README/AGENTS onboarding, security, and hosted-CI reviews depend on later deliverables and belong
to T027/T032–T035. Record results in tasks and handoff;
passing tests do not establish future financial correctness or production security.

## Delivery Sequence and Review Boundaries

1. Adopt constitutional security wording through its own workflow (completed in T002); honor the
   latest instruction approving completion of all Phase 0 tasks.
2. Use the generated story-based tasks with requirement/acceptance coverage; preserve verified completion
   evidence and implement only the approved Phase 0 scope.
3. Run cross-artifact analysis; resolve critical/high issues before implementation.
4. Implement the smallest runnable package and tests, followed by documentation and CI as scoped.
5. Run verification, inspect diff and dependency direction, update authoritative tasks and handoff.

This is a dependency sequence, not a replacement implementation checklist. Current delivery includes meaningful runtime source, tests, documentation, and CI configuration.
Future financial features and deployment remain excluded. Actual task status and evidence live
in tasks.md, not in a duplicate checklist here.

## Complexity Tracking

No constitution violations or complexity exceptions. One package, four source files, four direct
runtime dependencies, and standard-library tests are sufficient. Financial domain models, provider
ports, web APIs, scheduling, and live integrations are deliberately absent until needed.
