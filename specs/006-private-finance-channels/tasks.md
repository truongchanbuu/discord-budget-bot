# Tasks: Private Finance Channels

Input: [spec](spec.md), [plan](plan.md), [research](research.md), [model](data-model.md),
[UI](contracts/discord.md), [operations](contracts/operations.md), [security](contracts/security.md), [guide](quickstart.md).
Maintainer's speckit-tasks request accepts plan review. Planning only: every task starts unchecked.
Tests are mandatory (FR-015). Record meaningful expected failures before paired code and green results
before story completion; never call an expected red test a passing feature. Split files by responsibility
before250 lines. Do not read real env, touch the live bot/cloud, change generated tooling or deploy.
Every task inherits its phase gate plus explicit after IDs. Task paths are repository-relative;
continuation lines quote authoritative field constraints and are part of that task's description.
Ports grow only with fully implemented production methods and updated fakes in the same task;
compile/check each expansion. No todo!(), unimplemented!() or fake unavailable implementation.
Create/overwrite adapters precede provisioning/root consumers; sharing-specific store methods extend later.

## Phase 1: Setup

Goal: establish a safe existing-project baseline; no reinitialization or dependency upgrades.

- [ ] T001 Recording Git/source/test baseline in specs/006-private-finance-channels/tasks.md; preserve live bot, ignored config and generated tooling (after none; FR-015, FR-016; verify: clean scoped diff and recorded prior-test limits).
- [ ] T002 [P] Extracting the private DB fixture guard into src/platform/database/test_support.rs and reusing it in src/features/ledger/infrastructure/tests.rs (after T001; FR-015; verify: existing DB suite passes; real/remote URLs remain rejected).
- [ ] T003 [P] Testing and adding optional category/onboarding config in src/config/discord.rs and src/config/space_tests.rs (after T001; FR-001, FR-010; verify: absent/offline/partial/invalid/duplicate/1–8-ID cases pass without real env).

## Phase 2: Foundational

Goal: validated models, permission facts, durable storage and two real ports; blocks every story.

- [ ] T004 Writing reference/state/redaction tests in src/features/finance_spaces/domain/model_tests.rs (after T002, T003; FR-002, FR-005, FR-013; verify: record expected failure before paired model code).
- [ ] T005 Defining validated refs, static errors and space state in src/features/finance_spaces/domain/model.rs and src/features/finance_spaces/domain/error.rs; wiring src/features/mod.rs (after T004; FR-002, FR-005, FR-013; verify: reference/redaction and ready-state tests pass).
  `id`: “Generated positive primary key”; `guild_id, member_id`: “Canonical nonblank refs, UNIQUE pair; immutable ownership”; `journal_owner`: “FK fg_owners(owner), UNIQUE; immutable trusted mapping”.
  `legacy_binding`: “Exact configured account/OwnerRef association; at most one legacy space per guild”; `owner_joined_at`: “Last verified owner incarnation; NULL cannot authorize”; `generation`: “Fresh UUID text for channel creation/recovery; changes invalidate previous generation”.
  `channel_id`: “Nullable UNIQUE active-candidate ref; never names as identity”; `category_id`: “Configured parent, checked against current authoritative channel”; `state`: “prepared, creating, uncertain, verifying, ready, access_pending, suspended, retired”.
  `revision`: “Nonnegative monotonic CAS version, checked addition”; `current_operation_id`: “Nullable pending operation, at most one outstanding per space”; `created_at, updated_at`: “UTC audit timestamps”.
  Canonical positive refs use no SDK type; unknown joined_at denies; ready permits prepared unconfirmed UI, never unresolved dispatched access.

- [ ] T006 Defining operation state in src/features/finance_spaces/domain/model.rs (after T005; FR-002, FR-008, FR-013; verify: source/revision/transition tests pass).
  `id`: “Server-generated opaque unique operation reference”; `space_id`: “FK space, authoritative parent”; `kind`: “create, grant_viewer, revoke_viewer”.
  `source`: “Bounded trusted message/interaction origin, UNIQUE(space_id,source)”; `actor`: “Authenticated initiating member, no payload/note”; `generation, expected_revision`: “Immutable binding/version for this operation”.
  `target_member, target_joined_at`: “Required only for sharing; exact validated target”; `desired_access`: “viewer/absent for sharing, never editor”; `phase`: “prepared, dispatched, uncertain, verifying, confirmed, failed, retired”.
  `returned_channel_id`: “Candidate returned by create, not ownership proof alone”; `created_at, dispatched_at, completed_at`: “UTC lifecycle; dispatch is committed before provider call”; `outcome`: “Safe category only, no raw API/SQL error or financial content”.
  One nonterminal operation per space; retired means local retirement, never remote cancellation.

