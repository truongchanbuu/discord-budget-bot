# Development log

This log records feature milestones and links to authoritative specifications/tasks. It does not
replace their requirements, implementation checklists, or verification evidence.

## 2026-11-12 — 003 Owner Identity and Access Boundaries

Status: [specification](specs/003-owner-access/spec.md) accepted for planning;
[plan](specs/003-owner-access/plan.md), research/model/contract/validation guide generated.
Constitution checks pass; ADR 0004 is proposed. Quality checklist passed 16/16.
The [23-task checklist](specs/003-owner-access/tasks.md) now defines dependencies, verification and
all acceptance/requirement coverage; every task remains unchecked.
Selected the roadmap's single-owner identity/access policy: explicit Discord-account binding,
fail-closed owner resolution, owner-scoped access and privacy. Provider authentication and
trusted resource ownership remain caller obligations; live integration and persistence are excluded.
No product code or dependencies were added. Next: $speckit-analyze, then explicit implementation
authorization. Task generation does not establish implementation or security verification.

## 2026-11-12 — 002 Financial Core and Domain Invariants

Status: completed and locally verified; all 26 [tasks](specs/002-financial-core/tasks.md) checked.
Delivered exact whole-đồng VND values, nonnegative virtual previews, typed nonblank references,
pure new/replay/conflict validation and redacted diagnostics. No dependency or lockfile change.
All four Cargo gates passed; 27 tests passed, including 17 new core tests. Six external compiler
probes confirmed private construction/module boundaries and absence of bank-verification evidence.
Hosted CI remains unverified. Existing user edits were preserved.

See [spec](specs/002-financial-core/spec.md), [plan](specs/002-financial-core/plan.md),
[contract](specs/002-financial-core/contracts/financial-core.md), and accepted
[ADR 0003](docs/architecture/decisions/0003-financial-core-validation.md).
Next: define identity/authorization or persistence/atomic journal semantics through a separate
specification. No financial state, journal, bank provider or deployment was delivered.

## 2026-11-10 — 001 Project Foundation (historical milestone)

The tracked [foundation tasks](specs/001-project-foundation/tasks.md) record Phase 0 completion:
offline bootstrap, tests, documentation, tooling and CI configuration. Those results are historical;
pre-existing local edits are not reverified by Financial Core planning. No financial business
features or deployment were delivered. This entry was added retrospectively from repository records.
