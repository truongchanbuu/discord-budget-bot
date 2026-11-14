# Implementation Plan: Private Finance Channels

**Branch**: main (feature selector006-private-finance-channels) | **Date**: 2026-11-14
**Spec**: [spec.md](spec.md). Maintainer's speckit-plan request accepts specification review.
**Status**: Plan accepted by maintainer's speckit-tasks request2026-11-14;
[65 tasks](tasks.md): complete and locally verified2026-11-14. All Cargo gates, offline smoke
and both isolated PostgreSQL suites pass; live Discord and hosted CI remain unverified.

## Summary

Add self-service private channels in the configured server, created by explicit bot mention.
Each member owns an isolated journal; owner can grant/revoke read-only viewers. Reuse existing
manual journal rules, Supabase/SQLx and real Discord SDK. Preserve legacy configured-owner DM data.
Creation/sharing are durable external workflows with verified privacy, not atomic Discord/DB writes.
Guarantee one active bound channel; quarantine uncertain/duplicate physical artifacts rather than
blindly create/adopt/delete. No passwords, shared financial editing, new bank/AI features or deployment.

## Technical Context

- Language/platform: existing Rust stable1.99.1/Edition2024, single Cargo package, long-running bot.
- Dependencies: retain locked Poise0.7.1/Serenity0.12.5, Tokio1.54.1, SQLx0.9.0, Chrono0.4.45,
  tracing/thiserror. No new dependencies or Cargo features required. Pinned SDK source reviewed.
- Storage: managed PostgreSQL, additive0003/0004 migrations; never rewrite previously delivered migrations or existing entries.
  Use server-generated UUID text via gen_random_uuid(), existing whole-VND types and owner locks.
  External member incarnations use canonical UTC RFC3339 TEXT with9 fractional digits through one
  infrastructure codec; audit/expiry timestamps remain TIMESTAMPTZ (ADR0009).
- Interfaces: explicit leading bot mention, private member-selection/confirmation controls and
  existing finance buttons/modals; no new financial slash commands or HTTP API.
- Optional config: FUNDGUARD_FINANCE_CATEGORY_ID and FUNDGUARD_FINANCE_ONBOARDING_CHANNEL_IDS.
  Both are independent overrides; finance URL enables006 with automatic category and unrestricted
  text-channel onboarding by default. Reject invalid/duplicate supplied IDs; optional allowlist1–8.
  No finance URL means readiness only; overrides without storage fail. Offline ignores bot settings.
- Intents when006 enabled: GUILDS, GUILD_MESSAGES, GUILD_MEMBERS plus existing DIRECT_MESSAGES.
  GUILD_MEMBERS is justified for departure/rejoin and role invalidation; no MESSAGE_CONTENT/PRESENCES.
  No member roster enumeration/cache. Portal intent toggle is an explicit setup prerequisite.
- Permissions: bot guild role MANAGE_CHANNELS/MANAGE_ROLES, VIEW_CHANNEL/SEND_MESSAGES/
  READ_MESSAGE_HISTORY and the four send/thread bits it must deny to viewers. No Administrator.
  Exact masks and elevated-viewer rejection are in [security contract](contracts/security.md).
- Scope/capacity: one configured guild/category, max50 nonretired reservations and available category
  capacity (pending/uncertain reservations count); max8
  viewers per space. Ordinary outsiders are structurally denied; administrative bypass is disclosed.
- Budgets: SDK interaction ack/send2s; initial edit-modal lookup+response2s (else safe retry error).
  Space DB steps5s, privacy inspection5s, provider mutation10s, workflow attempt20s. No DB transaction
  spans HTTP. SDK rate-limit waits fit operation deadlines; timeout records uncertainty, not success.
- Rates: up to3 new creation attempts/member/10min and10/guild/min; completed-space lookups do not
  create attempts. Sharing up to10 changes/space/min. Financial commands10/member/min. Persist resource
  creation/change windows; reserve guild quota/capacity under a short advisory transaction lock; bounded in-memory non-resource command counters reset harmlessly on restart.
