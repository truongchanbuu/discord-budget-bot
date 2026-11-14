# Data Model: Private Finance Channels

Models, persistence and lifecycle integration implemented; local evidence is in tasks/quickstart. Existing ledger models
and migrations0001/0002 remain unchanged.
All diagnostic formatting redacts identifiers; persisted audit identifiers are not production log fields.

## Provider-neutral values

ServerRef/ChannelRef/MemberRef validate canonical positive account/channel identifiers without SDK
imports. Financial OwnerRef is reused through its existing facade. SpaceId is a positive backend
reference; operation IDs are opaque UUID text. Generation/operation tokens are opaque server-generated
UUID text, not passwords
or authorization credentials. Member incarnation is exact current joined_at; unknown incarnation denies.
Persist owner/target/viewer incarnations as canonical UTC RFC3339 TEXT with exactly nine fractional
digits; infrastructure encodes/decodes without precision loss. SQL CHECKs validate syntax/calendar;
domain/application retain exact DateTime<Utc> equality. Audit/expiry times remain TIMESTAMPTZ.
See [ADR0009](../../docs/architecture/decisions/0009-lossless-member-incarnation.md).

## FinanceSpace — fg_finance_spaces

| Field | Constraint / purpose |
| --- | --- |
| id | Generated positive primary key |
| guild_id, member_id | Canonical nonblank refs, UNIQUE pair; immutable ownership |
| journal_owner | FK fg_owners(owner), UNIQUE; immutable trusted mapping |
| legacy_binding | Exact configured account/OwnerRef association; at most one legacy space per guild |
| owner_joined_at | Last verified owner incarnation; NULL cannot authorize |
| generation | Fresh UUID text for channel creation/recovery; changes invalidate previous generation |
| channel_id | Nullable UNIQUE active-candidate ref; never names as identity |
| category_id | Configured parent, checked against current authoritative channel |
| state | prepared, creating, uncertain, verifying, ready, access_pending, suspended, retired |
| revision | Nonnegative monotonic CAS version, checked addition |
| current_operation_id | Nullable pending operation, at most one outstanding per space |
| created_at, updated_at | UTC audit timestamps |

States other than ready deny guild financial requests. Ready requires channel, owner incarnation,
matching generation and no unresolved dispatched access operation. Prepared unconfirmed sharing
UI may coexist with ready because no remote write has occurred; it cannot grant access or change
viewer rows before confirmed dispatch. No deletion cascade reaches ledger history.
New refs are fresh owner rows; insertion collision fails. Legacy owner row may already exist and is
reused only for the exact enabled configured identity. Conflicting existing association blocks setup;
configuration changes never silently transfer old journal history. Enabled legacy conflicts fail
connected startup before guild or DM financial handling, not just optional guild routing. Others initialize zero funds via005.

## SpaceOperation — fg_space_operations

| Field | Constraint / purpose |
| --- | --- |
| id | Server-generated opaque unique operation reference |
| space_id | FK space, authoritative parent |
| kind | create, grant_viewer, revoke_viewer |
| source | Bounded trusted message/interaction origin, UNIQUE(space_id,source) |
| actor | Authenticated initiating member, no payload/note |
| generation, expected_revision | Immutable binding/version for this operation |
| target_member, target_joined_at | Required only for sharing; exact validated target |
| desired_access | viewer/absent for sharing, never editor |
| phase | prepared, dispatched, uncertain, verifying, confirmed, failed, retired |
| returned_channel_id | Candidate returned by create, not ownership proof alone |
| created_at, dispatched_at, completed_at | UTC lifecycle; dispatch is committed before provider call |
| outcome | Safe category only, no raw API/SQL error or financial content |

Partial unique constraint permits at most one nonterminal operation per space. Store transitions
take the guild quota lock for reservation, then lock space row and operation; apply CAS expected revision/generation and reject stale completions.
Use a short guild advisory transaction lock when counting/reserving creation quota and capacity;
pending/uncertain reservations count toward the50-space ceiling. No DB lock spans provider await. Source replay returns the prior outcome, never a new create operation.
Definite pre-dispatch failures can start a safe retry; dispatched/unknown creates never auto-reissue.
Retain old confirmed create records to identify retired IDs: a retired physical channel cannot be
reactivated by a matching display name, new membership or repeated start request.

## ViewerGrant — fg_space_viewers

| Field | Constraint / purpose |
| --- | --- |
| space_id, generation, member_id | Composite primary key, FK space; member distinct from owner/bot |
| member_joined_at | Incarnation explicitly approved by owner |
| desired_state | viewer, revoked |
| confirmed_state | pending, viewer, absent, uncertain |
| revision | Monotonic per-grant version |
| operation_id | Latest authoritative grant/revoke operation |
| granted_by, changed_at | Actor and UTC audit |

