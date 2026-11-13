# Development log

This log records feature milestones and links to authoritative specifications/tasks. It does not
replace their requirements, implementation checklists, or verification evidence.

## 2026-11-12 — 004 Trusted Discord Owner Entrypoint

Status: [specification](specs/004-discord-owner-entry/spec.md) accepted for planning on 2026-11-13.
[Plan](specs/004-discord-owner-entry/plan.md), research, model, runtime/command contracts and guide
generated; constitution checks pass and ADR 0005 is proposed. Quality checklist remains 16/16.
Defines opt-in local Discord mode, operator-controlled binding,
one private /fundguard readiness command in one configured server, least privilege and safe
registration/failure/shutdown. Offline remains the default. No financial/resource operations.
Automated verification must remain credential-free; real Discord smoke evidence is separate.
Isolated dependency/framework probe passed on the pinned toolchain; no live provider call.
Repository code/dependencies are unchanged. On 2026-11-13, the maintainer requested $speckit-tasks,
accepting plan review for task generation. [Tasks](specs/004-discord-owner-entry/tasks.md) now contain
38 unchecked items across six phases, with dependency, acceptance and verification coverage.
Analysis found two medium and one low planning issue; authorized remediation clarified whitespace
semantics, explicit dispatch/duplicate-ack tests, and the Polish phase gate. Focused re-review passed;
all three findings were resolved before implementation. The maintainer then authorized implementation.
All 38 tasks are now locally verified: opt-in Poise/Serenity readiness, trusted configuration, private
owner replies, named registration, safe logs/errors and bounded lifecycle. All four Cargo gates pass;
69 tests pass and explicit offline run succeeds. ADR 0005 accepted. SDK builder owner-discovery override
was caught and fixed by the actual-framework test. Full SDK dispatch/error contexts are source-reviewed;
live smoke/hosted CI remain unverified. No financial feature or deployment. Next: persistence/atomic
financial-commit specification, including authoritative resource scope and durable idempotency.

Live follow-up (2026-11-13, maintainer-provided ignored local environment): authentication/gateway,
named guild registration and preservation readback passed; Ctrl-C stopped cleanly in 0.457s (exit 0).
No sensitive configured values appeared in captured output. Bot is stopped. The maintainer confirmed the private owner readiness reply. Other-account/disabled
denial, network recovery and hosted CI remain unverified.

## 2026-11-12 — 003 Owner Identity and Access Boundaries

Status: complete and locally verified; all 23 [tasks](specs/003-owner-access/tasks.md) checked.
Delivered canonical account values, caller-asserted actors, immutable owner binding, borrowed
owner resolution and scoped access with uniform denial/redaction. Reused existing OwnerRef through
the application facade without changing Financial Core. ADR 0004 is accepted.
All four Cargo gates passed; 37 tests passed, including 10 new identity tests. Five external compiler
probes verified private construction/import/field boundaries. Both minor analysis wording findings
were clarified. No dependencies, credentials, live authentication, lookup or deployment were added;
existing user edits are preserved. Hosted CI remains unverified.
Next: specify trusted owner configuration and a minimal authenticated interface before financial
commands, or separately define persistence/atomic commit semantics. Neither is implemented here.

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