- Testing: pure policies, real SDK builders/events, controlled application ports and private PostgreSQL
  suite; optional dedicated guild with owner/viewer/outsider accounts. No real financial test data.

## Constitution Check

Pre-research PASS; post-design PASS. No amendment/exception required.

| Principle | Design evidence |
| --- | --- |
| Readable idiomatic Rust | Cohesive feature modules, private APIs, explicit enums/errors; no giant generics; files<=250 lines |
| Feature-first boundaries | ledger → finance_spaces → identity/core; spaces never imports ledger; provider types stay in adapters |
| Financial integrity | Reuse005 posting rules, immutable entries, owner/channel/source checks and atomic idempotency |
| Minimal replaceable integrations | Existing SDK/SQLx only; two real space ports, no per-user bots, generic tenant engine or event bus |
| Verified persistent delivery | AC/FR matrix, ADR0008, separate future tasks and honest local/live evidence |
| Security | Current membership and per-request ownership; fail-closed privacy; minimal privileges and no credential/financial logs |

Current single-owner configuration is a legacy identity binding, not a global allowlist for006.
New members are authorized through durable server/member ownership, never channel visibility alone.
Privileged member intent is narrowly required by AC15; Message Content stays off as FR004 requires.

## Project Structure

Documentation: spec.md, this plan, [research](research.md), [model](data-model.md),
[commands](contracts/discord.md), [operations](contracts/operations.md),
[security](contracts/security.md), [validation guide](quickstart.md).
Implementation status is tracked in [tasks.md](tasks.md); all user stories are implemented; local verification evidence is recorded there.

Implemented feature entry points (behavior-specific helpers/tests are listed in project-structure.md):

```text
src/features/finance_spaces/
  mod.rs
  domain/{mod,model,policy,error}.rs
  application/{mod,ports,provisioning,sharing,authorization,recovery}.rs
  infrastructure/{mod,postgres,discord,mapping}.rs
  presentation/{mod,messages,components,views}.rs
migrations/0003_private_finance_spaces.sql
```

Each file has one responsibility; split persistence/tests by behavior when reaching250 lines.
Sibling domain/application/provider test modules cover real responsibilities, not placeholder trees.

Existing modifications: src/features/mod.rs; ledger/application/mod.rs for request authorization
context; ledger/presentation/{mod,messages,components,views}.rs for guild input
and safe responses; identity/application.rs gains a crate-private configured_account accessor only: distinguish the
reserved disabled legacy actor from an unrelated new member without exposing provider types.
platform/discord/client.rs and lifecycle integration wire optional Spaces/application and handlers;
config/discord.rs and tests validate optional setup; platform/database.rs registers0003.
Current financial domain/infrastructure arithmetic, schemas and source keys remain intact.
Update scripts/test-postgres.sh and .github/workflows/ci.yml to run both explicit ledger and
finance_spaces ignored DB suites. Extract existing private-host/fixture guard into test-only
src/platform/database/test_support.rs and reuse from both suites, never weaken it or read real env.
Update existing migration-count upgrade assertions for0003; retain historical financial regression tests.

## Application and Adapter Boundaries

Spaces application exposes onboarding, authorize-owner, viewer request/confirm/list and reconciliation.
Two genuine ports: SpaceStore for durable state/CAS and ChannelDirectory for Discord observations/
channel permissions. Provider-neutral IDs, snapshots and errors cross ports; no SQLx/Serenity types.
Store holds short transactions; directory returns canonical facts, not permission to skip policy.

Introduce port methods only alongside their complete concrete implementations and updated test fakes.
T013 defines only current-use provider-neutral data contracts and exports validated types; it adds
no port trait or empty service facade. T014 introduces SpaceStore primitives with complete SQL/fake
implementations; T015 introduces ChannelDirectory inspection with complete SDK/fake implementations.
Shared application test_support.rs holds these concrete fakes and remains test-only.
Foundational ports contain current observation/state primitives, not future required mutation stubs.
Reservation methods arrive with their SQL implementation; all Discord mutation methods arrive before
provisioning/root wiring; sharing-specific store methods arrive with their SQL implementation.
Every such checkpoint must compile. No todo!(), unimplemented!() or fake unavailable methods to
satisfy a future trait. Keep the two existing real boundaries; do not add a generic provider framework.