Only current-generation grants affect readiness; replacement channels start without carried-over viewers.
Retain revoked/old-generation rows: departure/rejoin or delayed Add cannot resurrect an old grant. At most8 desired
viewers per space. Current humans only; screening-pending or elevated recipients are rejected.
Grant/revoke success requires actual provider observation, settled prior writes and current CAS.

## AccessEvent — fg_space_access_events

Append-only space/operation/source/actor/action/outcome/time audit. No financial payload, token,
password, full provider body or unbounded external string. Actions include reservation, verification,
grant/revoke outcome, membership invalidation, privacy drift, retirement and explicit recovery.
One logical transition produces one event in its DB transaction; retries do not duplicate that event.
Append-only enforcement and indexes are part of migration/test work, not code generated by this plan.

## Derived snapshots — not durable authorization

ChannelSnapshot: provider channel/guild/parent/type, expected marker, canonical allow/deny principals,
bot capabilities, observation time. Adapter derives facts from real SDK records; domain evaluates
whether they satisfy the expected space policy. Unknown/obfuscated/missing fields are unsafe.
MemberSnapshot: authenticated account/current membership/joined_at/screening and elevated flags.
FinancialAccess: enabled journal binding/channel/generation/revision after authorize_finance; each
financial request and later response rechecks current membership/privacy. Not a long-lived credential.
OwnerManagementAccess: stored owner identity/space/generation/revision/incarnation for nonfinancial
status/cancel/recovery; no journal binding, never accepted by Ledger. Recovery preparation persists
fresh owner_joined_at and increments revision without ready status; confirmation rechecks both.

## Lifecycle transitions

Space: prepared → creating → verifying → ready. Dispatched ambiguity → uncertain; privacy/membership
failure → suspended. Grant/revoke starts ready → access_pending → verifying → ready only if settled
and exact. Deletion/owner-incarnation loss → retired; explicit owner recovery creates a fresh
operation/generation/channel under the SAME stable journal mapping. Never reactivate old channel IDs.
Channel/role/gateway changes may send ready → verifying/suspended before further guild acceptance.

Operation: prepared → dispatched → verifying → confirmed; dispatched uncertainty → uncertain.
failed means a classified definite failure, or expired/cancelled prepared sharing confirmation,
never assumed failure from timeout. Expiry is only prepared grant/revoke with dispatched_at=NULL
and now>=created_at+5min; atomically clear the matching pending pointer and audit once. Do not expire
creation/dispatched/uncertain operations or promote space privacy state. Confirm/cleanup use CAS. Late responses must
match current operation/generation; stale local completion never marks a space ready. Retiring a
known channel binding may mark its operations retired (not remotely cancelled) so explicit recovery
can use a fresh channel safely. Unsettled old-native access is retained as unknown, never revoked-success.

Viewer: absent/revoked → pending → viewer after owner confirmation and verified overwrite. Revocation
moves through pending to absent only after readback. Departure/changed joined_at/elevation invalidates
before remote reconciliation; pending/unknown blocks fresh disclosures. Add never regrants access.

## Migration and security

Add migrations/0003_private_finance_spaces.sql and register in platform/database.rs. Four tables enable RLS. Revoke PUBLIC and existing anon/authenticated/service_role privileges on
owned tables, sequences and audit trigger functions, matching backend-only0002. No client policies or new Supabase API keys. Application tests assert constraints,
RLS/grants, append-only audit, legacy-owner upgrade and existing immutable entries unchanged.
No financial table rewrite/backfill/source renaming is needed. Feature-disabled behavior remains005.

Space errors use static categories (denied, invalid, rate-limited, capacity, pending, permission,
conflict, storage, timeout, uncertain). They retain no raw provider source or user/financial payload.

Internal invalidated-grant cleanup records the authenticated bot actor. It may confirm a specific
removed overwrite with space still suspended; only complete privacy verification makes it ready.
Operation identity/source/target/generation/revision and an already recorded candidate are immutable
in the database. Future UI intent never authorizes a financial use case.

## Automatic category binding (0004)

fg_finance_categories stores canonical guild_id primary key, nullable unique category_id and
reserved_at. It is backend-only with RLS and API-role grants revoked. Atomic insert elects one
creator; NULL remains uncertain after interruption and does not permit another create. Recording
category_id is CAS from NULL; restart reads the stored ID and validates the authoritative category.
No financial payload, no name-based adoption, and no edits to existing spaces/journal ownership.
