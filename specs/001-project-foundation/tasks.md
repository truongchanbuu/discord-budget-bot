---
description: "Implementation tasks for FundGuard Phase 0 Rust Project Foundation"
---

# Tasks: FundGuard Phase 0 — Rust Project Foundation

**Input**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md),
[data-model.md](data-model.md), [runtime contract](contracts/runtime.md), and
[quickstart.md](quickstart.md), plus `.specify/memory/constitution.md`.

**Created**: 2026-11-10. **State**: Phase 0 complete; all 36 tasks verified. Source, tests, lockfile, documentation,
and CI configuration are delivered. Hosted CI execution remains explicitly unverified.

**Authorization**: The latest maintainer instruction authorizes completion of all approved Phase 0
tasks, superseding the initial T001–T006-only limit. Constitution 1.1.0 applies. This authorization
includes no financial feature, live integration, or production deployment.

**Organization**: Setup → blocking shared configuration → US1/P1 → US2/P2 → US3/P3 → final checks.
Tests are mandatory and belong to the behavior they verify. Within runtime tasks, write behavioral
tests before implementing that behavior, observe meaningful failures when runnable, then make them
pass. Do not retain empty stubs or placeholder directories just to create a test harness.

## Format and Execution Rules

Use `- [ ] Tnnn [P?] [USn?] Description`. Paths are relative to repository root. Each task below
identifies dependencies, target files, requirements, and a completion check. `[P]` means file-safe
parallel work **after all stated prerequisites are complete**; it is not permission to run ahead of
a phase gate. Unmarked tasks run sequentially. All dependencies list task IDs; phase gates add any
remaining previous-phase prerequisites. Parallel reads/checks are safe only against a stable tree.

The future implementer must preserve unrelated changes, inspect actual files before editing,
record meaningful verification beside each completed task here, and summarize outcomes in the
handoff. Do not copy this detailed checklist into other documents. Use synthetic redaction markers,
never real secrets. Test failure or missing evidence leaves the task partial/unchecked.

## Phase 1: Setup — Repository and Governance (2 tasks)

**Purpose**: Establish current state and resolve the recorded constitutional gap before code.

- [X] T001 Inspect repository instructions (including any newly added `AGENTS.md`), Git status, `.specify/memory/constitution.md`, and all `specs/001-project-foundation/` inputs; run `$speckit-analyze` on the generated artifacts and reconcile any critical/high finding through the owning workflow before code; record review authorization, preserved unrelated changes, and current findings in `docs/ai/SESSION_HANDOFF.md`. **Depends**: none. **Supports**: FR-010, FR-012, SC-003. **Verify**: feature paths resolve, all artifacts are read, no unresolved critical/high design finding is concealed, and no implementation starts without applicable review approval.
- [X] T002 Apply the additive security governance described in `specs/001-project-foundation/research.md` through `$speckit-constitution` to `.specify/memory/constitution.md`: prohibit collecting/storing banking passwords, OTPs, card PINs, CVVs, and banking-session credentials; prohibit direct real-money transfers in the MVP; make identity/authorization, Discord token protection, and awareness-before-handoff explicit while retaining existing financial invariants. Update the consistency/version references in `specs/001-project-foundation/spec.md` and `specs/001-project-foundation/plan.md`, then obtain/record any remaining plan-review approval in `docs/ai/SESSION_HANDOFF.md`. **Depends**: T001. **Supports**: FR-006, FR-010, AC-14, SC-006. **Verify**: required wording is adopted under constitution governance, dates/version and plan gate agree, temporary sync report is reviewed/removed before commit, generated templates remain unchanged, and the security amendment is not inferred merely from task-generation authorization.

**Checkpoint**: Scope and review authority are recorded; constitution alignment is adopted and
plan approval is resolved. If approval is still pending, record the blocker and stop dependent work.

## Phase 2: Foundational — Shared Project Configuration (4 tasks)

**Purpose**: Prepare real configuration without creating placeholder source or future features.

- [X] T003 Create `rust-toolchain.toml` with stable release `1.99.1`, minimal profile, and rustfmt/Clippy components as selected in the plan; install/select it locally during authorized implementation. **Depends**: T002. **Supports**: FR-007, FR-011, AC-01, AC-04. **Verify**: `rustup show`, `rustc --version`, `cargo fmt --version`, and `cargo clippy --version` resolve the documented compiler/components; if the researched release is unavailable or incompatible, report and reconcile the plan rather than substitute silently.
- [X] T004 Create `Cargo.toml` for one unpublished `fundguard` package, Edition 2024, with library target `src/lib.rs`, no workspace, and ordinary compatible requirements starting at Tokio 1.54.1 (defaults off; macros, rt), tracing 0.1.44 (defaults off; std), tracing-subscriber 0.3.23 (defaults off; fmt, std), and thiserror 2.0.21. Configure Rust unsafe-code prohibition and Clippy all warnings; do not add future integration or test-helper dependencies. Let Cargo discover `src/main.rs` when T009 creates it. **Depends**: T003. **Supports**: FR-007, FR-011, FR-013, AC-01. **Verify**: TOML parses and matches research; actual library compilation/lockfile resolution is verified at T007, executable compilation at T009. Do not add placeholder targets merely to make this configuration-only increment compile.
- [X] T005 [P] Create `rustfmt.toml` with stable Edition 2024 formatting and otherwise conventional defaults; do not add arbitrary function-length policies or an unused Clippy configuration file. **Depends**: T004. **Supports**: FR-005, FR-007, AC-03, AC-04. **Verify**: settings are supported by the pinned rustfmt; confirm formatting behavior on real source at T007/T028.
- [X] T006 [P] Create `.gitignore` excluding `/target/`, real `.env` variants, runtime logs, and relevant editor/OS debris while explicitly allowing `.env.example` and Cargo.lock; preserve generated Spec Kit exclusions. **Depends**: T004. **Supports**: FR-002, FR-006, FR-007, AC-07. **Verify**: `git check-ignore` classifies synthetic target/.env/log paths as ignored and `.env.example`/Cargo.lock as allowed; no secret file or empty directory is created for this check.

