# Tasks: Trusted Discord Owner Entrypoint

**Input**: [spec](spec.md), [plan](plan.md), [research](research.md), [model](data-model.md),
[runtime contract](contracts/runtime.md), [command contract](contracts/command.md), [guide](quickstart.md).
**Status**: Generated on 2026-11-13 under the maintainer's $speckit-tasks request; implementation not authorized.
**Tests**: Required by FR-015. Write behavioral tests before their implementation, observe meaningful
failure, then make them pass. Missing private module wiring may be added with its first real test/code.
**Format**: - [ ] Tnnn [P?] [USn?] Gerund description with repository-relative target paths.
All tasks start unchecked. Dependencies below supplement phase gates; verification clauses define completion.

Use existing single-package Rust setup; no new specification or business feature. Private unit tests
stay beside implementation; split into sibling test files only for actual responsibility/size needs.
Keep each file within repository size guidance. Never expose internals just to test them. All automated
inputs are synthetic; no real SDK client construction, credentials or provider sockets in tests.
Reuse tests/support/mod.rs; no parent environment mutation or production fake-mode setting.

## Phase 1: Setup

**Goal**: Preserve the baseline and resolve only the reviewed integration dependencies.

- [ ] T001 Recording repository/source/checklist baseline against specs/004-discord-owner-entry/plan.md in tasks.md execution notes; inspect Git status, src/, tests/, Cargo.toml and existing CI; verify active feature 004 and preserve unrelated work (FR-014/015).
- [ ] T002 Updating Cargo.toml and Cargo.lock with Poise 0.7.1/Serenity 0.12.5 defaults disabled, Serenity rustls_backend, Tokio macros/rt/signal/sync/time and dev-only test-util per research.md; verify cargo check --locked and cargo tree -e features, retaining pinned toolchain and existing dependencies (after T001; FR-014).

## Phase 2: Foundational

**Goal**: Establish safe errors and diagnostics before any live adapter can execute.
**Independent check**: Safe errors and both subscriber paths pass locally, retaining offline startup tests.

- [ ] T003 Writing categorical-error tests beside src/runtime.rs for configuration/local/session outcomes, nested Debug/Display/source redaction and fixed event_name values; verify failures against runtime contract without formatting raw SDK payloads (after T002; FR-004/012).
- [ ] T004 Defining ApplicationError in src/runtime.rs and declaring private runtime module in src/lib.rs; enforce “No raw SDK/provider payload or unsafe source chain; safe Display/Debug; event_name returns a fixed static reporting event”; verify T003 and leave legacy run/StartupError exports intact (after T003; FR-004/012/014).
- [ ] T005 Writing primary/fallback tracing tests beside src/bootstrap.rs and src/main.rs using captured writers/isolated children; inject SDK/transport sensitive markers at all five levels, misleading fundguard-like targets and ambient RUST_LOG; verify intended failures and retained subscriber-collision behavior (after T004; FR-004/015).
- [ ] T006 Applying target allowlist to src/bootstrap.rs and src/main.rs fallback: exactly fundguard or fundguard:: prefix, existing configured level/ERROR fallback, safe stderr and stdout behavior; verify T005 plus tests/bootstrap.rs, with no raw dependency warnings or environment-filter dependency (after T005; FR-004/012).

**Checkpoint**: T001–T006 complete before story work. No live connector is wired yet.

## Phase 3: US1 — Starting Only the Intended Runtime (P1)

**Goal**: Validate deliberate mode/configuration selection before external access.
**Independent test**: Supplied lookup/connector closures prove zero calls for offline/invalid settings
and exactly one controlled connector for complete live settings, without constructing a real SDK client.

### Tests

- [ ] T007 [P] [US1] Writing supplied-lookup mode tests beside src/config/runtime.rs: absent/offline ignores all bot fields without reading them, exact discord selects validation, empty/unknown/non-text modes fail; verify failures against contracts/runtime.md (after Phase 2; FR-001/002, AC-01–04).
- [ ] T008 [P] [US1] Writing live-field tests beside src/config/discord.rs for each missing/empty/padded/non-text field, u64 boundaries, canonical account grammar, exact nonblank owner bytes, enabled/disabled state and nested credential/config redaction; verify categorical errors retain no supplied markers (after Phase 2; FR-002–004, AC-02/03).