- [ ] T007 Defining viewer/audit records in src/features/finance_spaces/domain/model.rs (after T006; FR-007, FR-008, FR-009; verify: incarnation/generation and audit-redaction tests pass).
  `space_id, generation, member_id`: “Composite primary key, FK space; member distinct from owner/bot”; `member_joined_at`: “Incarnation explicitly approved by owner”; `desired_state`: “viewer, revoked”.
  `confirmed_state`: “pending, viewer, absent, uncertain”; `revision`: “Monotonic per-grant version”; `operation_id`: “Latest authoritative grant/revoke operation”.
  `granted_by, changed_at`: “Actor and UTC audit”; Audit: “Append-only space/operation/source/actor/action/outcome/time audit.”; “No financial payload, token, password, full provider body or unbounded external string.”; Current-generation viewers only; maximum8; unknown, bot, owner and elevated recipients cannot be granted read-only access.

- [ ] T008 [P] Writing owner/membership/privacy policy tests in src/features/finance_spaces/domain/policy_tests.rs (after T007; FR-003, FR-005, FR-007, FR-009; verify: record red cases for outsider, elevated guest, drift and unknown snapshots).
- [ ] T009 [P] Writing real SDK mask/mention fixtures in src/features/finance_spaces/infrastructure/discord_tests.rs (after T007; FR-001, FR-003, FR-004; verify: record red cases for role-view allow, reply-only ping, obfuscation and missing bot bits).
- [ ] T010 Implementing privacy/owner policy in src/features/finance_spaces/domain/policy.rs (after T008; FR-003, FR-005, FR-007, FR-009; verify: all policy cases pass; Administrator visibility never grants journal writes).
- [ ] T011 Writing additive schema/RLS/upgrade tests in src/features/finance_spaces/infrastructure/schema_tests.rs (after T009, T010; FR-002, FR-005, FR-010, FR-015; verify: record expected0003 failure using only the private fixture guard).
- [ ] T012 Adding migrations/0003_private_finance_spaces.sql and registering it in src/platform/database.rs; running both DB suites from scripts/test-postgres.sh (after T011; FR-002, FR-008, FR-010, FR-015; verify: quoted model constraints, audit immutability, RLS/revokes and idempotent upgrade pass).
  Enforce all quoted space/operation/viewer constraints above, FKs/unique partial indexes and no cascade into ledger history; leave0001/0002 unchanged; Update existing src/features/ledger/infrastructure/security_tests.rs migration-count assertions; do not report a zero-matching new suite as passed.

- [ ] T013 Defining current-use SpaceStore/ChannelDirectory ports and facade in src/features/finance_spaces/application/ports.rs and src/features/finance_spaces/application/mod.rs (after T012, T010; FR-014; verify: current methods compile with complete adapters/fakes; no future method stubs or provider types).
- [ ] T014 Implementing validated SQL mapping/read/CAS/audit primitives in src/features/finance_spaces/infrastructure/postgres.rs and src/features/finance_spaces/infrastructure/mapping.rs (after T013; FR-002, FR-008, FR-013; verify: restoration, stale CAS, one-event-per-transition and safe-error DB tests pass).
- [ ] T015 Implementing canonical SDK inspection/builders in src/features/finance_spaces/infrastructure/discord.rs (after T014, T009; FR-001, FR-003, FR-004, FR-009; verify: actual SDK masks and fresh member/role/channel facts satisfy earlier fixtures).
- [ ] T016 Adding configured_account access in src/features/identity/application.rs and legacy resolution in src/features/finance_spaces/application/authorization.rs (after T015; FR-005, FR-010; verify: enabled exact binding, disabled reserved actor and unrelated new-member tests pass).

## Phase 3: US1 — Opening a personal channel (P1)

Independent check: AC-01–04; two members, ten starts, failure/restart. Safe onboarding is the MVP; live guild finance stays gated until US4/US2.