**Checkpoint**: Configuration is ready. No claim of compilation is made until meaningful source
exists. Finish all Phase 2 tasks before user-story implementation.

## Phase 3: User Story 1 — Initialize and Verify Safely (P1, 7 tasks)

**Goal**: Credential-free, locally runnable initialization with explicit errors and repeatable tests.

**Independent Test**: From the documented toolchain, run library and executable tests, the four
quality commands, and default/invalid startup cases without Discord, PostgreSQL, or credentials.
T013 establishes this story's gate; T028–T031 repeat the required checks after all changes.

- [X] T007 [US1] Create private configuration behavior and colocated unit tests in `src/config.rs`, with `src/lib.rs` declaring only the meaningful private module; generate `Cargo.lock` from the actual dependencies. Quote the model's validation contract: "Absent → info; exactly trace/debug/info/warn/error" and "Configuration is immutable after construction." Read only FUNDGUARD_LOG_LEVEL, parse optional supplied OS strings into a typed tracing level, reject empty/uppercase/padded/unknown/non-Unicode input, and define thiserror errors that retain no raw value. Write tests first for defaults, every level, invalid categories, and safe Display/Debug; cover Unix non-Unicode bytes with a platform guard. **Depends**: T005, T006. **Supports**: FR-001, FR-002, FR-007, AC-01, AC-11, SC-001. **Verify**: `cargo test --lib` passes without changing global environment; lockfile is generated and `cargo check --lib --locked` compiles the real library; errors name the setting/accepted values without revealing synthetic input markers.
- [X] T008 [US1] Implement `src/bootstrap.rs` and extend `src/lib.rs` to expose only the startup entrypoint and typed startup error while keeping modules/configuration private. Test before implementing fallible diagnostics setup: validated configuration, stderr tracing with no ANSI, INFO event field `event="application_initialized"`, and normal return; subscriber installation failures propagate as safe typed errors. Include an isolated unit test for a preinstalled subscriber collision using a dedicated subprocess of the test executable (filtered child test and a synthetic child-mode marker, with a 10-second test-only deadline, safe output capture, and kill/reap on timeout); do not introduce injection traits or race parallel global subscriber initialization. **Depends**: T007. **Supports**: FR-001, FR-002, FR-003, FR-005, AC-10, AC-11. **Verify**: `cargo test --lib --locked` passes; the isolated collision yields a safe failure without panicking or emitting a success event; Debug/Display do not retain arbitrary subscriber text.
- [X] T009 [US1] Create `src/main.rs` with Tokio current-thread entrypoint invoking the synchronous bootstrap; return status 0 on success and nonzero with a safe structured stderr event through a scoped fallback ERROR tracing subscriber on failure; the fallback is independent of configuration and does not install/replace a global subscriber, and emits the stable event field application_initialization_failed at ERROR. Keep lifecycle wiring here and startup orchestration in `src/bootstrap.rs`; add no CLI, HTTP API, fake async service, live client, database pool, or scheduler. Review/update `Cargo.lock` only through Cargo. **Depends**: T008. **Supports**: FR-001, FR-003, FR-007, AC-01, AC-10, SC-001. **Verify**: `cargo check --locked` and `cargo run --locked` succeed; the default event appears on stderr and application stdout is empty; only four meaningful source files exist.
- [X] T010 [US1] Create executable contract tests in `tests/bootstrap.rs` using standard-library `Command` and Cargo's binary path. Clear each child environment; test absent/default and every accepted log level, zero exit, empty stdout, initialization marker on stderr for trace/debug/info and suppression for warn/error. Avoid complete-line/timestamp assertions and arbitrary sleeps; use a 10-second test-only deadline with bounded try_wait polling, safe output draining, and kill/reap on timeout; prove no input/service wait through bounded subprocess completion and review runtime code for absence of connection calls. **Depends**: T009. **Supports**: FR-001, FR-007, AC-04, AC-10, SC-001. **Verify**: `cargo test --test bootstrap --locked` passes without credentials or services; no new test crate or unsafe parent-environment mutation is introduced.
- [X] T011 [US1] Extend `tests/bootstrap.rs` with invalid/empty/uppercase/padded values, platform-guarded non-Unicode runtime input, and synthetic ambient Discord-token/database-URL markers. Assert failure status, the structured ERROR event with stable field application_initialization_failed, setting name/allowed-values diagnostic, empty application stdout, and no success event after failure, no invalid/credential marker in either stream, and no activation from ambient credentials on successful startup. Confirm library error redaction cases in `src/config.rs`/`src/bootstrap.rs` remain covered. **Depends**: T010. **Supports**: FR-002, FR-006, FR-007, AC-07, AC-11, AC-12, SC-001, SC-006. **Verify**: targeted and full `cargo test --locked` pass; tests never use real secrets or mutate the parent environment and distinguish Cargo build output from executable output.
- [X] T012 [US1] Create `.env.example` containing only optional `FUNDGUARD_LOG_LEVEL=info` and explain process-variable use, accepted values, safe default, and absence of implicit .env loading; document no token, owner, or database setting that the executable does not support. **Depends**: T011. **Supports**: FR-002, FR-007, AC-05, AC-07. **Verify**: example agrees with parser/runtime contract, contains no token-shaped samples, is not ignored, and supplying its supported variable changes logging as documented.
- [X] T013 [US1] Validate the runnable US1 increment against `specs/001-project-foundation/contracts/runtime.md` and only the Build and Verify, Run the Foundation, and Validate Failures and Redaction sections of `specs/001-project-foundation/quickstart.md` (README/AGENTS/architecture/security/hosted-CI reviews wait for T027/T032–T035); run all four required Cargo checks, default/filtered/invalid startup scenarios, and inspect `cargo tree -e features`. Record results beside this task and summarize in `docs/ai/SESSION_HANDOFF.md` without claiming full Phase 0 completion. **Depends**: T012. **Supports**: FR-001, FR-002, FR-007, FR-011, FR-012, AC-01, AC-04, AC-10–AC-12, SC-001, SC-002. **Verify**: package compiles, tests pass, errors are redacted and use the structured fallback event, no service/network feature is enabled unnecessarily, and runtime has no external call sites; distinguish crate downloads from runtime activity.

