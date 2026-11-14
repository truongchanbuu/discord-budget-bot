# Development log

This log records feature milestones and links to authoritative specifications/tasks. It does not
replace their requirements, implementation checklists, or verification evidence.

## 2026-11-14 — Live private owner channel verified

Maintainer completed `@bot bắt đầu`. Read-only Supabase evidence shows one ready space with no
pending operation and preserved legacy journal ownership. Discord readback confirms the stored
channel is inside the managed category with exact everyone-deny and owner/bot allow overwrites.
Category recovery, Gateway readiness and owner onboarding now pass live. No financial entry was
created. Viewer/outsider personas, help/access controls and hosted CI remain unverified.

## 2026-11-14 — Corrected category recovery/startup succeeds live

Further explicit operator approval: corrected create201, exact private guild/category profile verified,
matching pending row CAS-bound once. Stopped separately started stale local binary, launched corrected
bot and observed discord_ready. Current bot left running for interactive test. Human reported channel
creation but database has0 personal spaces at inspection; clarified category versus #tai-chinh and
requested actual mention onboarding. Category/startup PASS; personal-channel/help/sharing/finance
live outcomes remain pending. No real financial mutations, secret/env edits or deployment.

## 2026-11-14 — Live category403 diagnosed and corrected

One explicitly authorized recovery create returned403/50013. Official Discord contract prohibits
non-admin MANAGE_ROLES channel overwrites; removed this redundant bit from the category builder.
Bot continues using guild-level permission; no Administrator requested. Mask regression went red
before fix; safe diagnostic redaction test added. All65 local tasks verified;148 ordinary+27 private
PostgreSQL tests, four Cargo gates and offline smoke pass. Pending live reservation preserved;
previous one-attempt authority consumed, corrected live retry awaits approval. No financial writes.

## 2026-11-14 — Live006 startup blocked after permission repair

Verified required bot role bits and member-intent flag, no Administrator. Configured Supabase startup
applied migrations1–4 successfully. Automatic-category reservation was persisted but startup exited1
(client_build_failed) without a category ID/Gateway readiness; original provider cause was not retained.
Read-only visible-channel inspection found no recent creation, which is not proof of absence.
No blind retry/reset, financial mutation, live flow claim or deployment. Human category inspection/
verified-override recovery remains pending; no local bot process left running. Local tests unchanged.

## 2026-11-14 — 006 Automatic setup usability amendment

Maintainer requested genuinely optional location IDs. Financial storage now enables private spaces
by default; startup creates/reuses a private Tài chính category via durable0004 reservation. Either
location override works independently; absent onboarding allowlist permits ordinary accessible text
channels. Financial authorization remains confined to owner-bound spaces. Same SDK Http is moved
into ClientBuilder, no new dependency. Pending create outcomes stop safely; documented override
recovery avoids name-based adoption/duplicate creation. No live provider/database operations in this
amendment; latest observed Manage Roles/Server Members prerequisites still need operator setup.
All63 tasks verified; four Cargo gates/offline smoke pass,147 ordinary+27 isolated PostgreSQL
tests pass (12 Ledger,15 Spaces). Prior failed fixture assumed pre-existing API roles; corrected
fixture creates transactional roles and verifies revocation. No production workaround/suppression.

## 2026-11-14 — Authorized live Discord readiness smoke

Bot REST authentication/configured-guild access and readiness Gateway startup pass. No existing bot
process was duplicated. Database disabled for this smoke; no Supabase/migration/financial mutations.
Live006 blocked by missing category/onboarding config, Manage Roles and advertised Server Members
intent capability. Configured-guild slash registration verified; owned session stopped cleanly (SIGINT/exit0). Human
slash reply and three-persona privacy tests remain pending; secrets untouched.

## 2026-11-14 — 006 Private Finance Channels completed locally

All56 [tasks](specs/006-private-finance-channels/tasks.md) verified: self-service private onboarding,
per-member journal isolation/legacy DM preservation, owner-only financial routing, read-only sharing,
expiry/dispatch races, lifecycle invalidation/native cleanup and explicit fresh-channel recovery.
All four Cargo gates and offline smoke pass;143 ordinary+26 isolated PostgreSQL tests pass
(12 Ledger,14 Spaces). All16 ACs mapped to local tests/review in quickstart. ADR0008 accepted;
ADR0009 exact-incarnation repair retained. Existing dependencies/generated tooling/0001/0002 unchanged.
No live006 Discord/Supabase, hosted CI, deployment or push; these remain unverified. Earlier partial
checkpoints below are historical and superseded. Next: optional authorized synthetic live validation.

## 2026-11-14 — 006 Incarnation precision repair verified

Repaired owner/operation/viewer incarnation persistence with a canonical UTC RFC3339 nanosecond
codec and TEXT checks in unreleased0003. Exact authorization and audit/expiry timestamps unchanged.
Both prior failing DB regressions now pass;124 ordinary+20 isolated DB tests pass. Format/check pass;
Clippy still fails on previously recorded incomplete006 issues. ADR0009 accepted for this repair;
ADR0008 and parent tasks remain pending. No provider credentials, live bot/database, deploy or push.

## 2026-11-14 — 006 Continuation halted on identity precision regression

Added partial provisioning, lifecycle/authorization, shared SDK/root wiring, guild Ledger scope and
sharing/UI code plus tests. Ordinary121 PASS; isolated Ledger10 PASS/1 FAIL and Spaces7 PASS/1 FAIL.
Confirmed SQLx/PostgreSQL loses sub-microsecond joined_at precision; exact owner validation then
returns Denied. Added a deterministic regression and honored maintainer's stop-on-problem instruction.
Format/check pass; Clippy fails; additional acceptance/security cases and final integration remain
pending. No new tasks checked, live resources/credentials touched, commit/push or deployment.
Next: preserve incarnation losslessly, rerun both regressions, then complete the authoritative tasks.

