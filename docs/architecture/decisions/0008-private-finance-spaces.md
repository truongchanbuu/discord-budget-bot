# ADR 0008: Private finance spaces and reconciled channel access

Date: 2026-11-14. Status: Proposed with006 plan; no runtime change implemented.
Authority: maintainer requested private per-member channels and then invoked speckit-plan.

## Decision

One finance_spaces feature owns self-service membership, channel lifecycle and read-only sharing.
Dependencies remain acyclic: ledger → finance_spaces → identity/Financial Core. Spaces never imports
ledger. Ledger application authorizes guild request context through the spaces application facade,
then reuses current owner/channel-scoped journal mechanics. DM behavior stays compatible. No financial
adapter directly reads another feature's tables and no generic tenant/ACL framework is introduced.

Reuse locked dependencies and existing bot. Add only justified GUILDS/GUILD_MESSAGES/GUILD_MEMBERS
intents; retain DIRECT_MESSAGES, leave MESSAGE_CONTENT/PRESENCES off. Canonical initial overwrites
make ordinary outsiders invisible from creation; owner-approved guests receive native read-only
posting limits and no financial application authority. Server Administrator visibility remains.

Persist space/operation/viewer/audit state before external writes. Creation has no Discord idempotency
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