- [ ] T017 [P] [US1] Writing provisioning/crash/mention tests in src/features/finance_spaces/application/provisioning_tests.rs (after T016; FR-001, FR-002, FR-011; verify: record red boundaries before dispatch, after remote success and before ready).
- [ ] T018 [P] [US1] Writing reservation/quota/legacy tests in src/features/finance_spaces/infrastructure/provisioning_tests.rs (after T016; FR-002, FR-010, FR-013; verify: record red two-owner, ten-start, concurrent guild-quota and remap fixtures).
- [ ] T019 [US1] Implementing durable reservation/journal association in src/features/finance_spaces/application/ports.rs and src/features/finance_spaces/infrastructure/postgres.rs (after T017, T018; FR-002, FR-010, FR-013; verify: reservation DB tests pass without touching financial entries).
  Use guild quota lock then space/operation lock; max50 nonretired slots including uncertain;3 creations/member/10min and10/guild/min; never hold locks across HTTP; Reserve one legacy mapping/guild from exact enabled config; reject conflicting startup remaps and new-ref collisions; generate other refs server-side.

- [ ] T020 [US1] Adding complete SDK create/overwrite methods in src/features/finance_spaces/application/ports.rs and src/features/finance_spaces/infrastructure/discord.rs (after T019; FR-002, FR-003, FR-013; verify: all declared create/PUT/DELETE methods compile; restrictive masks, marker, safe errors and SDK fixtures pass).
- [ ] T021 [US1] Implementing prepared/dispatch/verifying/ready workflow in src/features/finance_spaces/application/provisioning.rs (after T020; FR-001, FR-002, FR-003; verify: fault tests prove dispatch persisted before provider call and no blind repeat).
- [ ] T022 [US1] Implementing shared mention classification and onboarding UI in src/features/finance_spaces/application/mod.rs and src/features/finance_spaces/presentation/messages.rs (after T021; FR-001, FR-004, FR-011, FR-012; verify: start/access/finance routes are disjoint; public financial text is neither echoed nor persisted).
- [ ] T023 [US1] Adding safe pending/ready/welcome views in src/features/finance_spaces/presentation/views.rs and routing fixtures in src/features/finance_spaces/presentation/tests.rs (after T022; FR-001, FR-011, FR-012; verify: US1 AC-01–04 pass in controlled ports/SDK fixtures; no finance accepted before US4 safety wiring).

## Phase 4: US4 — Keeping privacy valid (P1)

Independent check: AC-13–16; permission drift/deletion/departure/rejoin, unknown outcomes and fresh-channel recovery preserve journal. Required before guild finance.

- [ ] T024 [P] [US4] Writing retirement/rejoin/unknown-outcome tests in src/features/finance_spaces/application/recovery_tests.rs (after T023; FR-002, FR-008, FR-009; verify: record red zero/multiple/hidden candidate and out-of-order Remove/Add cases).
- [ ] T025 [P] [US4] Writing real lifecycle-event fixtures in src/features/finance_spaces/presentation/lifecycle_tests.rs (after T023; FR-003, FR-009, FR-013; verify: record red channel/category/role drift, duplicate Add and missing incarnation cases).
- [ ] T026 [US4] Implementing invalidation/reconciliation/recovery in src/features/finance_spaces/application/recovery.rs (after T024, T025; FR-002, FR-008, FR-009; verify: recovery tests pass; empty listing never proves absence and retired IDs never reactivate).
- [ ] T027 [US4] Persisting retirement/incarnation transitions in src/features/finance_spaces/infrastructure/postgres.rs and src/features/finance_spaces/infrastructure/lifecycle_tests.rs (after T026; FR-002, FR-008, FR-009; verify: durable restart/CAS tests preserve journal and never carry viewers into a new generation).
- [ ] T028 [US4] Writing financial/management authorization tests in src/features/finance_spaces/application/authorization_tests.rs (after T027; FR-003, FR-005, FR-009, FR-010, FR-011, FR-015; verify: record red ready-versus-retired recovery, fresh incarnation, wrong owner/channel, guest/admin and disabled-legacy cases).
- [ ] T029 [US4] Implementing authorize_finance/authorize_owner_management in src/features/finance_spaces/application/authorization.rs and src/features/finance_spaces/application/mod.rs (after T028; FR-003, FR-005, FR-009, FR-010, FR-011; verify: full authorization tests pass before root/Ledger wiring; management scope cannot supply a journal binding).
  Finance requires current verified ready owner/channel/privacy; management permits only safe owner status/cancel/recovery in non-ready states. Fresh recovery binds incarnation/revision; old-channel controls and unresolved-write bypass remain denied.

