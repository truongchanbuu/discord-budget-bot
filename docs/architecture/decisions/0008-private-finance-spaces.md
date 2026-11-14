# ADR 0008: Private finance spaces and reconciled channel access

Date: 2026-11-14. Status: Accepted; implemented and locally verified. Live/native Discord checks remain unverified.
Authority: maintainer requested private per-member channels and then invoked speckit-plan.

## Decision

One finance_spaces feature owns self-service membership, channel lifecycle and read-only sharing.
Dependencies remain acyclic: ledger → finance_spaces → identity/Financial Core. Spaces never imports
ledger. Ledger application authorizes guild request context through the spaces application facade,
then reuses current owner/channel-scoped journal mechanics. Financial authorization requires ready
privacy; separate owner-management authorization permits nonfinancial status/cancel/recovery without
returning a journal binding. Onboarding validates current-member eligibility, not ready state. DM behavior stays compatible. Ledger adapters never query space tables. Spaces creates/reuses only the approved shared fg_owners
anchor and never accesses entries/funds. No generic tenant/ACL framework is introduced.

Reuse locked dependencies and existing bot. Add only justified GUILDS/GUILD_MESSAGES/GUILD_MEMBERS
intents; retain DIRECT_MESSAGES, leave MESSAGE_CONTENT/PRESENCES off. Canonical initial overwrites
make ordinary outsiders invisible from creation; owner-approved guests receive native read-only
posting limits and no financial application authority. Server Administrator visibility remains.

Ports expand only with complete implementations/fakes; no future method stubs. Prepared grant/revoke
confirmation expiry atomically frees the matching slot/audits once; dispatched uncertainty never
expires into success or availability. Persist space/operation/viewer/audit state before external writes. Creation has no Discord idempotency
key; mark dispatch durably and reconcile exact generation/privacy profile. One active binding is
provable; exactly one physical channel is not. Unknown/multiple/hidden results remain quarantined.
One unresolved sharing write blocks contradictory changes and fresh disclosure. DB CAS cannot fence
late remote permission writes. Recheck membership/privacy before authorization and output; explicitly
state the provider/DB race and offline-history limits instead of promising atomic confidentiality.

Preserve legacy owner association, existing entries and source keys. New owners get independent
server-generated journal refs. Retire channel IDs after owner incarnation change/deletion; explicit
recovery uses a fresh channel/generation while retaining the journal. Old viewers are not carried
into replacement channels; old-native-history cleanup may remain pending separately.

Bind a concrete adapter to client.http once after client construction; no second SDK rate limiter,
custom gateway, SDK rewrite or new dependency. Lifecycle owns bounded reconciliation/cleanup.

## Consequences and alternatives

Four durable tables and two real ports are justified by non-atomic Discord/database workflows.
Extra privileges are limited and documented: MANAGE_CHANNELS for creation, MANAGE_ROLES for access
edits, requested send/thread bits needed for initial/guest masks; never Administrator. GUILD_MEMBERS
is necessary to observe removals/rejoins; point polling alone misses transitions.

Rejected: create publicly then lock, everyone-deny plus arbitrary role allows, password-based channel
privacy, per-user roles/bots, cache-only ownership, name-based channel adoption, blind create retries,
concurrent unresolved grants/revokes, and a custom HTTP client solely to hide remote uncertainty.
Read-only means no messages/financial operations, not disabling every reaction/Discord UI action.

[Spec](../../../specs/006-private-finance-channels/spec.md),
[plan](../../../specs/006-private-finance-channels/plan.md),
[research](../../../specs/006-private-finance-channels/research.md),
[security contract](../../../specs/006-private-finance-channels/contracts/security.md).

## Resolved precision issue — 2026-11-14

[ADR0009](0009-lossless-member-incarnation.md) preserves exact member incarnation using canonical
UTC RFC3339 TEXT. Both prior failing PostgreSQL regressions pass; ownership equality is unchanged.
The broader implementation is verified separately in tasks.md.

## Implementation safeguards

Internal cleanup invalidates a departed/elevated viewer before removing its native overwrite.
Readback may settle that removal while the space remains suspended; only a complete current privacy
check restores financial access. An uncertain write never permits an opposite write or fresh disclosure.
Owner recovery uses the current configured category and a new physical channel/generation.
A bounded command limiter and database quotas complement the finite cancellable reconciliation worker.
Only API10003 on a known binding proves deletion; empty/hidden candidate listings stay quarantined.
Local SDK/controlled-port/PostgreSQL evidence does not establish live Discord visibility or hosted CI.

## Authorized automatic setup amendment

Database-backed guild spaces now activate without location IDs. Either ID remains an independent
operator override; an absent allowlist admits onboarding in accessible ordinary text channels.
The bot prepares a private Tài chính category during startup. Additive0004 persists a unique guild
reservation/category binding; dispatch uncertainty stops startup until an operator supplies a
verified override, rather than adopting a name or creating duplicates. Existing spaces are not moved.
Startup uses the same Http subsequently owned by ClientBuilder/client.http, not a second SDK client.
A fifth backend-only table is justified by durable category identity across restarts; domain/use-case
financial authority and privacy checks remain unchanged. No additional dependency/privilege.

## Live-found overwrite restriction

The operator-approved recovery POST returned403/50013 with all required guild bits present.
Discord's Create Guild Channel contract permits setting MANAGE_ROLES in a channel overwrite only
for administrators: https://docs.discord.com/developers/resources/guild#create-guild-channel.
Remove that bit from the automatic category's bot overwrite; inherit existing guild-level
MANAGE_ROLES instead. Keep everyone-deny/private bot visibility and no Administrator requirement.
Creation diagnostics retain static category plus HTTP status/numeric Discord code only, never raw
body/URL/message. Rejected/unknown attempts do not automatically reset the pending reservation.
