# Research: Private Finance Channels

Date: 2026-11-14. Scope: read-only source/document research; no live credentials/resources used.
Two research agents independently examined permission/membership and recovery/persistence boundaries.
All product clarifications are resolved; the maintainer's speckit-plan request accepts spec review.

## Unknowns resolved

| Question | Decision | Detail |
| --- | --- | --- |
| Mention input without Message Content? | Yes, explicit leading bot mention only | R1 |
| Member departures without broad member enumeration? | GUILD_MEMBERS events plus point lookups/incarnation | R2 |
| Private channel/read-only proof? | Canonical overwrites, explicit viewers, reject elevated guests | R3 |
| Creation retry across provider/DB? | Durable dispatch intent, reconciliation, quarantine ambiguity | R4 |
| Out-of-order sharing writes? | One pending operation, freeze unknown outcomes | R5 |
| Multi-user journal reuse without cycles? | ledger → finance_spaces → identity/core | R6 |
| Shared SDK client and privileges? | Startup-bound adapter using client.http, no new dependency | R7 |

## R1 — Explicit mention routing

Decision: enable GUILDS/GUILD_MESSAGES for006, preserve DIRECT_MESSAGES, omit MESSAGE_CONTENT.
Require a literal leading `<@bot-id>` or `<@!bot-id>` token plus matching actual mention metadata;
reply-only mentions, role/everyone mentions, other bots and webhooks are ignored. Strip just this
prefix before applying the existing finite grammar. Outside ready private channels only bare/start
mentions in configured onboarding channels are recognized. Never interpret public financial text.
Rationale: Discord retains content when the app is explicitly mentioned, without Message Content.
Alternatives: passive text requires broader content privileges; new slash finance is outside006.
Source: [Gateway content exceptions](https://github.com/discord/discord-api-docs/blob/main/developers/events/gateway.mdx#message-content-intent).

## R2 — Membership incarnation and gateway events

Decision: add privileged GUILD_MEMBERS specifically for tracked-owner/viewer departure/rejoin and
role changes. No GUILD_PRESENCES, bulk member listing or full member cache. Fresh point Get Guild
Member supplies roles, joined_at, pending/screening state and current membership. Persist joined_at
for owners/viewers; missing/changed incarnation prevents automatic reactivation.
Rationale: point polling alone misses rapid leave/rejoin. Remove invalidates first; Add only marks
verification-needed (Discord can send Add for existing members). SDK callbacks are concurrent, so
DB revisions stop late callbacks restoring grants. On reconnect/startup, invalidate verified status
and reconcile all tracked spaces before guild financial acceptance. No blanket authorization on Ready.
Alternatives: enable no member intent and claim perfect revocation is unsound; enumerate every member
is unnecessary. Native access during downtime or concurrent admin edits cannot be guaranteed away.
Sources: [Member events](https://github.com/discord/discord-api-docs/blob/main/developers/events/gateway-events.mdx#guild-member-add),
[Get member versus list](https://github.com/discord/discord-api-docs/blob/main/developers/resources/guild.mdx#get-guild-member).

## R3 — Canonical privacy and viewer permissions

Decision: create unsynchronized GUILD_TEXT children under a configured managed category. Exact
initial overwrite set: everyone denies VIEW_CHANNEL; owner/bot allow VIEW_CHANNEL, SEND_MESSAGES,
READ_MESSAGE_HISTORY. No other role or member overwrite is accepted. An everyone deny alone is
insufficient if a role overwrite allows view; exact structure proves ordinary outsider denial
without a member roster. Verify guild/parent/type, bot capabilities and membership as well.
For each viewer, allow VIEW_CHANNEL/READ_MESSAGE_HISTORY and deny SEND_MESSAGES,
CREATE_PUBLIC_THREADS/CREATE_PRIVATE_THREADS/SEND_MESSAGES_IN_THREADS. Application separately denies
all financial use cases and sharing to guests. Read-only does not promise to disable every reaction
or other Discord UI action. Reject elevated guest recipients: server owner, Administrator, or effective
MANAGE_CHANNELS/MANAGE_ROLES/MANAGE_WEBHOOKS. Reconcile later role elevation as an invalid grant.
Creation needs MANAGE_CHANNELS and can allow/deny only bits the bot has at guild level. Permission
PUT/DELETE needs MANAGE_ROLES; pinned Serenity rustdoc incorrectly says ManageChannels here.
Never set MANAGE_ROLES in initial overwrites: Discord reserves that setting for Administrators.
Bot inherits management from its guild role. Missing requested bits fail closed; no Admin escalation.
Alternatives: category copying leaks roles; per-viewer roles inflate server administration; deny-all
masks require unjustified permissions and can fail creation. Passwords cannot hide visible history.
Sources: [Permissions](https://github.com/discord/discord-api-docs/blob/main/developers/topics/permissions.mdx),
[Create channel](https://github.com/discord/discord-api-docs/blob/main/developers/resources/guild.mdx#create-guild-channel),
[Edit overwrites](https://github.com/discord/discord-api-docs/blob/main/developers/resources/channel.mdx#edit-channel-permissions).

## R4 — Durable creation and honest provider limits

Decision: reserve space and immutable operation/generation before provider I/O; commit dispatch_started
before invoking SDK create. Include a fresh opaque generation marker in the initial private channel
topic and audit reason. Persist returned ID, verify exact scope/privacy, then mark ready. On response
loss or post-create DB failure, match exactly one expected guild/category/marker/safe-profile channel.
No match, multiple matches, hidden/obfuscated channels or missing visibility remain uncertain/manual.
Never infer not-created from an empty list; no blind create retry after dispatch. Pre-dispatch failure
may retry safely; known errors require classification, never raw provider diagnostics.
Rationale: Discord's channel-create endpoint has no documented idempotency key. From November16,2026,
channel listings omit invisible channels. Pinned SDK retries rate limits; malformed rate-limit headers
can also trigger a retry after a potentially successful response. Guarantee one ACTIVE BOUND channel,
not exactly one wire POST or physical artifact. Private orphan candidates are quarantined, never
financially active or deleted/adopted merely by name. SDK success is not durable application success.
Alternatives: hold a DB transaction over HTTP, create-then-make-private, name-based adoption and
recreating after timeout violate safety. Custom no-retry HTTP stack adds complexity without solving
remote atomicity; retain maintained SDK and state the limit.
Source: [Guild endpoints](https://github.com/discord/discord-api-docs/blob/main/developers/resources/guild.mdx#create-guild-channel).
Local SDK evidence: serenity0.12.5 http/ratelimiting.rs perform(), especially redo.unwrap_or(true),
http/client.rs create_channel(), builder/create_channel.rs (no idempotency/nonce field).

## R5 — Sharing is serialized reconciliation

Decision: persist one pending grant/revoke per space, immutable desired state and revision; mark
space access_pending before permission PUT/DELETE. Do not dispatch contradictory writes while an
earlier outcome is unknown. Verify readback plus current DB revision before success. Unknown old
writes cannot be declared settled just because one GET matches: keep blocked/manual if late completion
is possible. Database fencing cannot fence Discord writes: there is no remote CAS/version contract.
Rationale: retries/restart must not let a delayed grant undo a newer reported revocation. Check
membership/privacy before application authorization and again before fresh financial output. These
are point-in-time checks, not atomic confidentiality against administrative changes. Old read history
cannot be recalled. In-flight already-authorized work may finish; no future request bypasses fresh checks.
Alternatives: parallel grant/revoke, cache-only authorization and unconditional success on request
acceptance give false guarantees. No generic event bus/outbox framework is needed.

## R6 — Existing journal compatibility and dependency direction

Decision: new finance_spaces feature owns membership, provisioning, sharing and access verification;
it imports identity/Financial Core facades, NEVER ledger. Ledger application depends on its application
facade for guild authorization; ledger presentation retains existing financial views/parsing. Space
presentation owns onboarding/access UI and lifecycle events. Platform composes both handlers beside Poise.
Journal requests gain DM/guild context; guild authorization resolves a fresh enabled OwnerBinding
from authoritative space state, never user input. Existing JournalStore continues owner/channel scoping;
no ledger SQL reaches into another feature's tables. There is no cross-feature DB transaction need:
channel/share transitions never write financial entries. Recheck access before channel output.
Legacy configured account reuses its exact enabled OwnerRef; persist that association and reject later
conflicting remaps. Other members get server-generated refs via PostgreSQL gen_random_uuid(), decoded
as text (no UUID crate). Collision rejects insertion, never reuses an existing owner accidentally.
Never reactivate a retired channel ID; explicit recovery uses a new generation/channel with the same
journal. Current draft channel checks then keep old controls invalid without altering immutable entries.
Alternatives: ledger↔spaces imports form a cycle; exposing private financial adapters breaks boundaries;
using channel names as identity or overwriting historical origins breaks data integrity.
Evidence: current identity/application.rs; ledger/application/{mod,port}.rs;
ledger/infrastructure/{drafts,commit,queries}.rs already owner-scope every operation.

## R7 — Composition, versions and verification

Decision: retain existing Rust1.99.1/2024, Poise0.7.1/Serenity0.12.5, SQLx0.9.0, Chrono0.4.45,
Tokio1.54.1, tracing/thiserror and lockfile. No additional dependency/feature flag is needed by this design.
Pinned SDK supports channel/overwrite/member/role APIs, User select, member/channel events and
PartialGuild::user_permissions_in without enabling a global cache. Use the concrete SDK adapter with
startup-only OnceLock<Arc<Http>> binding: build client, bind client.http exactly once before start,
fail closed if unbound. Http is not Clone; a second client would fragment rate limits. Shared bot
ID is bound from Ready before routing. Workers are owned/joined by existing lifecycle cleanup.
Local source: serenity client/mod.rs ClientBuilder::new_with_http and Client.http;
builder/create_components.rs CreateSelectMenuKind::User; client/event_handler.rs.
Alternatives: custom bot gateway, blanket full features, a generic DI container and token-per-user
bots solve no current need. External version upgrade is unnecessary; API compatibility was checked
against current official docs and pinned SDK source, not a claim these are latest releases.

Verification design: pure policy and actual SDK builder fixtures; application port fault injection;
disposable PostgreSQL concurrency/restart/upgrade/RLS; optional dedicated-guild three-persona smoke.
No real credentials/financial records or provider calls were used in this planning research.

## Post-design review

Independent permission and recovery reviews found no blocking design contradiction. One transport
wording issue was corrected: ordinary messages cannot get ephemeral replies; controls can. Prepared
confirmation expiry uses operation.created_at; ready outcome replay still needs current privacy.
Constitution recheck PASS; no generated tooling, runtime code, credentials or live resources changed.