- [ ] T030 [US4] Mapping tracked membership/channel/role events in src/features/finance_spaces/presentation/mod.rs (after T029; FR-003, FR-009; verify: SDK fixtures pass; invalidate before provider cleanup and never auto-grant on Add).
- [ ] T031 [US4] Wiring optional Spaces/HTTP setup in src/platform/discord/client.rs and src/runtime.rs (after T030; FR-003, FR-010, FR-014; verify: one client.http binding, unbound denial, configured-guild preflight and pre-finance legacy-conflict tests pass).
- [ ] T032 [US4] Owning bounded reconciliation/shutdown in src/platform/discord/lifecycle.rs and src/platform/discord/lifecycle_tests.rs (after T031; FR-009, FR-013, FR-014; verify: startup/reconnect invalidation precedes guild acceptance; cancellation/join/close tests pass).
  Enable only GUILDS/GUILD_MESSAGES/GUILD_MEMBERS/DIRECT_MESSAGES; no MESSAGE_CONTENT/PRESENCES; two-space/four-lookup batches; respect2s/5s/10s/20s operation budgets without adding a scheduler.

- [ ] T033 [US4] Adding explicit fresh-channel recovery confirmation in src/features/finance_spaces/presentation/components.rs and src/features/finance_spaces/presentation/views.rs (after T032; FR-009, FR-011, FR-012; verify: AC-13–16 pass; disclose old-native access uncertainty and use management guard in retired state; require fresh owner/incarnation/revision and reject old controls before replacement).

## Phase 5: US2 — Recording in the correct channel (P1)

Independent check: AC-05–09; two isolated journals, owner-only controls, legacy DM preserved and no financial public fallback.

- [ ] T034 [P] [US2] Writing DM/guild authorization tests in src/features/ledger/application/scope_tests.rs (after T033; FR-004, FR-005, FR-010; verify: record red viewer/admin/cross-space/disabled-legacy and fresh-context cases).
- [ ] T035 [P] [US2] Writing finance routing/acknowledgment fixtures in src/features/ledger/presentation/guild_tests.rs (after T033; FR-004, FR-005, FR-011; verify: record red public/unmentioned/copied-control/output-drift/failed-ack cases).
- [ ] T036 [US2] Resolving guild request scope through Spaces in src/features/ledger/application/mod.rs (after T034, T035, T029; FR-005, FR-006, FR-010, FR-014; verify: application tests pass; every use case calls authorize_finance before existing JournalStore calls).
- [ ] T037 [US2] Adding owner guild messages/controls in src/features/ledger/presentation/mod.rs, src/features/ledger/presentation/messages.rs and src/features/ledger/presentation/components.rs (after T036; FR-004, FR-005, FR-006, FR-011; verify: SDK fixtures pass; retain DM branch, source/channel/revision checks and no private/public fallback).
- [ ] T038 [US2] Rechecking authorize_finance/space privacy before financial output in src/features/ledger/presentation/views.rs and src/features/ledger/presentation/components.rs (after T037; FR-005, FR-006, FR-011, FR-013; verify: lost-access-after-commit and failed-delivery tests retain one effect; edit/ack2s deadlines hold).
- [ ] T039 [US2] Adding two-owner durable finance regressions in src/features/ledger/infrastructure/guild_tests.rs (after T038; FR-005, FR-006, FR-010, FR-015; verify: income16m/expense6m/reserves5m, replay/rollback/restart and legacy DM data reconcile independently).
- [ ] T040 [US2] Enabling guild finance after safety readiness in src/platform/discord/client.rs and tests/discord_entry.rs (after T039; FR-004, FR-010, FR-014; verify: AC-05–09 and existing offline/readiness/DM suites pass; no second handler processes start/access input).

## Phase 6: US3 — Sharing read-only access (P2)

Independent check: AC-10–12; owner grants/revokes, guests cannot post/operate, uncertainty never reports success.

- [ ] T041 [P] [US3] Writing prepared/confirm/revoke fault tests in src/features/finance_spaces/application/sharing_tests.rs (after T040; FR-007, FR-008, FR-013; verify: record red abandoned-prepared slot release, exact5min boundary, confirm/cleanup race and never-expire-dispatched cases).
- [ ] T042 [P] [US3] Writing actual User-select/confirmation fixtures in src/features/finance_spaces/presentation/access_tests.rs (after T040; FR-007, FR-011, FR-012; verify: record red guest/foreign refs, history-warning, limits and ephemeral-vs-message cases).
- [ ] T043 [US3] Persisting viewer preparation/confirmation/audit in src/features/finance_spaces/application/ports.rs, src/features/finance_spaces/infrastructure/postgres.rs and src/features/finance_spaces/application/recovery.rs (after T041, T042; FR-007, FR-008, FR-013; verify: DB tests enforce slot release/one expiry audit/CAS, no native write, current generation, max8 and10 changes/space/min).
  Expire only prepared grant/revoke with dispatched_at=NULL at now>=created_at+5min; atomically fail/clear matching pointer/audit once with freshly loaded revisions. Prepared UI changes only operation intent, never viewer rows. Invoke before management preparation/status/confirm/cancel and startup; never expire create/dispatched/uncertain or promote privacy state.
