---
description: "Owner identity and access implementation checklist"
---

# Tasks: Owner Identity and Access Boundaries

**Input**: [spec](spec.md), [plan](plan.md), [research](research.md), [model](data-model.md),
[contract](contracts/owner-access.md), [quickstart](quickstart.md), constitution 1.1.0.
**Status**: Task generation only, 2026-11-12. All 23 tasks are unchecked; no implementation.
**Authorization**: The maintainer requested tasks from the plan. Implementation requires its own
instruction. Preserve existing user edits, generated tooling, dependency resolution and bootstrap.

## Format and Path Conventions

Descriptions are brief gerund-led phrases. Paths are repository-relative. [USn] maps to the
specification's story; [P] is safe only after listed dependencies and phase gates are satisfied.
The dependency/verification table is part of every task's completion criteria and traceability.

Only wire modules/exports when functionality exists. Identity has private domain.rs and
application.rs behind mod.rs, registered in src/features/mod.rs. src/lib.rs already exports
features; do not change it or existing Financial Core just to add identity. Tests use the public
facade and synthetic constructor fixtures, without state/provider mocks or parent env mutation.

Tests are mandatory under FR-013. Author tests before new behavior and record the expected missing
API/behavior failure, then passing evidence. Existing redaction/value behavior may already pass;
never introduce an artificial regression. Do not mark tasks complete without their verification.
Keep new files cohesive and within AGENTS.md size guidance; no speculative split or abstraction.

## Phase 1: Setup — Reviewing the Approved Foundation

Goal: establish the actual baseline and design authority without reinitializing the Rust package.

- [ ] T001 Reviewing instructions, user edits and baseline checks in `AGENTS.md`, `src/`, `Cargo.toml` and `Cargo.lock`.
- [ ] T002 Recording the reviewed policy decision in `docs/architecture/decisions/0004-owner-access-policy.md` after consistency analysis.

Checkpoint: source/user-edit baseline captured; spec/plan/contract and design approval reviewed.

## Phase 2: Foundational — Defining Safe Policy Vocabulary

Goal: shared categorical errors and explicit binding state, ready for account/actor behavior.

- [ ] T003 Defining errors (“static messages, no source payload”) as “InvalidAccountId or Denied” and states as “Enabled or Disabled” in `src/features/identity/domain.rs`, with live exports in `src/features/identity/mod.rs` and `src/features/mod.rs`.

Checkpoint: only real error/state modules are exported; cargo check --lib passes without new crates.

## Phase 3: US1 — Recognizing the Configured Owner (P1, MVP)

Goal: resolve exactly one explicitly configured enabled owner and deny all absent/unknown actors.
Independent test: AC-01–04 pass without scope policy, live Discord, stored accounts or resources.

- [ ] T004 [US1] Writing canonical-account and owner-compatibility cases in `tests/identity/accounts.rs`, wiring `tests/identity.rs`.
- [ ] T005 [US1] Implementing accounts (“Canonical positive ASCII decimal, no leading zeros, bounded by u64::MAX; checked parse only”) and actors (“construction does not verify provenance”) in `src/features/identity/domain.rs`.
- [ ] T006 [US1] Writing owner-resolution cases in `tests/identity/resolution.rs`, with synthetic fixtures in `tests/identity/support.rs` and wiring in `tests/identity.rs`.
- [ ] T007 [US1] Implementing binding (“Private account, OwnerRef, BindingState”; “constructor owns owner, values remain immutable”) and borrowed owner resolution in `src/features/identity/application.rs`, exporting through `src/features/identity/mod.rs`.
- [ ] T008 [US1] Verifying exact binding, denial defaults and AC-01–04 through `tests/identity/accounts.rs` and `tests/identity/resolution.rs`.

Checkpoint: canonical account/actor values and current binding resolution pass; no inferred owner.

## Phase 4: US2 — Restricting Access to the Owner's Scope (P1)

Goal: allow only the resolved owner's explicit authoritative scope; use current supplied binding.
Independent test: AC-05–08 pass using the US1 values/resolution, without actual resources or lookup.

