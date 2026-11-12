---
description: "Financial Core implementation checklist"
---

# Tasks: Financial Core and Domain Invariants

**Input**: [spec](spec.md), [plan](plan.md), [research](research.md), [model](data-model.md),
[contract](contracts/financial-core.md), [quickstart](quickstart.md), and constitution 1.1.0.
**Status**: Complete and locally verified, 2026-11-12; all 26 tasks are checked with evidence below.
**Authorization**: The maintainer invoked $speckit-implement for this task list. Existing user edits
and generated tooling are preserved. No dependency was added.

## Format and Working Rules

Descriptions use gerund-led phrases. [USn] identifies the story; [P] allows parallel work only
after prerequisites. The dependency/verification table supplies completion criteria and traceability
without repeating long descriptions in each task. Phase gates also apply.

All paths are repository-relative. When adding actual modules, wire only their live exports through
src/lib.rs, src/features/mod.rs, src/features/financial_core/mod.rs, and
src/features/financial_core/domain/mod.rs. Keep domain/application paths private; expose the
planned facade. Do not create placeholder modules or change main/bootstrap/config behavior.
Wire test modules into tests/financial_core.rs only when their test cases exist.

Author tests before their new behavior; record the expected failure, then passing results at the
story checkpoint. Test-authoring verification is review plus an expected missing-API/behavior
failure, not a claim of passing functionality. Existing shared behavior may already pass; never
introduce an artificial regression. Use synthetic values and existing std/thiserror only.

Keep files cohesive and within AGENTS.md size guidance. Necessary comments follow its brief
tagged-comment rule. Record task evidence here only after verification; summarize state in the
handoff and milestone in devlog.md without duplicating this checklist.

## Phase 1: Setup — Reviewing the Existing Foundation

Goal: preserve the actual baseline and confirm design authority before source work.

- [X] T001 Reviewing instructions, local edits, and baseline checks in `AGENTS.md`, `src/`, `Cargo.toml`, and `Cargo.lock`.
- [X] T002 Recording the reviewed design decision in `docs/architecture/decisions/0003-financial-core-validation.md` after preimplementation analysis.

Checkpoint: baseline results/local edits are recorded; plan/contract and ADR status are resolved.

## Phase 2: Foundational — Defining Shared Value Boundaries

Goal: provide real categorical errors and typed references without speculative layers.

- [X] T003 Defining categorical `CoreError` and `ReferenceKind` in `src/features/financial_core/domain/error.rs` with live facade exports.
- [X] T004 Adding references with “private owned String” values that “reject empty or Unicode-whitespace-only input” in `src/features/financial_core/domain/references.rs`.

Checkpoint: four reference roles and safe errors are usable; no business workflow or provider exists.

## Phase 3: US1 — Representing Amounts Exactly (P1, MVP)

Goal: exact whole-đồng VND construction, comparison, and checked arithmetic.
Independent test: AC-01–03 pass without balances, intents, or services.

- [X] T005 [US1] Writing exact-value and rejection cases in `tests/financial_core/money.rs` and wiring `tests/financial_core.rs`.
- [X] T006 [US1] Implementing `Money` within “0–18,446,744,073,709,551,615 inclusive” in `src/features/financial_core/domain/money.rs`.
- [X] T007 [US1] Verifying canonical parsing, checked arithmetic, and AC-01–03 through `tests/financial_core/money.rs`.

Checkpoint: the public Money contract passes, with no rounding, wrapping, saturation, or conversion.

## Phase 4: US2 — Protecting Virtual Balances (P1)

Goal: pure balance previews that reject overdraft and invalid changes.
Independent test: AC-04–06 pass using monetary values alone.

- [X] T008 [US2] Writing immutable-preview and boundary cases in `tests/financial_core/balance.rs`.
- [X] T009 [US2] Implementing virtual balances and changes (“Increase or Decrease”; changes: “Strictly positive amount; zero is invalid”) in `src/features/financial_core/domain/balance.rs`.
- [X] T010 [US2] Verifying no-overdraft previews and AC-04–06 through `tests/financial_core/balance.rs`.