- [ ] T044 [US3] Verifying sharing-specific SDK PUT/DELETE/readback in src/features/finance_spaces/infrastructure/discord_tests.rs (after T043; FR-003, FR-007, FR-008; verify: existing concrete methods target only the stored viewer, respect MANAGE_ROLES/masks and safe readback; no deferred production stubs).
- [ ] T045 [US3] Implementing owner-only viewer workflows in src/features/finance_spaces/application/sharing.rs (after T044; FR-007, FR-008, FR-012; verify: fault tests pass; management/finance guards stay distinct; expired slot accepts a new operation, no pre-confirm native change or false revoke success).
- [ ] T046 [US3] Implementing quyền launcher/select/confirm controls in src/features/finance_spaces/presentation/components.rs, src/features/finance_spaces/presentation/messages.rs and src/features/finance_spaces/presentation/views.rs (after T045; FR-007, FR-011, FR-012; verify: actual SDK fixtures pass; refs<=100, one human selected, private lists, no password/editor actions).
- [ ] T047 [US3] Adding durable sharing/lifecycle races in src/features/finance_spaces/infrastructure/sharing_tests.rs (after T046; FR-007, FR-008, FR-009, FR-015; verify: AC-10–12 plus leave/rejoin/elevation/unknown-grant-then-revoke pass on private PostgreSQL).
- [ ] T048 [US3] Integrating fgspace controls and disclosure blocking in src/features/finance_spaces/presentation/mod.rs (after T047; FR-007, FR-008, FR-011; verify: US3 controlled owner/viewer/outsider flows pass; prepared UI may coexist with ready, dispatched uncertainty may not).

## Phase 7: Polish and cross-cutting verification

Goal: required gates, both isolated DB suites, documentation/security review and honest delivery evidence.

- [ ] T049 Reviewing imports/visibility/privacy/quotas across src/features/finance_spaces/ and src/features/ledger/ (after T048; FR-003, FR-005, FR-013, FR-014; verify: acyclic graph, unchanged financial engine, safe errors/audit and no speculative dependencies verified).
- [ ] T050 [P] Updating README.md, .env.example and docs/security/security-baseline.md with actual setup/limits (after T049; FR-012, FR-016; verify: documented config, member intent, permissions and no-password/admin/history warnings match implemented behavior).
- [ ] T051 [P] Reconciling docs/architecture/overview.md, docs/architecture/decisions/0008-private-finance-spaces.md and docs/development/project-structure.md (after T049; FR-014, FR-016; verify: actual modules/recovery limits documented; ADR accepted only after verified implementation).
- [ ] T052 Running both explicit DB suites in .github/workflows/ci.yml and checking scripts/test-postgres.sh cleanup (after T050, T051; FR-015; verify: local CI-equivalent command lists match; private fixture guard retained and neither suite silently skipped).
- [ ] T053 Running fmt/Clippy/test/check and offline smoke; recording outcomes in specs/006-private-finance-channels/tasks.md (after T052; FR-015; verify: cargo fmt --all -- --check; cargo clippy --all-targets -- -D warnings; cargo test; cargo check; offline run pass).
- [ ] T054 Running scripts/test-postgres.sh after final code changes and recording separate suite totals in specs/006-private-finance-channels/tasks.md (after T053; FR-002, FR-006, FR-008, FR-010, FR-015; verify: all isolated upgrade/constraint/concurrency/restart/financial/security cases pass and fixture cleanup verified).
- [ ] T055 Validating acceptance evidence against specs/006-private-finance-channels/quickstart.md and recording local/live limits in specs/006-private-finance-channels/tasks.md (after T054; FR-012, FR-015, FR-016; verify: all 16 ACs have test/review evidence; optional live personas explicitly NOT RUN unless separately authorized).
- [ ] T056 Reviewing final diff and updating docs/ai/FEATURE_INDEX.md, docs/ai/SESSION_HANDOFF.md, docs/product/overview.md, docs/product/feature-map.md and devlog.md (after T055; FR-014, FR-016; verify: spec/plan/tasks/docs agree; completed/partial/blocked/unverified distinct; no deployment or unrequested live mutation).

