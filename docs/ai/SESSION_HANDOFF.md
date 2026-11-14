# Session Handoff

Updated: 2026-11-14. Active feature:
[006 Private Finance Channels](../../specs/006-private-finance-channels/spec.md).

## Current phase and authority

Maintainer invoked speckit-plan, accepting006 spec review. Research/design are complete:
[plan](../../specs/006-private-finance-channels/plan.md),
[research](../../specs/006-private-finance-channels/research.md),
[model](../../specs/006-private-finance-channels/data-model.md),
[UI/security/operations contracts](../../specs/006-private-finance-channels/contracts/discord.md),
[validation guide](../../specs/006-private-finance-channels/quickstart.md) and proposed ADR0008.
Quality16/16 preserved; constitution checks PASS before/after design. Plan review is next, before
speckit-tasks per workflow review-plan. No tasks.md or006 product code exists; nothing implemented.

## Scope and decisions

Self-service onboarding in configured server/channel(s); one active space per server/member;
new members have isolated empty journals. Owner alone manages finances and explicit viewer list.
Guests read history/future owner-posted responses, cannot post or invoke financial operations.
Current configured-owner journal must be preserved through trusted mapping; existing owner DM
behavior remains compatible. Message names/references and Discord visibility never grant ownership.

No passwords/PINs/codes in first increment. Native channel privacy cannot hide messages from server
owner/Administrators, nor undo previously read/copied history. These limitations must be disclosed.
Creation/sharing/permission failures require truthful pending states and fail-closed disclosure;
restart/retries must not create competing channels or financial effects. Channel deletion/departure
retains financial history and requires safe explicit recovery. No bank/AI/deployment scope.

## Related implemented context to inspect

Read005 spec/plan/contracts and [tasks](../../specs/005-conversational-finance/tasks.md) for delivered
journal behavior. Financial Core/identity are pure; ledger application currently uses a configured
single-owner binding, and presentation rejects guild messages.006 must replace these assumptions
with authoritative multi-user scope; merely accepting guild messages is unsafe. Domain/application
remain independent of Discord/SQLx. No generic tenant framework or per-user bots.

Existing stack: one Rust2024 package, pinned stable1.99.1, Poise/Serenity, SQLx0.9/Chrono0.4.45,
Supabase-managed PostgreSQL via direct/session5432, remote TLS, backend-only RLS/grants migration.
Do not change applied migrations or expose DB credentials/API roles. Preserve legacy ledger invariants.

## Verification and live limits

006: reviewed sixteen requirements/acceptance scenarios and six outcomes; design coverage matrix,
links/diff/sizes and source-backed permission/recovery decisions verified. Two read-only research
agents used as required by plan skill. No Rust changed; no new compilation/test/provider run claimed.
Previous implementation/local gates: fmt/Clippy/test/check PASS,90 ordinary +10 isolated PostgreSQL
cases passed; latest pre-push ordinary90 PASS,10 DB cases explicitly ignored in that command.
Actual Supabase connection/client TLS, two migrations, five RLS tables and absent API-role grants
verified during prior live smoke. Bot token corrected by maintainer and one bot restarted/ready.
Maintainer subsequently reported it works; full finance confirmation/modal/replay/report flow and
hosted CI remain unverified. Previous bot was left running; inspect before future live actions.
No live Discord/DB resources, configuration or credentials touched for006 specification.

## Repository and next action

origin: https://github.com/truongchanbuu/discord-budget-bot.git . Branch main was published normally
after destination was found empty. Do not assume new local work is pushed. Preserve unrelated work.
Active selector is locally ignored; FEATURE_INDEX is tracked discovery. Generated skills unchanged.

Next: review006 plan, then speckit-tasks → speckit-analyze → explicitly authorized implementation.
No unresolved product clarifications. Design decisions resolved in research; limits are explicit,
including no exactly-once remote creation or atomic provider/database confidentiality. No real env,
Discord/Supabase resources or running bot were touched by planning.

## Implementation decisions to preserve

Acyclic graph ledger → finance_spaces → identity/core; spaces imports no ledger. Journal application
checks guild context via Spaces before existing owner/channel-scoped store; recheck privacy on output.
No cross-feature SQL or financial engine rewrite. Four space/operation/viewer/audit tables via0003,
backend-only RLS/revokes. Legacy mapping exact/enabled/collision-rejecting; never change old entries.
Create has durable dispatch intent and opaque marker; unknown/multiple/hidden outcomes quarantine.
Sharing serializes pending writes; CAS cannot fence late Discord writes. Retired bindings use fresh
channel generations with no viewers carried over; old-native access uncertainty is stated honestly.
GUILD_MEMBERS justified for remove/rejoin; Message Content stays off. Canonical unsynced overwrites,
minimal management/send/thread bits; reject elevated viewers. Use existing shared SDK Http with one
startup binding and bounded lifecycle worker. No new dependency/feature flag.
Future tests must include both ledger and finance_spaces DB suites in script/CI, preserve private
fixture-host guard and update migration-count expectations; do not silently test only005.