- [ ] T009 [US2] Writing matching, foreign, missing and revoked-scope cases in `tests/identity/scope.rs`, wiring `tests/identity.rs`.
- [ ] T010 [US2] Implementing scoped authorization (“absence denies; no lookup or resource entity”) in `src/features/identity/application.rs`, exporting through `src/features/identity/mod.rs`.
- [ ] T011 [US2] Verifying owner isolation, current-binding evaluation and AC-05–08 through `tests/identity/scope.rs`.

Checkpoint: no scope claim rescues an unknown actor; disabled/absent binding denies subsequent calls.

## Phase 5: US3 — Keeping Trust and Privacy Explicit (P2)

Goal: reusable deterministic policy with safe diagnostics and honest caller trust obligations.
Independent test/review: AC-09–12 pass without a provider, transport dispatch or persistent grant.

- [ ] T012 [US3] Writing diagnostic and channel-independent policy cases in `tests/identity/privacy.rs`, wiring `tests/identity.rs`.
- [ ] T013 [US3] Ensuring redacted values, results and payload-free errors in `src/features/identity/domain.rs` and `src/features/identity/application.rs`.
- [ ] T014 [US3] Documenting authenticated-actor, trusted-binding and authoritative-scope obligations in `docs/security/security-baseline.md`, reviewing `specs/003-owner-access/contracts/owner-access.md`.

Checkpoint: full identity target passes; AC-10 guidance review is recorded separately from tests.
AC-12 requires behavior evidence and source/API review, completed during final verification.

## Phase 6: Polish — Verifying Delivery and Documentation

Goal: accurate implemented/planned status, all required checks, privacy probes and consistency review.

- [ ] T015 [P] Updating identity ownership and dependency direction in `docs/architecture/overview.md` and `docs/development/project-structure.md`.
- [ ] T016 [P] Updating policy-only status and development guidance in `README.md`, `docs/product/overview.md` and `docs/product/feature-map.md`.
- [ ] T017 Running `cargo fmt --all -- --check` against `src/` and `tests/`.
- [ ] T018 Running `cargo clippy --all-targets -- -D warnings` for `Cargo.toml`.
- [ ] T019 Running targeted and full `cargo test` checks for `tests/identity.rs`, `tests/financial_core.rs` and `tests/bootstrap.rs`.
- [ ] T020 Running `cargo check` and checking unchanged `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` and `.github/workflows/ci.yml` compatibility.
- [ ] T021 Verifying private construction and imports for `src/features/identity/` through temporary external compiler probes.
- [ ] T022 Reviewing requirement coverage, trust limits and scope across `specs/003-owner-access/`, `src/` and the repository diff.
- [ ] T023 Closing verified progress in `specs/003-owner-access/tasks.md`, `docs/ai/SESSION_HANDOFF.md`, `docs/ai/FEATURE_INDEX.md` and `devlog.md`.

## Dependencies and Verification Criteria

Every task must satisfy its row before completion; phase/story checkpoints apply in addition.
Commands below describe future implementation evidence. None was executed by task generation.