## Dependencies and parallel execution

Setup → Foundational → US1 → US4 → US2 → US3 → Polish. US4 precedes US2 because privacy invalidation
must exist before accepting guild finances; all P1 stories precede P2 sharing. Each task's explicit
after list plus phase barrier is authoritative. Do not parallel-edit shared mod/model/postgres/client files.

Safe examples after their prerequisites: Setup T002/T003; Foundational T008/T009;
US1 T017/T018; US4 T024/T025; US2 T034/T035; US3 T041/T042; Polish T050/T051.
[P] pairs write distinct files/parent modules; serialize test-module declarations if a shared parent
needs editing, then run final verification after joining. No other implementation parallelism assumed.

## Requirement and acceptance coverage

| Requirement | Tasks |
| --- | --- |
| FR-001 | T003, T009, T015, T017, T021, T022, T023 |
| FR-002 | T004, T005, T006, T011, T012, T014, T017, T018, T019, T020, T021, T024, T026, T027, T054 |
| FR-003 | T008, T009, T010, T015, T020, T021, T025, T028, T029, T030, T031, T044, T049 |
| FR-004 | T009, T015, T022, T034, T035, T037, T040 |
| FR-005 | T004, T005, T008, T010, T011, T016, T028, T029, T034, T035, T036, T037, T038, T039, T049 |
| FR-006 | T036, T037, T038, T039, T054 |
| FR-007 | T007, T008, T010, T041, T042, T043, T044, T045, T046, T047, T048 |
| FR-008 | T006, T007, T012, T014, T024, T026, T027, T041, T043, T044, T045, T047, T048, T054 |
| FR-009 | T007, T008, T010, T015, T024, T025, T026, T027, T028, T029, T030, T032, T033, T047 |
| FR-010 | T003, T011, T012, T016, T018, T019, T028, T029, T031, T034, T036, T039, T040, T054 |
| FR-011 | T017, T022, T023, T028, T029, T033, T035, T037, T038, T042, T046, T048 |
| FR-012 | T022, T023, T033, T042, T045, T046, T050, T055 |
| FR-013 | T004, T005, T006, T014, T018, T019, T020, T025, T032, T038, T041, T043, T049 |
| FR-014 | T013, T031, T032, T036, T040, T049, T051, T056 |
| FR-015 | T001, T002, T011, T012, T028, T039, T047, T052, T053, T054, T055 |
| FR-016 | T001, T050, T051, T055, T056 |
| SC-001 | T017, T018, T019, T021, T020, T023, T026, T027, T054 |
| SC-002 | T008, T010, T028, T029, T034, T035, T036, T037, T039, T041, T042, T045, T047 |
| SC-003 | T039, T054 |
| SC-004 | T008, T009, T010, T015, T025, T026, T027, T028, T029, T030, T041, T044, T047, T048 |
| SC-005 | T023, T033, T042, T046, T050 |
| SC-006 | T028, T029, T049, T052, T053, T054, T055, T056 |

AC-01–04: US1; AC-05–09: US2; AC-10–12: US3; AC-13–16: US4, with sharing/financial regression checks.

All 16 FRs, 16 ACs and 6 SCs are mapped. Live/native visibility remains a separate optional evidence category.

## Implementation strategy and Definition of Done

Implement incrementally in dependency order. MVP is US1 onboarding WITH US4 safety prerequisites;
verify through controlled ports/SDK/private DB first. Add US2 financial routing, then US3 sharing.
Do not activate incomplete guild behavior or deploy any stage automatically. Source005 rules remain
authoritative for financial math;006 owns spaces/access, not another payment/accounting engine.

Done requires all required tasks verified, all 16 ACs evidenced, four Cargo gates and both isolated
DB suites green, no migration/history rewrite, no cross-feature SQL/cycle, safe credentials/logging,
accurate setup/recovery/ADR/handoff and a reviewed diff. Optional live/native-visibility/hosted CI may
remain unverified only when explicitly recorded; fixture success never substitutes for those claims.
No task is complete during generation. No implementation/test execution is claimed by this checklist.

## Authorized analysis remediation

I1: management/recovery authority is separate from finance-ready authority. C1: T028/T029 implement
and test full Spaces authorization before consumers. U1: T041/T043/T045 specify abandoned prepared
expiry, atomic slot release and dispatched protection. D1: ports expand with concrete implementations;
T020 SDK mutations precede T021 provisioning and T031 root wiring. All56 tasks remain unchecked.