### Implementation

- [ ] T009 [US1] Building private DiscordConfig in src/config/discord.rs using existing account/OwnerRef/OwnerBinding constructors: “All fields required in live mode; no partial default; immutable startup snapshot; aggregate Debug redacted”; SecretToken: “Nonempty UTF-8, no surrounding Unicode whitespace; no format regex, raw Display, serialization or public getter; custom redacted Debug”; target: “Canonical decimal 1–u64::MAX, no padding/sign/leading zero; not an account ID type”; verify T008 (after T007/T008; FR-002–004).
- [ ] T010 [US1] Implementing RuntimeConfig selection and supplied Option<OsString> lookup in src/config/runtime.rs, wiring modules in src/config.rs: “Absent mode → Offline; exact offline/discord only; no provider access during parsing”; binding uses “Existing DiscordAccountId, OwnerRef and explicit Enabled/Disabled; exact equality/validation unchanged”; verify T007/T008, no dotenv/fallback credential/app-ID lookup (after T009; FR-001–003).
- [ ] T011 [US1] Writing selection/orchestration tests beside src/runtime.rs with a captured private connector and controlled initialization; assert complete validation/logging before invocation, zero calls on failure, one on valid enabled/disabled settings and unchanged offline run behavior; verify meaningful failures (after T010; FR-001–004/012, AC-01–04).
- [ ] T012 [US1] Implementing internal async runtime dispatch in src/runtime.rs using the narrow private connector boundary, existing offline initialization and safe error mapping; verify T011 and old bootstrap tests; defer public executable/live wiring to T030 rather than adding a temporary production connector (after T011; FR-001–004/012).
- [ ] T013 [P] [US1] Documenting exact mode and required live settings in .env.example with offline default and blank/synthetic commented values only; state no automatic loading, disabled-binding semantics and secure local token provisioning; verify against contracts/runtime.md and ignore rules (after T010; FR-002–004).

**Checkpoint**: Validate selection locally. This is the first MVP checkpoint; connected operation awaits US3.

## Phase 4: US2 — Checking Readiness as the Configured Owner (P1)

**Goal**: Translate genuine SDK identity into existing owner policy and one private fixed response.
**Independent test**: Real SDK fixtures/builders exercise production mapping; captured sends verify
owner/denial content, private flags and bounded failure without contacting Discord.

### Tests and implementation

- [ ] T014 [US2] Writing mapper/data tests beside src/features/identity/presentation.rs using serenity::json::from_value fixtures; cover owner/nonowner, spoofed names/content, missing/foreign guild, invalid actor, disabled binding, closing gate and repeated requests; verify all denials match and no remembered grant/resource scope exists (after Phase 2 and T010; FR-006–008, AC-05–08).
- [ ] T015 [US2] Defining command data/decision/error and production mapper in src/features/identity/presentation.rs, wiring crate-visible presentation in src/features/identity/mod.rs; CommandData: “Feature presentation owns Poise data; token absent; Debug redacted; flag gates lifecycle only”; decision: “Private presentation output from current server/actor mapping and existing resolve_owner; no owner payload/grant”; error: “Safe presentation-owned Poise error category, no source payload; setup failure details signaled separately”; verify T014 through existing resolve_owner each request (after T014; FR-006–008/014).
- [ ] T016 [US2] Writing actual slash/reply-builder and paused-send tests beside src/features/identity/presentation.rs; serialize to_slash_initial_response through serenity::json::to_value; verify zero args, private flags, exact contract text, empty mentions and one send on error/2s timeout with no fallback (after T015; FR-005/008/009/013, AC-05/06/09).
- [ ] T017 [US2] Implementing /fundguard handler/reply builder in src/features/identity/presentation.rs using actual invoking user/guild, lifecycle accepting gate and narrow private send future; reply: “Fixed content, ephemeral=true, empty allowed mentions; no embeds/attachments or public fallback”; definition: “fundguard, slash/chat input, zero arguments; only named target-guild upsert”; verify T016 with immediate dispatch, no auto-defer/owners_only/context precheck bypass, one 2s logical send (after T016; FR-005–009/013).
- [ ] T018 [US2] Verifying src/features/identity/presentation.rs with targeted unit tests plus tests/identity.rs and tests/financial_core.rs; review SDK-independent domain/application, unchanged existing policy and private adapter visibility; record results in tasks.md (after T017; FR-007/014/015, AC-05–09).