Checkpoint: exact depletion succeeds; overflow/overdraft fail without changing the supplied balance.

## Phase 5: US3 — Recognizing Safe Retries and Conflicts (P2)

Goal: compare supplied prior context before fresh spending-capacity checks.
Independent test: AC-07–10 pass with no registry, stored entries, or concurrent workers.

- [X] T011 [US3] Writing replay/context cases in `tests/financial_core/replay.rs` with reusable fixtures in `tests/financial_core/support.rs`.
- [X] T012 [US3] Defining named intent fields and “HandoffOnly versus ManualConfirmation” in `src/features/financial_core/domain/intent.rs`.
- [X] T013 [US3] Implementing ordered validation and candidate “private fields” in `src/features/financial_core/application.rs`.
- [X] T014 [US3] Verifying replay identity, semantic conflicts, and AC-07–10 through `tests/financial_core/replay.rs`.

Checkpoint: replay borrows its coherent prior result even with current balance zero; no effect is applied.

## Phase 6: US4 — Preserving Origin and Evidence Meaning (P2)

Goal: preserve virtual provenance while rejecting handoff-as-completion and unsafe diagnostics.
Independent test: AC-11–13 pass; evidence and errors never assert bank proof or disclose payloads.

- [X] T015 [US4] Writing provenance, evidence, and redaction cases in `tests/financial_core/evidence.rs`.
- [X] T016 [US4] Ensuring redacted diagnostics and coherent provenance across `src/features/financial_core/domain/` and `src/features/financial_core/application.rs`.
- [X] T017 [US4] Verifying reference rejection, private-result boundaries, and AC-11–13 through `tests/financial_core/evidence.rs`.

Checkpoint: complete public validation behavior passes with no bank-verification or commit guarantee.

## Phase 7: Polish — Reviewing Delivery and Verification

Goal: align documentation and verify the final combined tree without scope expansion.

- [X] T018 [P] Updating ownership and public-facade guidance in `docs/architecture/overview.md` and `docs/development/project-structure.md`.
- [X] T019 [P] Updating implemented/planned status in `README.md`, `docs/product/overview.md`, and `docs/product/feature-map.md`.
- [X] T020 [P] Updating validation trust limits and privacy guidance in `docs/security/security-baseline.md`.
- [X] T021 Running `cargo fmt --all -- --check` against `src/` and `tests/`.
- [X] T022 Running `cargo clippy --all-targets -- -D warnings` for `Cargo.toml`.
- [X] T023 Running public-contract and full `cargo test` checks for `tests/financial_core.rs` and `tests/bootstrap.rs`.
- [X] T024 Running `cargo check` and checking unchanged `Cargo.toml`/`Cargo.lock` resolution.
- [X] T025 Inspecting consistency, privacy, imports, and scope across `specs/002-financial-core/`, `src/`, and the repository diff.
- [X] T026 Closing verified progress in `specs/002-financial-core/tasks.md`, `docs/ai/SESSION_HANDOFF.md`, `docs/ai/FEATURE_INDEX.md`, and `devlog.md`.

## Dependencies and Verification

Every row describes evidence required before its task is checked. Story checkpoints must pass
before moving on. No command/result below has been executed or claimed by task generation.