**Checkpoint**: US1 is a useful runnable increment. A working executable alone does not complete
Phase 0: architecture, security, continuity, CI, and final evidence remain mandatory.

## Phase 4: User Story 2 — Extend Features Within Explicit Boundaries (P2, 6 tasks)

**Goal**: Document boundaries and human-readable Rust practices without implementing future layers.

**Independent Test**: Place a hypothetical future use case and Discord/Web/mobile presentation
using the guides; inspect the actual four-module package for narrow visibility and no business code.
No future-domain or framework-specific tests are introduced for unimplemented behavior.

- [X] T014 [P] [US2] Create `docs/architecture/overview.md` from the plan's responsibility/dependency matrix: pure domain, provider-independent application, adapter infrastructure, transport presentation, platform lifecycle, composition root, and cohesive shared concepts. Define explicit cross-feature application calls and no cycles; explain logical/privacy review limits within one package. Include the Poise/Serenity-compatible Discord boundary, minimal justified intents, interaction registration versus feature presentation, future Web/mobile/multiple-bot reuse, and explicit design coverage for slash commands, DMs, pattern input, buttons, selects, modals, image/attachment processing, scheduled reminders, report images, and payment awareness. Keep all capabilities documentation-only with no Phase 0 HTTP/bot implementation. **Depends**: T013. **Supports**: FR-003, FR-004, FR-013, AC-02, AC-09, SC-007. **Verify**: a hypothetical second adapter reuses the same use case without Serenity/SQLx types crossing into domain/application; every dependency rule agrees with plan.md.
- [X] T015 [P] [US2] Create `docs/development/project-structure.md` distinguishing actual main/lib/bootstrap/config responsibilities from intended `src/features/{identity,funds,income,payments,ledger,reports,reminders}/` layers and `src/platform/{discord,database,scheduling}/`/`src/shared/` ownership. Explain when modules become justified; do not create these directories or placeholder files. **Depends**: T013. **Supports**: FR-003, FR-008, FR-013, AC-02, AC-08. **Verify**: documented current files exist, future paths are explicitly planned, and a new contributor can locate a future feature without global technical-layer folders.
- [X] T016 [P] [US2] Create `docs/development/coding-standards.md` covering every FR-005 topic: naming, cohesive responsibilities/SRP at function/module/feature level, guards/control flow, commands/queries where useful, Result/thiserror, safe errors, ownership/borrowing, minimal clones/mutability, private APIs, explicit side effects, async/runtime boundaries, meaningful traits/generics/injection, domain value objects, future atomic transactions, structured logging, comments explaining why, and unit/integration organization. Include practical good/bad Rust examples and reject arbitrary length limits, tiny wrappers, unclear boolean parameters, speculative patterns, and unjustified unwrap/expect. **Depends**: T013. **Supports**: FR-005, FR-008, AC-03. **Verify**: examples are readable and internally valid, production examples fit actual dependencies, future money/transaction examples are explicitly illustrative with no floats, and no example is presented as delivered financial functionality.
- [X] T017 [P] [US2] Create `docs/architecture/decisions/0001-feature-first-modular-monolith.md` recording context, one-package/feature-first decision, inward dependencies, alternatives, consequences, and concrete triggers to reconsider crate separation; distinguish accepted scope from future adapters and avoid promising compiler enforcement absent from the package. **Depends**: T013. **Supports**: FR-003, FR-010, FR-013, AC-02, AC-09. **Verify**: ADR matches current source and plan, captures a meaningful decision, and requires no unused traits or feature skeletons.
- [X] T018 [P] [US2] Create `docs/architecture/decisions/0002-offline-foundation-bootstrap.md` recording initialization-and-exit lifecycle, four used dependencies and enabled features, typed safe log configuration, future integration deferral, test isolation, and selected toolchain; link researched evidence and runtime contract rather than duplicate authoritative requirements. **Depends**: T013. **Supports**: FR-001, FR-010, FR-011, AC-08, AC-10. **Verify**: dependency graph/runtime behavior and lockfile agree with the ADR and no future Poise/SQLx/Serde installation is claimed.
- [X] T019 [US2] Review `src/`, `Cargo.toml`, `docs/architecture/overview.md`, both ADRs, `docs/development/project-structure.md`, and `docs/development/coding-standards.md` against the US2 scenarios; correct discrepancies in their owning files. **Depends**: T014, T015, T016, T017, T018. **Supports**: FR-003, FR-004, FR-005, FR-013, AC-02, AC-03, AC-08, AC-09, SC-005, SC-007. **Verify**: only intended startup/error exports, no circular or uncontrolled imports, no framework-bound business rules, no empty directories/stubs, and no financial operations; record review evidence beside this task.