**Checkpoint**: Owner readiness is proven under controlled transport boundaries; no live-delivery claim.

## Phase 5: US3 — Operating the Small Integration Safely (P2)

**Goal**: Wire scoped registration, one supervised session and bounded cleanup behind explicit opt-in.
**Independent test**: Captured named-upsert operations, real framework configuration, paused supervisor
signals and real executable offline/invalid cases verify operation without provider connections.

### Registration and safe errors

- [ ] T019 [P] [US3] Writing captured-upsert tests beside src/platform/discord/registration.rs for actual slash builder, exact guild/name/type/zero args, one operation, no unrelated-command replacement, safe failure and min(10s, remaining startup) timeout; verify failures without an HTTP client (after T018; FR-010/013, AC-10/11).
- [ ] T020 [US3] Implementing named registration in src/platform/discord/registration.rs through Command::create_as_slash_command and GuildId::create_command; preserve SDK rate limiting, never use bulk/global helpers or cleanup; verify T019 and actual production method selection, not only fake call counts (after T019; FR-010/013).
- [ ] T021 [P] [US3] Writing captured diagnostic/framework-error tests beside src/platform/discord/diagnostics.rs for safe static categories, synthetic provider markers, no extra response and no direct raw stderr; verify failure for every handled framework variant/fallback (after T018; FR-004/009/012, AC-09/11).
- [ ] T022 [US3] Implementing safe category mapping and full custom Poise on_error in src/platform/discord/diagnostics.rs; never delegate to builtins, retain SDK errors, print raw errors or send another reply; verify T021 and T005 logging filters (after T021; FR-004/009/012).

### Supervisor and cleanup

- [ ] T023 [P] [US3] Writing paused-time supervisor tests beside src/platform/discord/lifecycle.rs for 30s absolute startup including builder, setup Failed/closed channel with pending client, unexpected Ok/Err/panic/cancellation and observed recovery; verify readiness only after successful setup and failure categories without SDK clients (after T018; FR-012/013, AC-11/12).
- [ ] T024 [US3] Implementing concrete supervisor inputs/state in src/platform/discord/lifecycle.rs: “Connected or Unavailable mapped from the current single-shard manager snapshot”; sample 1s after first ready, expire 30s after observed loss, clear only on Connected; race setup/client/interruption/deadlines and keep SDK reconnect behavior without outer retries; verify T023 (after T023 and T022; FR-012/013).
- [ ] T025 [US3] Writing paused cleanup tests beside src/platform/discord/lifecycle.rs for interrupt during builder/setup/running/recovery, immediate accepting=false, pending/failed close and abort/join; verify 8s SDK close and one 10s total deadline, clean interruption success versus failure exits (after T024; FR-012/013, AC-13).
- [ ] T026 [US3] Implementing bounded stop/close/cancel/join in src/platform/discord/lifecycle.rs with no trailing unbounded await; “Clean bounded closure returns success; timeout/failure returns a sanitized ApplicationError”; ensure no surviving tracked reconnect/session work; verify T025 including failures during cleanup (after T025; FR-012/013).

### Client and executable integration