| Task | Depends on | Verification / requirement coverage |
| --- | --- | --- |
| T001 | None | Read constitution, active artifacts and related code; record baseline four Cargo checks and user edits outside temporary repo artifacts; preserve hashes/diffs. FR-013/014/016. |
| T002 | T001 | Run $speckit-analyze; resolve critical/high findings through owning workflows before code. Adopt ADR only under implementation authorization, retaining domain-only scope. FR-001/012/014. |
| T003 | T002 | Use the complete contract error taxonomy, no raw payload/source strings; wire only implemented modules. cargo check --lib passes; no new dependency. FR-009/014/015. |
| T004 | T003 | Test OwnerRef/TargetRef/OriginRef/IdempotencyKey constructors: blank/Unicode whitespace rejected; other bytes/case preserved, private String and deliberate accessors; no generation/authentication. cargo test --lib passes. FR-006/009/015; AC-11. |
| T005 | T004 | Cover the full money grammar/error order, leading zeros, zero, negative, fractions, currency, MAX/MAX+1 and arithmetic; record initial targeted result. FR-002/003/016. |
| T006 | T005 | Match contract Money::parse/whole_dong/comparison/checked_add/checked_sub; private u64; exact VND, no float/implicit narrowing. Wire Money exports. FR-002/003/009. |
| T007 | T006 | cargo test --test financial_core money passes; 1000+250, MAX+1 string/addition, 0−1 and constructor errors match; sensitive input is not retained. FR-002/003/016; AC-01–03; SC-001. |
| T008 | T007 | Cover unchanged snapshots, exact depletion, excess decrease, zero change and maximum overflow; record initial result. FR-004/005/016. |
| T009 | T008 | Private VirtualBalance/BalanceChange values; no mutators/bank type; preview maps decrease underflow to InsufficientFunds and overflow to ArithmeticOutOfRange. Wire exports. FR-004/005/009. |
| T010 | T009 | cargo test --test financial_core balance passes; small exhaustive 0–32 previews agree with exact arithmetic, alongside MAX cases; original values unchanged. FR-004/005/016; AC-04–06; SC-002. |
| T011 | T010 | Cover None/new, prior replay at current zero, every semantic-field conflict, owner/key mismatch and precedence; use synthetic typed fixtures. Record initial result. FR-007/008/016. |
| T012 | T011 | Use owner/target/origin/key/change/completion_basis typed fields only; no timestamp, bank proof, duplicate DTO or builder. Candidate snapshots must not alias mutable caller intent. FR-006/008/011. |
| T013 | T012 | Implement the complete contract: scope mismatch → semantic conflict/replay → fresh basis → capacity. Eligible owns coherent intent/before/proposed; Replay borrows prior; no public candidate construction or I/O/state. FR-001/007–010/012/013. |
| T014 | T013 | cargo test --test financial_core replay passes; verify borrowed prior identity, unchanged snapshots, fresh-None repeat limits, and conflict/mismatch precedence over capacity/basis failures. FR-007/008/016; AC-07–10; SC-003. |
| T015 | T014 | Cover all reference roles, handoff-only rejection, manual virtual result, origin preservation, caller mutation isolation and nested Debug/error-source markers. Record initial result. FR-006/010/011/015/016. |
| T016 | T015 | Ensure all sensitive Money/reference/balance/intent/candidate/decision formats are redacted or absent; add redacted Debug where contract tests need it. No raw Display, core logs, bank-proof variant, result setter or restoration bypass. FR-006/010/011/015. |
| T017 | T016 | cargo test --test financial_core evidence passes; inspect private fields/no public unchecked constructor and origin/virtual accessor results. Run full financial_core target. FR-010/011/015/016; AC-11–13; SC-004. |
| T018 | T017 | Document actual facade/application/domain boundaries and meaningful modules; keep future layers absent; link authoritative contract/ADR. FR-014; SC-006. |
| T019 | T017 | Describe implemented pure core separately from unimplemented funds/ledger/payment workflows; commands/links match actual package. FR-001/016; SC-005/006. |
| T020 | T017 | Document owner/prior trust, future atomic effect/origin/key/audit commit, full-range storage, input-limit and verification obligations; no current durable/auth/bank guarantee. FR-010/011/012/015; SC-004/006. |
| T021 | T018, T019, T020 | Exact formatting command exits 0 on final source/tests. FR-016; SC-005. |
| T022 | T021 | Exact all-targets Clippy command exits 0 with zero warnings; no suppressions weakening agreed rules. FR-014/016; SC-005. |
| T023 | T022 | cargo test --test financial_core and cargo test pass; all 13 AC and existing bootstrap tests run without services/credentials. Label platform/CI limits honestly. FR-016; SC-001–005. |
| T024 | T023 | cargo check exits 0; manifest/lockfile hashes match T001 baseline; CI discovers tests with existing commands. FR-013/014/016; SC-005. |
| T025 | T024 | Run read-only $speckit-analyze, review full diff and git diff --check; inspect privacy/source chains, exact facade exports, std/thiserror-only domain and pure application. No cycles, floats, state, external services, credentials, bank proof or scope creep. FR-001/012–016; SC-006. |
| T026 | T025 | Record actual evidence/remaining gaps, reconcile feature metadata and proposed/accepted ADR status, update devlog/handoff, verify links and whitespace. No false hosted-CI or business completion claims. FR-016; SC-005/006; full-feature completion. |

