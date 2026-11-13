# Tasks: Conversational Personal Finance

Input: [spec](spec.md), [plan](plan.md), [research](research.md), [model](data-model.md),
[contract](contracts/finance.md), [guide](quickstart.md). Full workflow authorized by maintainer.
Tests required. Write meaningful failing tests before paired code; mark complete only after verification.
Private tests may be split by responsibility/250-line guidance. Do not use real .env.local or Discord.

## Phase 1: Setup

- [ ] T001 Recording clean Git/source/ignore baseline in specs/005-conversational-finance/tasks.md; preserve real .env.local and generated tooling; verify previous69-test status (FR-014).
- [ ] T002 Adding reviewed SQLx0.9/Chrono0.4.45 minimal features to Cargo.toml/Cargo.lock; verify locked compilation and feature tree (after T001; FR-013).

## Phase 2: Foundational

- [ ] T003 Writing amount/phrase/date/field-redaction tests in src/features/ledger/domain/tests.rs before implementation; cover missing/multiple amounts and expected income (after T002; FR-002/004/012).
- [ ] T004 Defining validated redacted model/error/parser in src/features/ledger/domain/ and wiring src/features/mod.rs; verify T003 and full-u64 reuse, bounded fields and finite aliases (after T003; FR-002/004).
- [ ] T005 Writing chronology/transfer/report math tests in src/features/ledger/domain/posting_tests.rs and summary_tests.rs; cover overflow/backdated overdraft/carryover/zero income (after T004; FR-008/010).
- [ ] T006 Implementing pure historical posting validation and monthly calculations in src/features/ledger/domain/posting.rs and summary.rs; verify T005 with core arithmetic and checked u128 totals (after T005; FR-004/008/010).
- [ ] T007 Creating migrations/0001_personal_journal.sql with whole/range CHECKs, owner/fund/draft/source uniqueness, references, revisions and immutable-entry trigger; verify with disposable PostgreSQL (after T006; FR-005/006/012).
- [ ] T008 Implementing bounded pool/embedded migration lifecycle in src/platform/database.rs and optional redacted DB setting in src/config/discord.rs; offline ignores it and missing URL preserves readiness-only (after T007; FR-001/013/014).

## Phase 3: US1 — Private review and recording (P1)

Independent check: owner-only preview/edit/cancel/confirm boundaries with no unconfirmed money effects.

- [ ] T009 [US1] Defining one real JournalStore port and owner-authorized Journal application facade in src/features/ledger/application/; actual owner/channel/resource scope checked for every operation; verify pure denial tests (after T008; FR-003/004/009).
- [ ] T010 [US1] Writing private durable-draft tests in src/features/ledger/infrastructure/tests.rs for same-source replay, edit revision, channel isolation, expiry/cancel and safe errors (after T009; FR-003/005/009).
- [ ] T011 [US1] Implementing PgStore mapping and locked draft lifecycle in src/features/ledger/infrastructure/{mod,mapping,drafts}.rs; verify T010 without interpolated SQL or raw error sources (after T010; FR-003/005/009).
- [ ] T012 [US1] Writing actual SDK view/modal/component fixture tests in src/features/ledger/presentation/tests.rs; verify five fields, unique IDs, bounds, private flags, empty mentions and DM-only auth selection (after T011; FR-003/009/012).
- [ ] T013 [US1] Implementing Vietnamese preview/buttons/modal/result builders in src/features/ledger/presentation/{views,modals}.rs; verify T012 and no financial payload in IDs (after T012; FR-003/011/012).
- [ ] T014 [US1] Implementing authenticated DM message/help/draft flow in src/features/ledger/presentation/messages.rs; ignore public/bot messages and ambiguous unsupported chatter, preserve original note (after T013; FR-001/002/003/011).
- [ ] T015 [US1] Implementing guarded button/modal action flow in src/features/ledger/presentation/components.rs; bounded lookup before initial modal, private defer before mutation, current revision/expiry/channel and no second financial attempt after send failure (after T014; FR-003/005/009/012).

## Phase 4: US2 — Durable income and fund allocation (P1)

Independent check: synthetic income/opening/transfer histories and real concurrent durable posting.

- [ ] T016 [US2] Writing real PostgreSQL atomic confirmation tests in src/features/ledger/infrastructure/commit_tests.rs before posting implementation; duplicate race, cross-owner, negative history, rollback and restart (after T015; FR-005/008/009).
- [ ] T017 [US2] Implementing owner-serialized confirmation in src/features/ledger/infrastructure/commit.rs; committed replay first, authoritative fund/history validation, single transfer entry and draft/audit update in one transaction; verify T016 (after T016; FR-005/006/008).
- [ ] T018 [US2] Implementing zero/default/additional funds and authoritative balance queries in src/features/ledger/infrastructure/queries.rs; one opening per fund, immutable entries, synthetic tests (after T017; FR-006/007/008).
- [ ] T019 [US2] Connecting Vietnamese fund list/create and received/opening/transfer commands in src/features/ledger/presentation/messages.rs; verify income16m/consumption6m/reserves5m example and no expected-income posting (after T018; FR-006/007/011).

## Phase 5: US3 — Accurate monthly view (P2)

Independent check: month fixtures reconcile opening, actual income, expenses, allocations and closing.