**Checkpoint**: Feature, platform, shared, composition, and Discord boundaries are actionable design
contracts, not speculative source modules. US2 review can be performed without Discord credentials.

## Phase 5: User Story 3 — Resume Development and Review Security Consistently (P3, 8 tasks)

**Goal**: Accurate onboarding, security guidance, CI, and session state discoverable without chat.

**Independent Test**: Follow AGENTS/README links in a fresh context to identify active scope,
architecture, tasks, verification, risks, and next action; inspect credential-free CI and security.

- [X] T020 [P] [US3] Create `docs/product/overview.md` describing personal-use/single-owner starting scope, Discord-first financial management/payment awareness vision, future multi-user/interface possibility, current foundation-only behavior, and financial correctness/maintainability priorities. Link the feature map and authoritative spec rather than duplicate acceptance criteria. **Depends**: T019. **Supports**: FR-008, FR-015, AC-06, AC-15. **Verify**: no fund, payment, report, or reminder feature is claimed implemented; assumptions agree with spec.md.
- [X] T021 [P] [US3] Create `docs/product/feature-map.md` mapping identity, funds, income, payments, ledger, reports, and reminders to intended responsibilities; list deferred QR/OCR/AI, allocation, reporting, handoff, banking, salary scheduling, multiple bots/interfaces, financial migrations, and deployment as future scope subject to future specs. Recommend Financial Core/domain invariants next and keep the no-direct-transfer MVP constraint explicit. **Depends**: T019. **Supports**: FR-004, FR-008, FR-015, AC-08, AC-15. **Verify**: each spec exclusion is covered, Financial Core precedes Discord payments, and a roadmap entry never becomes Phase 0 implementation authorization.
- [X] T022 [P] [US3] Create `docs/security/security-baseline.md` with the preliminary threat model (assets, actors, entry points, trust boundaries, threats, mitigations, current state, future owner, residual risks). Cover tokens, environment secrets, sensitive logging, malicious messages/attachments, owner authorization, replay/forged completion, dependency/CI compromise, and safe external integrations; carry every constitutional/spec financial invariant and prohibited banking-data/MVP rule. Distinguish Phase 0 safeguards from unimplemented auth/payment controls. **Depends**: T019. **Supports**: FR-002, FR-006, FR-008, FR-014, AC-07, AC-14, SC-006. **Verify**: each required threat has an explicit boundary/mitigation/status; no real provider/identity system is introduced or claimed; handoff/manual confirmation/bank verification and virtual/verified balances remain separate.
- [X] T023 [P] [US3] Create `AGENTS.md` requiring startup reads of constitution, product overview, handoff, active spec/tasks, related source, architecture, and standards; enforce agreed scope, relevant checks, verified task completion, handoff updates, and ADRs for significant choices. Include discovery from tracked `docs/ai/FEATURE_INDEX.md`/handoff when locally ignored `.specify/feature.json` is missing and preserve generated tooling. **Depends**: T019. **Supports**: FR-008, FR-009, FR-010, AC-06. **Verify**: all eleven requested session behaviors are present, paths are exact, and instructions require evidence without relying on chat or duplicating tasks.
- [X] T024 [P] [US3] Create `.github/workflows/ci.yml` for pushes and pull requests with contents:read, finite timeout, official checkout pinned to a verified maintained release commit SHA and credential persistence disabled, and the same rust-toolchain.toml compiler/components as local. Run fmt plus locked Clippy/all-targets, tests, and check; include no service container, secret, real external account, deployment, privileged pull_request_target, or unnecessary cache/toolchain action. **Depends**: T019. **Supports**: FR-007, FR-011, FR-012, AC-04, AC-13, SC-002. **Verify**: workflow YAML/event/permissions/pin/commands are reviewed, all commands are executed locally using the pinned compiler, and lockfile remains stable; hosted CI stays unverified until a real run exists.
- [X] T025 [US3] Create concise `README.md` covering purpose/vision, implemented versus planned status, chosen stack, architecture links, rustup/toolchain and dependency installation, supported environment setup, run commands, four exact quality commands, Spec Kit workflow/review gates, roadmap, and active feature/task links. Explain no implicit .env loading or service credentials. Link detailed guides instead of duplicating specifications. **Depends**: T020, T021, T022, T023, T024. **Supports**: FR-007, FR-008, FR-010, AC-05, AC-15. **Verify**: every documented command is applicable to the actual package, file links resolve, and setup can be followed without Discord/PostgreSQL/banking accounts.
- [X] T026 [US3] Update `docs/ai/FEATURE_INDEX.md` and `docs/ai/SESSION_HANDOFF.md` with actual phase, active spec/plan/tasks links, verified progress, pending work, blockers, decisions/ADRs, verification limits, and next action; remove stale pre-task statements. Link to `specs/001-project-foundation/tasks.md` for the checklist and keep completed/partial/blocked/unverified states distinct. **Depends**: T025. **Supports**: FR-008, FR-009, AC-06, AC-15, SC-004. **Verify**: no detailed duplicate checklist, false hosted-CI claim, or premature Phase 0 completion; tracked discovery works without feature.json.
- [X] T027 [US3] Perform the independent session/security/CI walkthrough using `AGENTS.md`, `README.md`, `docs/ai/FEATURE_INDEX.md`, `docs/ai/SESSION_HANDOFF.md`, `docs/security/security-baseline.md`, and `.github/workflows/ci.yml`; fix documentation/instruction gaps in their owning files. **Depends**: T026. **Supports**: FR-006, FR-007, FR-008, FR-009, FR-014, AC-05, AC-06, AC-07, AC-13, AC-14, AC-15. **Verify**: a reader without chat locates architecture, scope, current progress, authoritative next task and required checks; CI needs no accounts and documentation does not mistake planned controls for implemented ones.