- [ ] T027 [US3] Writing client/setup-wiring tests beside src/platform/discord/client.rs using controlled operations; assert empty intents, one shard/command, both owner shortcuts disabled, custom errors, setup once across Ready/reconnect and Failed signal before Err; verify no real builder connection and no false discord_ready (after T020/T022/T026; FR-010–013, AC-10–13).
- [ ] T028 [US3] Wiring real SDK framework/client/setup in src/platform/discord/client.rs, narrow session entry in src/platform/discord/mod.rs and src/platform/mod.rs, root declaration in src/lib.rs; one consumed setup registers before signaling readiness, forwards safe failure independently of Client::start, samples real runners and supervises from before builder await; verify T027 and T019–T026 (after T027; FR-010–014).
- [ ] T029 [US3] Writing tests/discord_entry.rs with tests/support/mod.rs child harness/env_clear for default/explicit offline, malformed ambient bot fields, invalid mode/log level and each incomplete live setting; assert safe exit/events/stdout/no sensitive markers within process deadline; never run valid-live binary configuration (after T028; FR-001–004/012/015, AC-01/03/04/11/14).
- [ ] T030 [US3] Connecting public async start/ApplicationError exports in src/lib.rs, actual selected connector in src/runtime.rs and awaited start in src/main.rs; preserve synchronous offline run/StartupError, fixed error event names and scoped fallback, no nested runtime; verify T029/T011 and all bootstrap/collision tests (after T029; FR-001/012/014/015).
- [ ] T031 [US3] Verifying targeted command/registration/client/supervisor tests and tests/discord_entry.rs; review src/platform/discord/ production call sites for named-only registration, no owner discovery, one session, deadline coverage and no live calls in automated tests; record evidence in tasks.md (after T030; FR-010–015, AC-10–14).
- [ ] T032 [US3] Reconciling specs/004-discord-owner-entry/quickstart.md with implemented commands and opt-in local smoke procedure; record registration/owner/denial/privacy/shutdown as pass/fail/unverified separately, leave live checks unverified unless voluntarily run, never request tokens in chat (after T031; FR-011/016).

**Checkpoint**: Three stories work together; simulated verification does not establish live authentication/delivery.

## Phase 6: Polish & Cross-Cutting Concerns

**Goal**: Finish documentation, credential-free CI and evidence-backed delivery.

- [ ] T033 [P] Updating docs/architecture/overview.md, docs/development/project-structure.md and docs/architecture/decisions/0005-trusted-discord-entrypoint.md to reflect actual wiring/directions, retained offline API and reviewed decision status; verify imports/public API match without new abstractions (after T031; FR-014).
- [ ] T034 [P] Updating docs/security/security-baseline.md threat/control status for live token provisioning/rotation, operator binding, reply privacy, allowlisted logs, safe failures and least privilege; verify no production/security-certification claim or financial authorization shortcut (after T031; FR-003/004/011/016).
- [ ] T035 Updating README.md, docs/product/overview.md and docs/product/feature-map.md with actual offline/opt-in behavior, exact setup/access, executable commands and scope limits; link authoritative contracts/guide rather than duplicate requirements; verify every documented automated command, with manual live commands clearly opt-in (after T032–T034; FR-001/011/016).
- [ ] T036 Reviewing .github/workflows/ci.yml against full suite and reviewed Cargo.lock; retain locked checks, minimal permissions, no credentials/live startup/provider accounts or needless jobs; change only concrete gaps and verify YAML/commands locally, hosted status separately (after T035; FR-015).
- [ ] T037 Running cargo fmt --all -- --check, cargo clippy --all-targets -- -D warnings, cargo test and cargo check from repository root, plus FUNDGUARD_MODE=offline cargo run --locked; record actual outcomes in specs/004-discord-owner-entry/tasks.md, retain previous suites and all new cases, report failures honestly (after T036; FR-015, AC-14).
- [ ] T038 Reviewing complete diff/imports/dependency features/secrets and spec-plan-task AC/FR/SC coverage; reconcile docs/ai/FEATURE_INDEX.md, docs/ai/SESSION_HANDOFF.md, devlog.md and specs/004-discord-owner-entry/tasks.md with verified status, blockers and next action; confirm no financial/persistence/deployment scope, no empty modules and all documentation links valid (after T037; FR-004/014–016).

## Dependencies & Execution Order

Phase gates: Setup → Foundational → US1 → US2 → US3 → Polish. This conservative delivery order
avoids concurrent changes to root/module declarations; it is not a claim that stories lack dependencies.
US2's mapper can be tested without a running session; US3 needs the real US2 command for registration.
US1's dispatch is independently testable with a private closure; executable live wiring waits for US3.