- [ ] T020 [US3] Writing durable monthly-query fixtures in src/features/ledger/infrastructure/report_tests.rs for carryover, in-period initialization, category sums, later-date exclusion and integer ratios (after T019; FR-010).
- [ ] T021 [US3] Implementing owner-scoped monthly queries in src/features/ledger/infrastructure/queries.rs using pure summary rules; verify T020 and reconciliation (after T020; FR-010).
- [ ] T022 [US3] Connecting month/help responses in src/features/ledger/presentation/messages.rs and views.rs; display explicit virtual/manual labels, no bank-proof claim, safe zero-income percentage (after T021; FR-010/011).
- [ ] T023 [US3] Wiring concrete FinanceEvents in src/features/ledger/presentation/mod.rs beside Poise via src/platform/discord/client.rs; share Arc owner/accepting state, DIRECT_MESSAGES only when finance enabled, bounded sends and pool close via src/runtime.rs; verify no feature cycle (after T022; FR-001/004/009/012).

## Phase 6: Polish and cross-cutting verification

- [ ] T024 Extending src/config/discord_tests.rs, tests/discord_entry.rs and existing framework tests for optional DB redaction/offline ignoring/finance-disabled readiness; verify prior suites remain intact (after T023; FR-001/012/013).
- [ ] T025 Adding scripts/test-postgres.sh to create/stop a private disposable local cluster and run explicitly ignored DB tests; require synthetic fixture, no existing environment URL; verify safe cleanup (after T024; FR-013/014).
- [ ] T026 Completing DB CHECK/immutable-entry/rollback/concurrency/restart tests in src/features/ledger/infrastructure/; execute them against isolated PostgreSQL18 and record pass/ignored distinctions (after T025; FR-005/008/013).
- [ ] T027 Reviewing SDK production routing, timeouts, DB/actor/channel/expiry paths, sensitive-marker diagnostics and exact money range in src/features/ledger/ and src/platform/; fix concrete gaps only (after T026; FR-004/009/012).
- [ ] T028 Updating .github/workflows/ci.yml with a disposable synthetic PostgreSQL integration job while retaining credential-free normal gates and minimal permissions; verify local equivalent, hosted status separately (after T027; FR-013).
- [ ] T029 Updating README.md, .env.example, product/architecture/security/structure docs and ADR0006 with actual phrases/setup/categories/funds/date/recovery/test limits; no real values retained (after T028; FR-014).
- [ ] T030 Running cargo fmt --all -- --check, cargo clippy --all-targets -- -D warnings, cargo test, cargo check and explicit offline run; record actual results in specs/005-conversational-finance/tasks.md (after T029; FR-013).
- [ ] T031 Running scripts/test-postgres.sh after final changes; verify all DB cases, closed pool/server cleanup, migration idempotency and no real services/credentials (after T030; FR-005/013).
- [ ] T032 Reviewing final diff, visibility/dependency/requirement coverage and documentation links; update docs/ai/FEATURE_INDEX.md, SESSION_HANDOFF.md and devlog.md with verified done/partial/blocked/unverified state (after T031; FR-014).

## Dependencies and parallel work

Setup → Foundational → US1 → US2 → US3 → Polish. Task IDs/after clauses define the critical path.
Draft/commit/query tests share migration/port foundations. Do not parallel-edit shared model, root,
query or message files. After T029, independent documentation/link review can accompany read-only
verification; task list deliberately marks no implementation task [P] to avoid unsafe shared-file races.

## Requirement coverage

| Requirement | Tasks | Acceptance |
| --- | --- | --- |
| FR-001 | T008/T014/T023/T024 | AC-05 |
| FR-002 | T003/T004/T014/T019 | AC-01/02/07 |
| FR-003 | T009–T015 | AC-01–04 |
| FR-004 | T003–T006/T009/T023/T027 | AC-05/09 |
| FR-005 | T007/T010/T011/T016/T017/T026/T031 | AC-04/08/14 |
| FR-006 | T007/T016–T019 | AC-06/10 |
| FR-007 | T003/T004/T019 | AC-01/07 |
| FR-008 | T005/T006/T016–T019/T026 | AC-08/09 |
| FR-009 | T009–T015/T016/T023/T027 | AC-03–05 |
| FR-010 | T005/T006/T020–T022 | AC-11–13 |
| FR-011 | T013/T014/T019/T022 | AC-02/10/14 |
| FR-012 | T003/T007/T012/T015/T024/T027 | AC-05/14 |
| FR-013 | T002/T025/T026/T028/T030/T031 | AC-04/09/14 |
| FR-014 | T001/T029/T032 | AC-14 |

SC-001: T003/T004/T010–T015; SC-002: T016/T017/T026; SC-003: T005–T007/T016–T019;
SC-004: T005/T006/T020–T022; SC-005: T009/T012/T015/T024/T027;
SC-006: T025–T032. All fourteen AC mapped; real provider/hosted evidence remains separate.

## Definition of Done

All32 tasks backed by evidence, core/offline/readiness preserved, no raw financial/credential logging,
all four gates pass, local DB race/rollback/restart and whole-u64 constraints verified, UI builders
and scope reviewed, documentation/handoff agree. No production deployment, bank transfer or AI.
Ordinary cargo test may explicitly ignore DB suite only if separately executed and reported.

## Execution evidence

Not implemented at task generation. Quality checklist16/16; analysis runs before code. User authorized
scope/review gates and fixes within this full workflow; no repeated approval is required.