**Checkpoint**: All three stories are delivered and independently inspectable; final verification
must still run against the final combined tree before Phase 0 is called complete.

## Phase 6: Final Verification and Documentation Consistency (9 tasks)

**Purpose**: Produce honest completion evidence on the final tree. Do not run mutation and checks
concurrently. Fixes invalidate affected prior checks; rerun only those needed after new changes.

- [X] T028 Run `cargo fmt --all -- --check` against `src/` and `tests/` using `rust-toolchain.toml`; record exit/result beside this task. **Depends**: T027. **Supports**: FR-005, FR-007, FR-012, AC-04, SC-002. **Verify**: command exits 0 without source changes; format any failures and repeat before completion.
- [X] T029 Run `cargo clippy --all-targets -- -D warnings` for `Cargo.toml`; resolve actual warnings without weakening lint policy or introducing abstractions just to silence it. **Depends**: T028. **Supports**: FR-005, FR-007, FR-012, AC-04, SC-002. **Verify**: exit 0 and zero warnings on all targets; record any fixes and rerun affected formatting/tests.
- [X] T030 Run `cargo test` for all unit, executable integration, and applicable documentation tests declared by `Cargo.toml`; retain no real credentials or services in the test environment. **Depends**: T029. **Supports**: FR-001, FR-002, FR-007, FR-012, AC-04, AC-10, AC-11, AC-12, SC-001, SC-002. **Verify**: all tests pass, platform-gated coverage is labeled accurately, and no ignored/failing required case is reported complete; record counts/results.
- [X] T031 Run `cargo check` for `Cargo.toml`, then compare Cargo.lock before/after the four requested checks. **Depends**: T030. **Supports**: FR-007, FR-011, FR-012, AC-01, AC-04, SC-002. **Verify**: exit 0, expected imports/types compile, and the committed resolution is stable; investigate rather than conceal dependency drift.
- [X] T032 Execute the setup/run/verification instructions in `README.md` and `specs/001-project-foundation/quickstart.md` from a clean checkout or isolated clean copy that includes the proposed files and lockfile; verify `.env.example` and actual runtime output/exit behavior. Review `.github/workflows/ci.yml` equivalents and inspect any available hosted run without requiring a push or external-account setup. **Depends**: T031. **Supports**: FR-001, FR-007, FR-008, FR-012, AC-05, AC-10, AC-11, AC-13. **Verify**: all documented commands work without production accounts/services; record exact environment and results, mark hosted CI unverified if no run exists, and never claim enabled/configured means passed.
- [X] T033 Review `src/`, `tests/`, `Cargo.toml`, `Cargo.lock`, `.gitignore`, `.env.example`, `.github/workflows/ci.yml`, and `docs/security/security-baseline.md` for real secrets/personal records, unsafe environment mutation, leaks in Display/Debug/output, unnecessary integration features, public API expansion, cycles, placeholders, and premature financial behavior. Inspect `cargo tree -e features` and actual runtime call sites. **Depends**: T032. **Supports**: FR-002, FR-003, FR-004, FR-006, FR-011, FR-012, FR-013, FR-014, AC-02, AC-07, AC-08, AC-09, AC-14, SC-005, SC-006, SC-007. **Verify**: no prohibited content or behavior; current safeguards and future risks are accurately stated; output assertions alone are not claimed to prove absence of network activity.
- [X] T034 Review every required documentation path from `specs/001-project-foundation/spec.md`, both ADRs, and `.specify/memory/constitution.md` for existing useful content, resolving links, correct current/future labels, consistent dependency directions, financial/security rules, and agreement with code/runtime contract. **Depends**: T033. **Supports**: FR-003, FR-004, FR-005, FR-006, FR-008, FR-009, FR-015, AC-02, AC-03, AC-05, AC-06, AC-09, AC-14, AC-15, SC-004, SC-006, SC-007. **Verify**: all requested documents exist, no competing feature spec or duplicated checklist, constitution alignment is actually adopted, and unresolved product decisions have owning future scope.
- [X] T035 Run `$speckit-analyze` across `specs/001-project-foundation/spec.md`, `plan.md`, and `tasks.md` after any approved corrections; verify the coverage matrices below and review the full repository diff, including untracked proposed files, plus `git diff --check`. **Depends**: T034. **Supports**: FR-010, FR-012, AC-08, SC-003, SC-005. **Verify**: no unresolved critical/high inconsistency, orphan requirement/scenario/task, unrelated change, tooling modification, or out-of-scope implementation; analysis remains read-only and any remediation uses the owning workflow before rerunning affected checks.
- [X] T036 Close out `specs/001-project-foundation/tasks.md`, `docs/ai/SESSION_HANDOFF.md`, and `docs/ai/FEATURE_INDEX.md` using actual evidence from T028–T035; update `README.md` status only if necessary, preserving earlier tested commands. Summarize completed/partial/blocked/unverified work, final decisions, and Financial Core as the next specification. **Depends**: T035. **Supports**: FR-008, FR-009, FR-012, FR-015, AC-06, AC-15, SC-003, SC-004. **Verify**: every completed checkbox has evidence, all Definition of Done conditions below hold, final metadata edits pass diff/link checks, and no hosted CI, future feature, deployment, or banking verification is falsely claimed. If source/workflow changes are needed, rerun affected gates before closeout.