## Dependencies and Parallel Opportunities

```text
T001 → T002 → T003 → T004
→ T005 → T006 → T007 [US1]
→ T008 → T009 → T010 [US2]
→ T011 → T012 → T013 → T014 [US3]
→ T015 → T016 → T017 [US4]
→ {T018, T019, T020} → T021 → T022 → T023 → T024 → T025 → T026
```

Setup/foundation block all stories. US2 needs Money; US3 needs validated references/changes and
balance preview; US4 verifies the completed decision/evidence model. Their tests remain independently
runnable against the public facade. Shared exports/test wiring make source edits sequential.
US1, US2 and US3 have no safe task-level parallel batch; their table-driven cases may run concurrently
because the core has no global state. US4 also stays sequential while hardening shared types.
T018–T020 can run together after T017: distinct document sets, no unfinished dependency or code edit.
No same-file edits are marked parallel. [P] does not authorize agent delegation by itself.

## Acceptance and Requirement Coverage

| Acceptance | Implementation | Verification |
| --- | --- | --- |
| AC-01–03 | T003, T006 | T005, T007, T023 |
| AC-04–06 | T009 | T008, T010, T023 |
| AC-07–10 | T004, T012, T013 | T011, T014, T023 |
| AC-11 | T004 | T004, T015, T017, T023 |
| AC-12–13 | T012, T013, T016 | T015, T017, T023 |

| Requirement / outcome | Tasks |
| --- | --- |
| FR-001 | T002, T013, T019, T025 |
| FR-002, FR-003 | T005–T007 |
| FR-004, FR-005 | T008–T010 |
| FR-006 | T004, T012, T015, T016 |
| FR-007, FR-008 | T011–T014 |
| FR-009 | T003, T004, T006, T009, T013 |
| FR-010 | T013, T015–T017, T020 |
| FR-011 | T012, T013, T015–T017, T020 |
| FR-012 | T002, T013, T020, T025 |
| FR-013 | T001, T013, T024, T025 |
| FR-014 | T001–T003, T018, T022, T024, T025 |
| FR-015 | T003, T004, T015–T017, T020, T025 |
| FR-016 | T005, T007, T008, T010, T011, T014, T015, T017, T021–T024 |
| SC-001 | T005–T007, T023 |
| SC-002 | T008–T010, T023 |
| SC-003 | T011–T014, T023 |
| SC-004 | T015–T017, T020, T023 |
| SC-005 | T019, T021–T024 |
| SC-006 | T018–T020, T025, T026 |

## Implementation Strategy and Definition of Done

Deliver US1 as the smallest useful increment, then US2, US3 and US4 in order. Review the exact
contract at each checkpoint; do not create a service/journal to demonstrate these domain rules.
Commit coherent verified increments only within the active authorization, preserving user edits.

Completion requires all 26 tasks verified, all 13 AC automated, all four Cargo gates passing,
unchanged dependency resolution, private coherent candidates, safe diagnostics, accurate docs/
ADR/devlog/handoff, and no critical/high consistency issue. No unchecked task is called complete.
No monetary state is mutated or persisted. No journal, registry, authentication implementation,
provider, BankVerified value, payment handoff, fund CRUD, allocation, scheduling, QR/OCR/AI,
financial migration or deployment belongs to this feature. Caller-supplied prior context cannot
prove durable/concurrent exactly-once commit; future persistence must enforce that obligation.

## Implementation Evidence