Within phases, follow each explicit “after” clause. Unqualified sequential tasks follow earlier
phase gates and listed dependencies. Tests may be temporarily failing until paired implementation;
never mark a test task complete merely because it was written. Verify the paired behavior first.

| Work | Additional ordering / safe parallel example |
| --- | --- |
| US1 | T007 + T008 after Phase 2 (different test files); T009 → T010 → T011 → T012; T013 can accompany T011/T012 after T010 |
| US2 | T014 → T015 → T016 → T017 → T018; one shared presentation file means no safe simultaneous edits |
| US3 | T019 + T021 + T023 after T018 (separate files); T020 and T022 follow their tests; T024 needs T022/T023; T025 → T026; T027 → T028 → T029 → T030 → T031 → T032 |
| Polish | T033 + T034 after T031; T035 waits for T032/T033/T034; then T036 → T037 → T038 |

[P] means safe alongside the examples above once prerequisites are complete, not permission to race
an implementation with tests editing its file. No delegation or live access is implied by this list.
Critical path runs through dependencies → diagnostics → US1 config → US2 private command → US3
supervisor/client/root → full verification. No deploy step, database, resource query or financial logic.

## Coverage and Verification

| Requirement | Implementation / verification tasks | Acceptance |
| --- | --- | --- |
| FR-001 | T007/T010–T012/T029/T030/T037 | AC-01 |
| FR-002 | T007–T013/T029/T030 | AC-02–04 |
| FR-003 | T008–T010/T014/T015/T034 | AC-03/07/08 |
| FR-004 | T003–T006/T008/T009/T021/T022/T034/T038 | AC-03/09/11 |
| FR-005 | T016/T017/T019/T020 | AC-05/06/10 |
| FR-006 | T014/T015/T017 | AC-05–08 |
| FR-007 | T014/T015/T018 | AC-05–08 |
| FR-008 | T014–T017 | AC-05–09 |
| FR-009 | T016/T017/T021/T022 | AC-09 |
| FR-010 | T019/T020/T027/T028/T031 | AC-10/11 |
| FR-011 | T027/T028/T032/T034/T035/T038 | AC-08/10/14 |
| FR-012 | T003/T004/T021–T031 | AC-11–13 |
| FR-013 | T016/T017/T019/T020/T023–T028/T031 | AC-09–13 |
| FR-014 | T001/T002/T018/T028/T030/T033/T038 | AC-14 |
| FR-015 | T005/T007/T008/T011/T014/T016/T018/T019/T021/T023/T025/T027/T029/T031/T036–T038 | AC-14 |
| FR-016 | T032/T034/T035/T038 | Manual smoke evidence limits |

SC-001: T007–T012/T029; SC-002: T014–T018; SC-003: T016/T017/T021/T022;
SC-004: T019/T020/T027/T028/T031; SC-005: T031/T032/T036–T038;
SC-006: T023–T031/T037/T038. All fourteen AC are covered by controlled tests or explicit wiring/docs
review. Named upsert review supports preservation semantics; offline tests do not prove provider state.

## Implementation Strategy and Definition of Done

First deliver Setup/Foundational and US1's credential-free selection checkpoint. Then US2's private
command boundary, followed by US3's supervised real adapter. Do not expose an incomplete connected
executable after US1. Run relevant tests at each checkpoint; stop and report real blockers.
After authorized implementation, commit coherent verified increments and update this checklist/handoff.

Completion requires passing four Cargo gates, retained offline/core/identity behavior, tested private
command/error/registration/lifecycle paths and all deadlines. Docs/CI/configuration must agree with
actual code; source/visibility/security/dependency and scope review must pass. No task completion
without verification evidence. Optional live smoke and hosted CI may remain explicitly unverified;
actual live failures remain known issues rather than being overruled by simulated success.

## Execution Notes

No implementation tasks completed and no application verification run during task generation.
Record concise task IDs, commands/results and any blocked/partial work here during implementation;
link detailed progress from handoff instead of copying this checklist into other documents.