## Dependencies and Execution Order

### Phase and Story Dependencies

```text
T001 → T002 → T003 → T004 → {T005, T006}
    → T007 → T008 → T009 → T010 → T011 → T012 → T013  [US1]
    → {T014, T015, T016, T017, T018} → T019          [US2]
    → {T020, T021, T022, T023, T024} → T025 → T026 → T027  [US3]
    → T028 → T029 → T030 → T031 → T032 → T033 → T034 → T035 → T036
```

All user stories depend on setup/foundation. US2 reviews actual US1 source. US3 intentionally
follows US2 so onboarding/security/CI describe a stable architecture and executable. Story checks
remain independently repeatable without invoking a future business feature or external account.

### Safe Parallelization Examples

- **Shared configuration**: T005 rustfmt and T006 ignore rules after T004, on different files.
- **US1**: No task-level parallel edits: lib/config/bootstrap and the integration test file share
  a deliberate sequence. Existing unit tests may execute in parallel because global environment
  is never mutated and subscriber collision is isolated in a child process.
- **US2**: T014 architecture, T015 structure, T016 standards, T017 ADR1, and T018 ADR2 use distinct
  files and the same stable plan after T013; T019 reconciles them before anything relies on them.
- **US3**: T020 product overview, T021 roadmap, T022 security, T023 instructions, and T024 CI use
  distinct files after T019. T025 waits for all so README links and claims agree.
- **Final checks**: Run sequentially on a stable tree; do not mutate source while checks run.

No task that edits the same file as another concurrent task is marked safe for that concurrent
batch. In particular, tasks touching lib.rs, tests/bootstrap.rs, or SESSION_HANDOFF.md are ordered.
Parallelizable tasks do not authorize delegation unless the user or applicable instructions do.

### Critical Path and Incremental Delivery

The hard prerequisites are repository/review validation and constitution alignment (T001–T002),
toolchain/package setup (T003–T006), tested configuration/startup/executable (T007–T013), reviewed
architecture batch (T014–T019), documentation/security/CI batch and onboarding (T020–T027), then all
final gates (T028–T036). Each parallel batch joins before its dependent task; the longest member
of each batch determines elapsed time. No duration estimate is asserted.

US1 after T013 is the smallest useful runnable increment, **not** the complete Phase 0 deliverable.
Finish US2 and US3, then final evidence. Commit coherent increments if authorized; stage explicitly
and keep temporary tool output outside the repository. Never create financial business code,
unused Discord adapters, empty future layers, or a production deployment as a shortcut.

## Acceptance Scenario Coverage

| Scenario | Implementation / documentation tasks | Verification tasks |
| --- | --- | --- |
| AC-01 Repository initialization | T003, T004, T007–T009 | T013, T031 |
| AC-02 Architecture | T014, T015, T017 | T019, T033, T034 |
| AC-03 Clean Code standards | T005, T016 | T019, T028, T029, T034 |
| AC-04 Credential-free checks | T003–T005, T007–T011, T024 | T013, T028–T031 |
| AC-05 Practical setup | T012, T025 | T027, T032, T034 |
| AC-06 Session continuity | T020, T023, T026 | T027, T034, T036 |
| AC-07 Secret-free repository | T006, T011, T012, T022 | T027, T033 |
| AC-08 Minimal foundation | T004, T009, T015, T021 | T019, T033, T035 |
| AC-09 Extensibility | T014, T017 | T019, T033, T034 |
| AC-10 Safe bootstrap | T007–T010 | T013, T030, T032 |
| AC-11 Invalid configuration | T007–T009, T011 | T013, T030, T032 |
| AC-12 Ambient credentials | T011 | T013, T030, T033 |
| AC-13 CI independence | T024 | T027, T032 |
| AC-14 Security review | T002, T022 | T027, T033, T034 |
| AC-15 Honest product state | T020, T021, T025, T026 | T027, T034, T036 |

## Requirement and Outcome Coverage

