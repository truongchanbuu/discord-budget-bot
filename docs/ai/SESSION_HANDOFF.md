# Session Handoff

Updated: 2026-11-14. Active feature:
[006 Private Finance Channels](../../specs/006-private-finance-channels/spec.md).

## Current phase and authority

Specification drafted under maintainer request for per-member private server channels created by
mentioning the bot. Two explicit decisions: invited members read-only; financial messages still
mention this bot in the private channel. Quality checklist16/16; constitution review PASS.
Specification needs review before planning per .specify/workflows/speckit/workflow.yml review-spec.
No plan.md/tasks.md or006 product code exists; no implementation work is marked complete.

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

006: reviewed sixteen requirements, sixteen acceptance scenarios and six outcomes; links/diff/scope
and quality checklist verified. No Rust changed, so no new Rust run claimed for this spec-only step.
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

Next: review006 spec, then speckit-plan; review plan, then tasks/analyze/authorized implementation.
Technical planning must resolve provider/DB provisioning reconciliation, membership/privacy proof,
read-only channel permissions and legacy journal binding before implementing. No missing product
clarifications remain; remaining technical choices belong in plan/research/ADRs, not guesses in code.