Spaces implements authorize_finance and authorize_owner_management as separate application policies
before root wiring and Ledger guild integration. The former yields FinancialAccess with a verified journal binding only
for ready owner finances; the latter yields OwnerManagementAccess with no financial authority and supports owner status/cancel/
explicit recovery in non-ready states. Onboarding has its own current-member eligibility check.
Recovery preparation binds the new incarnation/revision without reactivating the old channel;
confirmation checks it again. Lifecycle service operations do not impersonate a user control.


Journal gains DM/Guild request contexts and an optional Spaces application dependency. Each existing
use case authorizes current context before using its existing JournalStore. DM authorization stays
unchanged. Guild authorization resolves current member, verified active space and trusted OwnerBinding.
No caller can supply another space's binding/OwnerRef. Financial adapters never query space tables.
Space/share workflows do not mutate entries, so no cross-feature transaction/framework is introduced.
Authorization is point-in-time; legitimately authorized in-flight work can complete after an external
change. Recheck current space/privacy before every fresh guild financial output. A successful commit
with blocked/failed reply stays committed; same source retry identifies it without a second effect.

SpacesEvents handles onboarding/access UI and member/channel/role lifecycle. Existing FinanceEvents
handles financial input/controls; Poise retains readiness. One shared classifier prevents double
handling: start/access commands are excluded from finance parsing. Prefixes fgspace and fgfinance
are disjoint; financial rules/views stay in ledger, avoiding duplicate business logic.

Concrete Discord directory has startup-only OnceLock<Arc<Http>> binding. Build handlers/services,
await existing Client builder, bind client.http exactly once before start; unbound calls deny safely.
Bind real bot ID on Ready before mention parsing. No second SDK HTTP client/rate limiter. The finite
startup/reconnect reconciliation worker is lifecycle-owned, bounded and joined/cancelled on shutdown;
it is not a new reminder scheduler. Invalidate ready spaces on startup/reconnect, then reconcile in
small batches (two spaces, four concurrent provider lookups), with on-demand checks for active requests.

## Persistence and Ownership

Four feature tables: spaces, operations, viewers, access_events; see data-model for constraints.
New owner refs are generated server-side and collision-rejecting; existing configured account uses
only its enabled exact trusted OwnerRef. Persist legacy association and reject conflicting remaps on
subsequent setup before connected financial handling. Existing configured-owner access is not automatically extended to other members.
New tables enable RLS. Revoke PUBLIC/anon/authenticated/service_role privileges on owned tables,
sequences and audit functions as0002; trusted backend owner accesses them. No Supabase Auth/frontend keys or REST financial writes.

Stable space/journal identity survives channel generations. Never reactivate retired channel IDs;
deleted channels or owner-incarnation change require explicit fresh-channel recovery. Preserve old
financial data; new channel starts without carried-over viewers. Retired-generation operations
cannot activate the replacement; old-native history/access cleanup is reported separately. Existing
draft channel checks then keep old controls invalid. New source format is
discord:guild:<message-id>; existing discord:dm:<message-id> origins are unchanged.

## Provisioning, Sharing and Failure Recovery

1. Check configured guild and a permitted onboarding text channel, authenticated current human membership and quotas.
2. Reserve unique guild/member space, journal association and immutable operation/generation.
3. Commit dispatch_started before SDK create with restrictive overwrites and opaque generation marker.
4. Persist returned ID, verify scope/membership/privacy, then CAS ready and reveal only the private link.
5. Unknown outcomes reconcile by exact durable marker/profile, never names or empty channel listings.
   Multiple/hidden/missing candidates remain uncertain/manual; no blind repeat of the creation call.