| Requirement | Tasks |
| --- | --- |
| FR-001 Safe initialization | T007–T010, T013, T030, T032 |
| FR-002 Configuration | T006, T007, T008, T011, T012, T013, T022, T030, T033 |
| FR-003 Architecture | T008, T009, T014, T015, T017, T019, T033, T034 |
| FR-004 Interface evolution | T014, T019, T021, T033, T034 |
| FR-005 Coding standards | T005, T008, T016, T019, T028, T029, T034 |
| FR-006 Security governance | T002, T006, T011, T022, T027, T033, T034 |
| FR-007 Developer tooling | T003–T013, T024, T025, T027–T032 |
| FR-008 Documentation | T015, T016, T020–T023, T025, T026, T027, T032, T034, T036 |
| FR-009 Session continuity | T023, T026, T027, T034, T036 |
| FR-010 Spec-driven delivery | T001, T002, T017, T018, T023, T025, T035 |
| FR-011 Dependency discipline | T003, T004, T013, T018, T024, T031, T033 |
| FR-012 Completion evidence | T001, T013, T024, T028–T033, T035, T036 |
| FR-013 Minimal structure | T004, T014, T015, T017, T019, T033 |
| FR-014 Preliminary threat model | T022, T027, T033 |
| FR-015 Roadmap/scope | T020, T021, T034, T036 |
| SC-001 Safe runtime outcomes | T007–T011, T013, T030 |
| SC-002 Quality gates | T013, T024, T028–T031 |
| SC-003 Traceability and consistency | T001, T035, T036 |
| SC-004 Discoverability/documents | T020–T027, T034, T036 |
| SC-005 No premature behavior | T019, T033, T035 |
| SC-006 Security/integrity coverage | T002, T011, T022, T033, T034 |
| SC-007 Reusable business boundaries | T014, T017, T019, T033, T034 |

## Definition of Done

All 36 tasks must be implemented/reviewed and verified before Phase 0 completion is reported.
The pinned Edition 2024 package compiles; the four required commands pass; meaningful tests cover
configuration, error redaction, startup, process isolation, and absence of service requirements;
CI is configured with least privilege and locked dependencies. Hosted CI success is optional
external evidence and stays explicitly unverified until a run exists.

All required documentation and ADRs contain useful, consistent content. Constitution security
alignment is adopted. Every requirement and acceptance scenario has implementation or review
evidence, the authoritative checklist and handoff agree, and no critical/high consistency issue
remains. A fresh session discovers the next task without chat. Source/diff/security review finds
no secrets, sensitive records, empty placeholders, speculative frameworks, premature financial
business behavior, or deployment. Unfinished or unverifiable work is not marked complete.

## Recorded Open Information and Scope Boundaries

- Constitution 1.1.0 alignment and scoped review authorization were resolved by T001–T002 under
  the explicit Setup/Foundational implementation request. All remaining Phase 0 tasks are now authorized; business features remain excluded.
- Dependency/toolchain versions in research are a dated verified selection; actual availability,
  features, lockfile, and compilation must be checked during T003–T013. Any change must be reconciled
  with plan/research rather than silently expanding dependencies.
- Exact checkout commit SHA is deliberately resolved during T024; hosted CI results depend on
  repository availability/authorized runs and are not a local implementation prerequisite.
- Private Rust type/function names and final ADR status follow the design and existing conventions;
  no unimplemented financial entity or authentication mechanism is needed to proceed.
- Poise/Serenity compatibility is a documented future boundary, not a request for a live client.
  Environment loading means supported process variables; .env autoloading remains excluded.
- Future identity, currency, persistence, multi-user isolation, retention, financial transactions,
  payment handoff, reports, reminders, QR/OCR/AI, banking, and deployment need separate owning specs.

## Historical Setup and Foundational Verification Evidence — 2026-11-10

| Task | Evidence | Limit |
| --- | --- | --- |
| T001 | Read instructions/designs; checklist 16/16 passed. Re-ran read-only consistency checks: 36 dependencies valid, 15 FR/15 AC/7 SC mapped. Remediated C1/I1/E1/U1 in owning artifacts, then reviewed the revised design against the constitution. No remaining critical/high finding identified. | Does not prove unimplemented runtime behavior. |
| T002 | Constitution 1.1.0 adopts required security additions; no existing principle weakened. Spec/plan/research/task/runtime-contract references aligned. Scoped implementation authorization is the user's explicit T001–T006 request. Reviewed and removed temporary sync report. | No runtime/data migration; future security controls remain planned. |
| T003 | rustup installed 1.99.1 with rustfmt/Clippy. rustup show selects project-pinned 1.99.1; rustc 1.99.1, rustfmt 1.10.0-stable, clippy 0.1.99 reported successfully. | Global default stable alias is unchanged. |
| T004 | TOML validation and cargo metadata --no-deps --format-version 1 passed: one package, one explicit library target, Edition 2024, four intended dependency requirements/features. Direct registry checks confirm selected versions are non-yanked and within the compiler's declared MSRV support. | Missing src/lib.rs is intentional until T007; no compilation or resolved dependency graph claimed. |
| T005 | Pinned rustfmt --check --config-path rustfmt.toml passed on formatted stdin. | Actual repository source formatting awaits T007 and final gates. |
| T006 | git check-ignore verified 8 excluded and 6 allowed synthetic paths, including target, .env variants, logs, .env.example, and Cargo.lock. | No test files or credentials were created. |

Reviewed complete proposed file contents and whitespace, and verified generated
skills/scripts/templates/workflows match pre-task SHA-256 fingerprints. No src/, tests/,
Cargo.lock, .env.example, bot, database client, financial behavior, or CI workflow was created.
At this historical checkpoint full cargo fmt/Clippy/test/check had not run because source began
at T007. The later completion-session evidence below supersedes that status.

## Historical Planning Readiness Review — 2026-11-10

The maintainer authorized review and correction of planning artifacts only. No task checkbox
changed, no constitution/configuration/tooling was modified, and no product source was created.

| Finding | Severity | Resolution |
| --- | --- | --- |
| I2: stale state across plan/research/model/quickstart/checklist notes | Low | Distinguish delivered T001–T006 configuration from pending source/tests/runtime verification; tasks.md is authoritative now. |
| U2: T013 could imply executing later quickstart documentation reviews | Medium | Limit the US1 checkpoint to runtime/build sections; defer full onboarding/security/documentation review to existing later tasks. No new dependency or task needed. |
| E2: structured fallback had no explicit executable-test assertion | Medium | Name the failure event in the runtime contract and require ERROR/event/stdout/redaction assertions in T011/T013. Collision testing does not add production test hooks. |