| Task | Verified result |
| --- | --- |
| T001 | Read instructions/design/current code and captured user edits/config hashes in /tmp. Baseline Clippy/test/check passed (10 tests); fmt found only the existing leading blank in src/lib.rs, scheduled for feature wiring without restoring removed comments. |
| T002 | Reviewed spec/plan/task consistency against constitution; prior read-only analysis has no findings. Explicit implementation request authorizes the task list; ADR 0003 accepted with scope/proof limits unchanged. |
| T003 | cargo check --lib --locked passed; complete payload-free error taxonomy and real private-module/facade wiring added using existing thiserror only. |
| T004 | Tests first failed with absent reference types; then cargo test --lib passed all 8 tests. Four role types reject Unicode blanks, preserve bytes/case, expose borrowed accessors and redact Debug; no ID generation/authentication. |
| T005 | Public money contract tests authored before implementation; targeted run failed specifically because Money was absent. Covers canonical grammar, error precedence, exactness, comparison, leading zeros and bounds. |
| T006,T007 | Money implemented with private u64, exact VND grammar and checked arithmetic. All 3 public money tests pass, covering AC-01–03, MAX/MAX+1, underflow, comparison and input classification. No new dependency. |
| T008 | Balance contract tests authored first; targeted run fails specifically on absent balance/change/direction APIs. Cases include immutable snapshots, depletion, overdraft, zero change, MAX overflow and exhaustive small ranges. |
| T009,T010 | All 3 balance contract tests pass: unchanged input/change values, exact depletion, insufficient funds, zero changes and MAX overflow. Exhaustive initial balances 0–32 and positive changes 1–32 match exact arithmetic; previews have no state mutation. |
| T011 | Replay/fixture tests authored before intent/application APIs; targeted run fails on missing types/functions. Cases cover all semantic fields, owner/key mismatch, current-zero replay and absence not proving durable uniqueness. |
| T012 | cargo check --lib passes with named intent fields built only from validated values and two evidence meanings. No timestamp, bank proof, duplicate DTO, builder or source-state dependency. |
| T013,T014 | All 5 replay contract tests passed. Pure application checks owner/key then exact meaning, borrows matching prior before fresh capacity/evidence checks, and only creates owned coherent private-field candidates for fresh manual input. No state/lookup/commit; input mutation cannot alter captured provenance. |
| T015 | All 4 evidence/privacy cases pass for behavior already implemented with safe value types: blank role references, handoff rejection, virtual manual results, retained origin and nested Debug/error-source redaction. No artificial regression introduced. |
| T016,T017 | All 15 Financial Core contract tests passed, including all 13 AC groups. Sensitive scalar/reference/record/decision formats and error sources are redacted; no extra hardening abstraction needed. Six external rustc probes in /tmp correctly rejected candidate/value forging, private domain/application imports and a nonexistent BankVerified variant. |
| T018,T019,T020 | Reviewed updated architecture/structure/coding guidance, README/product roadmap and threat/trust documentation against implemented facade and contract. All local links resolve; journal/commit/auth/bank proof remain explicitly absent, full-range persistence and future enforcement obligations documented. |
| T021 | cargo fmt --all -- --check passed on final source/tests; only inherited leading blank in src/lib.rs removed while adding feature export, all other pre-existing source edits preserved. |
| T022 | cargo clippy --all-targets -- -D warnings passed with zero warnings; no lint weakening or suppression. |
| T023 | Targeted and full cargo test passed: 15 Financial Core contract tests plus 2 new reference unit tests and all 10 existing foundation tests (27 total, zero failures/ignored cases), no services or credentials. |
| T024 | cargo check passed. Manifest/lockfile hashes and existing bootstrap/main/config/instructions/constitution content match baseline; library change is only its required feature export and leading-blank normalization. Existing CI cargo test automatically discovers the new target. |
| T025 | Read-only artifact/source analysis found 16 FR, 6 SC and 13 AC covered, valid task order, private facade boundaries and no critical/high issue. Full new source/test bodies, existing diffs, privacy probes, imports, file sizes and scope reviewed; git diff --check passed. Only final metadata closeout remains. |
| T026 | Reconciled implemented status in spec/plan/model/research/contract/quickstart, index/devlog/handoff and accepted ADR; all 85 local documentation links resolve, predecessors verified, baseline content preserved, whitespace clean. No after-implement extension hooks are registered. Hosted CI remains unverified; no implementation blocker remains. |