Prepared sharing UI expires atomically at5min during new-request/status/confirm/cancel/startup checks:
clear only its matching pending slot, record one expired audit, leave grants/privacy unchanged. Never
expire dispatched/uncertain or creation operations. Test the exact boundary and confirm/cleanup race.
One pending access operation per space. Mark access_pending before remote grant/revoke; block financial
disclosure while uncertain. Never dispatch an opposite/newer permission write over unresolved older
writes. SDK success requires readback and durable current revision before reported success. CAS fences
DB state, not remote late writes; manual recovery is required when quiescence cannot be established.
No hard claim of exactly-once physical channel creation or atomic Discord/database confidentiality.

On member remove, invalidate owner binding/viewer grant first; Add never grants access. Compare
joined_at from fresh membership snapshots on reconciliation and before disclosure. Changed/missing
viewer incarnation requires a new explicit invitation. Retired owner channel IDs cannot be reused.
Guild role/category/channel changes invalidate affected spaces before reconciliation. Never silently
repair unrelated overwrites; operator corrects drift, then exact safe profile must be verified.

## Validation and Delivery Sequence

This is implementation order, not a generated task checklist:
1. Pure ownership/mention/privacy policies and optional config tests.
2. Additive migration, durable state constraints and legacy mapping/upgrade tests.
3. Directory builders, current membership/permission verification and durable operation fault tests.
4. Provisioning/recovery and lifecycle invalidation/reconciliation; guild acceptance stays blocked
   until these safety hooks are wired.
5. Guild journal authorization/view reuse, then read-only sharing and bounded root composition/cleanup.
6. Documentation, optional synthetic live personas, required Cargo gates, full isolated DB suite,
   diff/import/security review, task/handoff updates. Never auto-deploy or touch the running bot.

| Verification group | Coverage | Required evidence |
| --- | --- | --- |
| V1 membership/ownership | AC03/08/09/12/15; FR005/010 | actor/resource denial, no legacy remap or admin bypass |
| V2 mention/SDK routing | AC01/05/07; FR001/004/011 | leading actual bot token, no reply-only/public finance |
| V3 durable operations | AC02/04/11/14; FR002/008 | crash boundaries, ten starts/concurrent guild quotas, late completion, unknown blocking |
| V4 PostgreSQL | AC02/03/06/09/14; FR006/010 | isolated upgrade/restart/uniqueness/legacy preservation/RLS |
| V5 financial regression | AC05/06/08/09; FR005/006 | same core math, two owners, copied controls, DM compatibility |
| V6 permission/UI fixtures | AC01/10/12/16; FR003/007/012 | actual SDK masks/select/confirm, elevated guest rejection |
| V7 lifecycle/security | AC11/13/14/15; FR008/009/013 | failed revoke, role drift, remove/add ordering and stale joined_at |
| V8 delivery | all; FR014/015/016, SC001–006 | architecture/docs/scope, four gates, local/live distinction |

All16 FR,16 AC and6 SC are represented; [tasks](tasks.md) owns exact IDs/parallel dependencies.
Planning validation is source/document review only, not proof of implementation or provider behavior.
No new Rust/DB/live test run is claimed by this plan.

## Complexity Tracking

No constitution violations. Required complexity: four durable state/audit tables, two real ports,
one optional guild authorization path and one startup binding for the real shared HTTP dependency.
Rejected: multi-crate/multi-bot deployment, generic ACL/tenant/event frameworks, per-user roles,
new financial engine, broad roster caching, password authentication and custom no-retry HTTP stack.
See [ADR0008](../../docs/architecture/decisions/0008-private-finance-spaces.md).

## Automatic setup amendment

Prepare an automatic category during bounded/interruptible startup using the same Http instance
subsequently moved into ClientBuilder::new_with_http and shared via client.http.0004 stores one
backend-only guild/category binding. Reserve before native creation; null ID means pending/uncertain
and never authorizes a blind retry. A stored ID is verified by existing category preflight, never
adopted by display name. Operators can supply a category override after verifying an uncertain result.
Initial category overwrites deny everyone visibility and allow the bot; children use existing explicit
owner/bot/viewer profiles. No category deletion/move/rename, ledger mutation or new dependency.
An empty onboarding list denotes unrestricted ordinary guild text channels; nonempty retains the
allowlist. Presentation validates channel kind/guild; financial authorization remains channel-bound.