## 2026-11-14 — 006 Foundation implementation checkpoint

Implemented and verified T001–T016 in [tasks](specs/006-private-finance-channels/tasks.md): optional
validated config, shared private DB fixture guard, pure typed models/privacy policy, additive0003
backend-only schema, SQL primitives, SDK observation/mask/mention helpers, real ports/fakes and legacy
identity resolution. No guild handlers/lifecycle activated; user stories T017–T056 remain pending.
110 ordinary tests and13 isolated PostgreSQL tests passed; formatting/check/offline smoke passed.
Clippy FAILS on unused foundations awaiting runtime wiring; no suppression added. Feature006 is partial,
not ready for use/deployment; hosted CI/live guild visibility unverified. No real env/provider touched.
Server tools/runtime libraries provisioned under /tmp; disposable DB cleaned by script. Existing D1
planning changes preserved. ADR0008 remains proposed; no new architectural decision.

## 2026-11-14 — 006 Analysis remediation

Maintainer authorized planning fixes for I1/C1/U1/D1: distinct finance/management guards, explicit
Spaces authorization work, atomic abandoned-prepared expiry and complete incremental port adapters.
Task list now56 unchecked items with consistent renumbered dependencies and coverage; no new product
scope. Plan/model/contracts/ADR/handoff reconciled. No implementation or live resources touched;
only document validation performed. Follow-up analysis identified residual D1 sequencing ambiguity:
T013 now defines data contracts only; T014/T015 introduce each port with its complete adapter/fake.
Plan and FR-014 coverage reconciled; 56 tasks remain unchecked and dependency order unchanged.
Document checks passed; no Rust or live verification performed for this correction.

## 2026-11-14 — 006 Private Finance Channels specification

Created [spec](specs/006-private-finance-channels/spec.md) and16/16 quality checklist for mention-based
self-service private channels with per-member journals. Maintainer clarified read-only viewers and
explicit bot mentions. Defined owner-only writes/access management, privacy/admin/history limits,
legacy journal preservation, provisioning/retry/recovery and permission drift boundaries. Passwords
are deferred. Maintainer invoked speckit-plan, accepting spec review. Plan/research/model/contracts/guide generated,
ADR0008 proposed; pre/post constitution checks passed. Two read-only research agents resolved SDK
permission/intents, durable recovery and legacy mapping. One active binding, not exactly-once physical
channel creation; no cross-feature SQL/financial rewrite. Maintainer invoked speckit-tasks, accepting plan review.54 unchecked tasks now cover16 FR/AC and6 SC;
ordered Setup/Foundational/US1/US4/US2/US3/Polish with seven safe parallel pairs, model constraints,
verification and DoD. Minor model prose corrected to existing UUID operation-ID definition. Format,
dependencies/coverage/links verified; analysis next. No code/test execution, live resources/config,
new dependencies or deployment. Index/handoff updated.

## 2026-11-14 — Repository publication

Maintainer requested main/rebase/push to discord-budget-bot. Local already main; empty destination
has no base to rebase onto. Configured origin; normal main push authorized. Four local Cargo gates
pass,90 ordinary tests pass;10 DB tests explicitly ignored/not rerun for Git delivery. Historical
tracked blobs contain no configured credentials; local env stays ignored. No product scope added.

## 2026-11-14 — Live Supabase/local bot smoke (partial)

Initial attempt found missing process config and unreachable Direct IPv6. Maintainer changed to
Session pooler; real connection/client TLS then passed. Restarted one local bot with finance enabled;
discord_ready observed, both embedded migrations applied, five tables RLS-protected and no API-role
access. Project empty at read-only probe.90 ordinary tests passed earlier in this smoke. Maintainer corrected the bot token; intended bot authenticated/restarted successfully. Owner DM
help/funds confirmation pending; financial UI/mutation/report smoke unverified. Bot remains running.
No agent Discord send, financial mutation, local-config edit or credential value retained in repo.

## 2026-11-14 — 005 Supabase storage amendment

Maintainer requested managed Supabase instead of self-hosted application PostgreSQL. ADR0007 keeps
SQLx/domain/atomic journal; remote TLS/session validation and additive backend-only RLS/grant migration
implemented. Setup/example/security/feature artifacts reconciled. All five amendment tasks verified:
four Cargo gates,90 ordinary tests and10 real disposable PostgreSQL tests pass, including API-role
denial and lossless v1 upgrade/restart. No real .env.local read, live Supabase connection, existing-data
migration, new dependencies or deployment. Next: configure dedicated Supabase project/Data API off/URL
privately, then separately verify hosted connectivity. Detailed evidence remains in feature tasks.

## 2026-11-13 — 005 Conversational Personal Finance

Maintainer authorized full Spec Kit workflow for private conversational income/expenses, categories,
virtual funds/reserves and monthly totals. [Spec](specs/005-conversational-finance/spec.md) quality16/16;
[plan](specs/005-conversational-finance/plan.md), contracts/model/research and
[32 tasks](specs/005-conversational-finance/tasks.md) generated; cross-artifact review passed.
Implementation complete: all32 tasks verified, four Cargo gates pass,87 ordinary tests plus8 explicit
PostgreSQL tests pass. DM previews/edit/confirm/cancel, durable manual journal, fund initialization/
transfers, categories and exact monthly view implemented. Owner serialization and immutable history
passed concurrency/rollback/restart/full-u64 checks. ADR0006 accepted. Real .env.local untouched;
no real financial data, live finance/provider test or deployment. Hosted CI remains unverified.
Next: optional synthetic finance DM smoke, then separately specify immutable correction/reversal.

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