All 15 functional requirements, 7 success criteria, and 15 acceptance scenarios remain mapped.
All 36 task IDs/dependencies and the existing safe parallel batches are retained. Feature-first
boundaries, private implementation details, and the one-package/minimal-dependency decision remain
consistent with constitution 1.1.0. No financial implementation or speculative framework is added.
At that historical review, implementation was ready to start at T007. Subsequent completion
session evidence records the later full authorization and verification.

Verification T007: Configuration tests first failed with missing Config, then all 4 passed; cargo check --lib --locked passed. Cargo.lock generated. Temporary dead-code warnings are expected until bootstrap consumes configuration in T008.

Verification T008: Bootstrap tests first failed with missing startup APIs; then all 6 library tests passed, including isolated subscriber collision. cargo check --lib --locked passed without warnings.

Verification T009: cargo check --locked and cargo run --locked passed. Direct executable under empty environment exited 0, stdout was empty, stderr contained initialization event. No service clients or financial code exist.

Verification T010: Executable success contract passed for absent/default and all five levels in a cleared child environment with 10-second deadline and drained pipes; filtering, empty stdout and no ANSI verified.

Verification T011: cargo test --locked passed: 6 unit and 4 integration tests. Invalid, empty, uppercase, padded, Unix non-Unicode and synthetic ambient credential cases verify ERROR event, redaction, status, stdout and isolation.

Verification T012: Non-sensitive example is allowed by Git, documents process-variable-only loading and all exact levels; actual warn startup exits successfully with no INFO output.

Verification T013: US1 checkpoint passed all four required commands, 10 tests, default/filtered/error startup cases and cargo tree -e features review. No network/client/database dependencies enabled; source has no external connection calls.

Verification T014,T015,T016,T017,T018: Created and reviewed architecture/ownership docs and accepted ADRs against actual source/plan; future capabilities labeled unimplemented. Six standalone good/bad Rust examples compiled in /tmp. Links validated except explicitly pending README/roadmap delivered by later tasks.

Verification T019: Reviewed four production modules and ownership guides/ADRs: private config/bootstrap, only run and StartupError exports, no provider imports, financial behavior, placeholders or feature cycles. Documentation and code agree.

Verification T020,T021,T022,T023,T024: Product roadmap, preliminary threat model and mandatory session instructions reviewed; all links resolve. CI YAML parsed, least-privilege/no-secret/no-deployment constraints checked, official checkout v7.0.1 full SHA/input verified, equivalent locked Cargo commands passed locally. Hosted CI remains unverified.

Verification T025: Concise README covers purpose/status, installation/fetch/run, environment semantics, exact quality commands, architecture, workflow, authoritative links and roadmap. All links resolve and cargo fetch --locked succeeded; run/check commands already executed at T013/T024.

Verification T026: Tracked feature index/handoff identify full current authorization, active artifacts, verified progress and pending final gates without duplicating tasks; all links and ADR paths resolve.

Verification T027: Fresh-context walkthrough from AGENTS/README/index/handoff resolved active artifacts, code/architecture/ADRs, scope, verified state and final next steps. All required document links valid; security/CI state and hosted execution limits distinguished.

Verification T028: Final formatting check passed after applying rustfmt to the last test edit.

Verification T029: Final Clippy all-targets check exited 0 with zero warnings; no lint weakening or suppressions.

Verification T030: Final cargo test passed: 6 unit tests, 4 executable integration tests, zero failures/ignored cases; Unix non-Unicode coverage executed on Linux.

Verification T031: Final cargo check passed. Cargo.lock remains the resolved set of 17 external packages plus fundguard; exact hash stability checked in final validation.

Verification T032: Isolated clean copy without local feature.json passed README/quickstart installation/fetch/fmt/Clippy/tests/check/run and all documented success/failure cases. Cargo.lock SHA256 unchanged. No real service credentials supplied. No repository remote or hosted CI run exists; workflow execution remains unverified.

Verification T033: Reviewed source/call sites, dependency graph, private/public API, workflow, ignores and safe fixtures. Four production modules, four dependencies, no external call sites/unsafe code/financial behavior/placeholders. Secret-pattern scan plus manual review found no real credentials or financial records; tests verify redaction. No production security or network-sandbox guarantee claimed.

Verification T034: Reviewed all requested docs and two accepted ADRs against source, runtime contract and constitution 1.1.0. 20 Markdown files have valid local links/fences/whitespace. Financial/security constraints and current/future labels agree; historical requirements-quality checklist preserved as a planning review, not implementation status.

Verification T035: Read-only cross-artifact analysis of spec/plan/tasks plus constitution/contracts found 15/15 FR, 7/7 SC and 15/15 AC mapped, 36 unique tasks, acyclic prerequisites and no orphan, critical/high issue, scope creep or architecture exception. Full source and proposed documentation/configuration diffs reviewed; generated tooling unchanged; cached diff whitespace clean.

Verification T036: Closed the authoritative checklist and handoff after all prior checks; README/spec/plan/index show locally verified Phase 0 completion, future scope and hosted/non-Linux/security limits remain explicit. Final metadata edits received link/whitespace/task-count checks; no source/workflow edits after passing checks.