| Task | Prerequisites | Completion evidence and requirement coverage |
| --- | --- | --- |
| T001 | None | Read session instructions, constitution, spec/design, existing code and active checklists; capture existing user edits/config hashes outside the repository. Record actual fmt/Clippy/test/check baseline; preserve unrelated edits. FR-011/013. |
| T002 | T001 | Run $speckit-analyze before code; resolve critical/high issues through owning workflows. Adopt ADR only under subsequent implementation authorization, retaining single-owner pure policy and proof limits. FR-008/011/012. |
| T003 | T002 | Payload-free thiserror AccessError has exactly InvalidAccountId/Denied, static messages and no source payload; BindingState has Enabled/Disabled. Export only implemented items; cargo check --lib passes. FR-001/006/010/011. |
| T004 | T003 | Review public tests for 1, 42, MAX, MAX+1, zero/leading zeros, empty, signs, ASCII/Unicode whitespace, Unicode digits, decimals, exponents, separators, mentions/names and >20-byte input. Existing OwnerRef rejects Unicode blanks and preserves other bytes/case. Confirm expected absent-API failure. FR-002/003/013; AC-04. |
| T005 | T004 | DiscordAccountId privately wraps NonZeroU64; accepts 1–20 digits, first 1–9, checked u64 bound. TrustedActor privately wraps validated account and exposes construction only publicly; both Copy/Eq and redacted Debug. Narrow crate-visible account accessor only; no raw-number accessor/Display, normalization or credential claim. Account tests pass. FR-002/008/010/011. |
| T006 | T005 | Tests cover matching enabled mapping, mismatched/absent actor, absent/disabled binding, repeated current inputs and unchanged values. Fixtures use public validated constructors and synthetic identities. Confirm expected missing binding/resolution failure. FR-001/004/006/009/013; AC-01–04. |
| T007 | T006 | Implement contract signatures exactly: binding owns private account/OwnerRef/state; resolve_owner borrows the bound OwnerRef or returns Denied. Application imports only Financial Core facade, domain imports no feature; no owner re-export/duplicate validation. No binding getters/setters/Clone/serialization, state, I/O or fallback. Targeted resolution tests pass. FR-001/003/004/006/008/009/011. |
| T008 | T007 | cargo test --test identity accounts and resolution filters pass; same owner returned repeatedly, every denial has no payload, exact owner semantics and unchanged inputs retained. Review result lifetime tied only to binding. FR-001–004/006/009/013; AC-01–04; SC-001/003. |
| T009 | T008 | Write absent/matching/foreign scope and absent/matching/nonmatching actor with absent/enabled/disabled binding matrix. Include correct owner claim from unknown actor and success then changed/disabled/absent binding. Confirm missing authorize_scope failure. FR-005/007/013; AC-05–08. |
| T010 | T009 | Resolve current owner first, then require Option<&OwnerRef> scope equality. Return bound owner reference, never scope input; all failures Denied. No prior-success input, grant wrapper, cache, lookup or mutations. Targeted scope tests pass. FR-005–009/011. |
| T011 | T010 | cargo test --test identity scope passes full policy matrix, exact case/padding mismatch, unchanged inputs and fresh revocation checks. Compare returned reference with resolve_owner result; no owner clone or current mapping override. FR-005–007/009/013; AC-05–08; SC-002/003. |
| T012 | T011 | Use valid numeric account markers and nonblank owner markers in Debug/pretty/nested Result/error-source cases. Repeat same inputs for message/command/component-labelled cases without new dispatch API. Review tests; record expected failures or already-safe behavior honestly. FR-006/009/010/013; AC-09/11/12. |
| T013 | T012 | Custom sensitive Debug and existing redacted OwnerRef make all formats safe; errors have None source/static categories. Run cargo test --test identity, including privacy cases; no blanket logging/instrumentation. Do not add hardening frameworks or fake regressions if prior code already passes. FR-006/009–011/013; AC-09/11; SC-004. |
| T014 | T013 | Review all six contract caller obligations: authenticated provider actor, operator binding, authoritative ownership, current per-use-case/replay checks, uniform safe denial, reusable adapter boundaries. Security doc describes these as future integration obligations, policy as implemented only after passing code checks. No live auth/lookup/revocation claim; AC-10 review evidence distinct from tests. FR-008/012; AC-10/12; SC-006. |
| T015 | T014 | Architecture/structure describe actual facade and identity application → Financial Core facade, domain purity and no reverse edge; no speculative layers. Links resolve. FR-011; AC-12; SC-006. |
| T016 | T014 | README commands are executable after implementation; policy availability distinguished from live auth, storage and financial workflows. No dependency/roadmap completion inflation. Links resolve. FR-013; SC-005/006. |
| T017 | T015, T016 | Required rustfmt command succeeds; preserve user edits and record necessary scoped formatting changes. FR-013. |
| T018 | T017 | Required Clippy command succeeds with zero warnings, no lint weakening/suppression. FR-011/013. |
| T019 | T018 | cargo test --test identity and cargo test pass; retain foundation/Financial Core behavior with no env, real secrets or services. All 12 AC have tests/review evidence; no live-interface claim. FR-013; SC-005. |
| T020 | T019 | cargo check succeeds; manifest/lockfile/toolchain remain unchanged. Inspect existing CI --locked checks discover identity target; do not claim a hosted pass or alter workflow without demonstrated need. FR-009/011/013; SC-005/006. |
| T021 | T020 | External rustc probes under /tmp reject raw DiscordAccountId construction, binding field modification and private domain/application imports. Record rejection categories; add no compile-test dependency or repository artifacts. FR-011/013; AC-12. |
| T022 | T021 | Inspect full new code/tests and diff, visibility, declared dependency edge, absent side effects/logging, no durable grants/financial logic. Recheck spec/plan/task coverage; resolve critical/high findings, verify quickstart guidance, links and git diff --check. FR-008–013; AC-10–12; SC-006. |
| T023 | T022 | Update task evidence only after checks; reconcile implemented artifact/ADR status, index/devlog/handoff and actual verification limits. All local links/whitespace valid; user edits preserved, no false hosted-CI claim. FR-013; full-feature completion. |

## Dependency Graph and Execution Order

```text
T001 → T002 → T003
  → US1: T004 → T005 → T006 → T007 → T008
  → US2: T009 → T010 → T011
  → US3: T012 → T013 → T014
  → {T015 || T016} → T017 → T018 → T019 → T020 → T021 → T022 → T023
```

US1 is the smallest independently useful increment. US2 requires its real values/resolution;
US3 checks the completed policy vocabulary. Each story has independently repeatable verification,
not an artificial claim that shared source can be implemented concurrently. The critical path is
the graph above, with either documentation branch required before final checks.

## Parallel Execution Examples

- US1: no parallel mutation; tests/exports and domain/application additions share files and depend
  on validated values. Run T004 → T005, then T006 → T007 → T008.
- US2: no parallel mutation; write scope tests before editing the shared application/facade, then
  verify T009 → T010 → T011.
- US3: no parallel mutation; privacy tests precede shared diagnostics, then security guidance review:
  T012 → T013 → T014.
- After US3, T015 and T016 can run concurrently: disjoint architecture/structure versus product/
  README files, with the same completed code prerequisite. Join both before T017. No other task
  is marked [P]; a marker does not authorize agent delegation by itself.

## Acceptance and Success Coverage

| Scope | Implementing / verifying tasks |
| --- | --- |
| AC-01–03 | T006–T008 |
| AC-04 | T004–T008 |
| AC-05–08 | T009–T011 |
| AC-09 | T012–T013 |
| AC-10 | T014, T022 |
| AC-11 | T012–T013, T019 |
| AC-12 | T007, T010, T014–T015, T021–T022 |
| SC-001 | T006–T008 |
| SC-002 | T009–T011 |
| SC-003 | T004–T005, T008–T011 |
| SC-004 | T012–T013 |
| SC-005 | T017–T020, T023 |
| SC-006 | T014–T016, T020–T023 |

FR-001–013 are mapped individually in the dependency table; no requirement is deferred silently.

## Implementation Strategy and Definition of Done

Deliver Setup/Foundational then US1 as MVP; add US2, then US3, then final documentation/verification.
Use coherent verified commits, preserving unrelated user changes. Stop at a restricted scope if the
maintainer authorizes only certain phases; otherwise complete the approved list in dependency order.
Do not deploy or demonstrate a live bot to validate this library policy.

Completion requires all 23 tasks verified, all 12 AC backed by tests/reviews, four Cargo gates
passing, unchanged dependency resolution and existing tests, safe diagnostics, private fields,
correct one-way feature dependency, accepted ADR and consistent docs/handoff. No critical/high
consistency issue may remain. Unverified or partial work stays unchecked with exact blockers.

No live authentication, operator-config loading, resource lookup, session/grant, persistent binding,
roles, account linking, multi-user administration, hot reload, in-flight revocation or owner-transfer
workflow is included. No financial mutation, ledger, payment, banking, scheduling, reporting,
QR/OCR/AI, production credentials or deployment belongs to this feature. Policy results never
prove provider authentication, durable permission, actual financial commit or bank verification.
