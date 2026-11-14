# Feature Specification: Private Finance Channels

**Feature Branch**: 006-private-finance-channels (selector; current Git branch main)
**Created**: 2026-11-14
**Status**: Specification accepted for planning by maintainer's speckit-plan request2026-11-14;
research/design complete, plan review pending. No tasks or implementation generated.
**Input**: Mention the bot to create a personal finance channel; other members can create their own.
Each owner controls who can view their channel. Guests are read-only; finance input still mentions
this bot. Optional password access is deferred in the first increment.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Opening a personal finance channel (Priority: P1)

As a member of the configured server, I mention the bot to obtain a personal finance channel without
asking the operator to make me the single configured owner.
**Why this priority**: Self-service onboarding needs a safe boundary before financial disclosures.
**Independent Test**: Two ordinary members onboard, repeat requests and restart; inspect visibility
using both accounts and an unrelated non-administrator.

**Acceptance Scenarios**:
1. **AC-01**: Given an eligible member, when they send `@bot bắt đầu` or mention only this bot in
   an allowed onboarding channel, then one personal finance channel is created and linked to them.
   Initially only its owner, bot and unavoidable administrative principals can view it.
2. **AC-02**: Given a channel, when requests repeat concurrently or after restart, then the same
   channel is returned. Uncertain creation outcomes never cause blind duplicate creation.
3. **AC-03**: Given another member onboarding, then they receive a different channel and empty
   financial state; neither ordinary member can view or act in the other's channel.
4. **AC-04**: Given missing bot permissions, capacity limits or timeout, when onboarding fails,
   then no publicly visible intermediate channel or successful-ready claim is produced. The
   requester receives a bounded, nonfinancial outcome and can safely retry/recover.

### User Story 2 - Recording finances in the correct channel (Priority: P1)

As channel owner, I mention the bot with financial requests and retain review/edit/confirm/cancel
against my own journal.
**Why this priority**: Multi-user routing must preserve financial integrity and ownership.
**Independent Test**: Two members execute synthetic journal flows, attempt cross-channel queries
and copied controls, then compare isolated balances and durable history.

**Acceptance Scenarios**:
5. **AC-05**: Given my active private channel, when I send `@bot nhận lương 15tr`, then a preview
   appears there; only my valid confirmation posts one received-income entry to my journal.
6. **AC-06**: Given supported expense/opening/transfer/help/fund/monthly input after a bot mention,
   then existing whole-VND, historical nonnegative and reporting rules apply. Sửa/Hủy/expiry preserve
   their meanings; repeated confirmation cannot duplicate a financial effect.
7. **AC-07**: Given unmentioned conversation or mentions outside my active private channel, then no
   financial preview, report or mutation occurs there. Public input is never copied into a journal;
   onboarding replies expose only generic instructions and the requester's private channel link.
8. **AC-08**: Given a guest, different member or administrator invoking an owner's controls, or
   references copied to another channel, then no financial action or private bot payload is allowed.
   Administrative visibility never grants application ownership.
9. **AC-09**: Given existing configured-owner DM history, then onboarding preserves its balances and
   entries under the trusted existing owner mapping. Other members start empty and cannot claim
   that history through names/references. Existing authorized DM behavior remains compatible.

### User Story 3 - Sharing and revoking read-only access (Priority: P2)

As owner, I let selected current server members view my channel and later remove their access.
**Why this priority**: Sharing is explicit, limited and revocable rather than implied by server roles.
**Independent Test**: Grant/revoke another member; inspect actual visibility and owner/viewer/outsider
behavior, including provider failures and restart.

**Acceptance Scenarios**:
10. **AC-10**: Given my channel, when I explicitly add a current member as viewer, then they may
    read its history and future posted responses; this scope is disclosed before confirmation.
    They cannot post messages, request financial operations or manage access through the bot.
11. **AC-11**: Given a viewer, when I remove them, then success is reported only after actual channel
    access is revoked. Failure remains pending/failed, never false success. Uncertain privacy stops
    new financial disclosures until reconciled.
12. **AC-12**: Given a non-owner, malformed target, role/everyone mention, absent member or bot
    recipient, when sharing is attempted, then access stays unchanged and denial is generic. Only
    selected human members may be viewers; repeated grant/revoke is idempotent.

### User Story 4 - Keeping privacy and ownership valid (Priority: P1)

As owner/operator, I need permission changes and channel failure to fail safely without losing money.
**Why this priority**: Discord access can change independently of the application.
**Independent Test**: Change permissions, remove members, delete a channel and restart during setup
or sharing; verify safe recovery and unchanged financial entries.

**Acceptance Scenarios**:
13. **AC-13**: Given unexpectedly broad channel/category access, lost bot visibility or unverifiable
    permissions, then new financial disclosures/actions stop. Recovery requires verified safe access;
    the bot never overwrites unrelated configuration or claims previous exposure was undone.
14. **AC-14**: Given deletion or owner departure, then channel bindings become unusable and history
    is retained. A similarly named channel is never adopted. Explicit owner re-onboarding may recover
    a safe replacement after old bindings are invalidated; old controls stay invalid.
15. **AC-15**: Given owner removal from the server, then future server-channel actions fail even with
    old references. Rejoining with the same authenticated identity may recover only their own space;
    renamed/different accounts cannot inherit it. Removed viewers are not automatically restored.
16. **AC-16**: Given onboarding/sharing confirmation, then users learn that server owner/Administrators
    can see private channels, viewers can read existing history, channel posts are not ephemeral,
    and revocation cannot erase screenshots or information already read.

### Edge Cases

- Bot/role/everyone/reply-only references are not explicit mentions of this bot. Webhooks and bots
  never trigger onboarding or journal operations; actors come from authenticated incoming events.
- Financial text submitted publicly is already visible there. Do not echo, retain or promise to
  erase it; direct the requester to private entry instead.
- A viewer may own a separate space; neither their grants nor financial state merge across spaces.
- Rename/move never changes ownership. Broader inherited access invalidates privacy; channel names
  and requester-supplied owner references are never authoritative.
- Creation retries after uncertain external success and sharing changes survive restart; no
  competing active channels, cross-owner adoption or unverified success announcements.
- Disabled legacy owner bindings cannot be bypassed to recover their history through self-onboarding.
- Failed sends never create new financial source keys. DM drafts remain bound to their original DM;
  guild drafts remain bound to their originating private channel and owner.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Restrict onboarding to current human members of the configured server in designated
  onboarding channels. Require an explicit mention of this bot and supported start input. Ignore
  ineligible servers, bots, webhooks and unrelated messages.
- **FR-002**: Maintain at most one active personal channel per server/member, durable trusted ownership
  and lifecycle state. Deduplicate repeated/concurrent requests and recover uncertain external
  outcomes across restart without public intermediate or competing active channels.
- **FR-003**: Verify restrictive effective visibility before ready status: owner, bot and explicit
  viewers only, subject to server-owner/Administrator bypass. Bot privileges are minimal; do not
  require Administrator or modify unrelated channels/roles to obtain access.
- **FR-004**: Route supported owner financial input only from explicit bot mentions in their active,
  verified private channel. Preserve finite Vietnamese input and review/confirmation; no passive
  server-wide message collection or privileged Message Content requirement.
- **FR-005**: Isolate funds/drafts/entries/origins/reports per owner. Check current authenticated member,
  authoritative ownership and channel on every request/interaction. Channel visibility, Administrator
  or knowledge of references never grants application writes.
- **FR-006**: Preserve exact VND, historical nonnegative balances, transfer conservation, immutable
  history, atomic durable idempotency and date/expiry/source/channel/revision checks from005.
- **FR-007**: Only the channel owner can add/list/remove explicitly selected human viewers. Confirm
  historical/future visibility before grant. Guests cannot send messages, request financial queries,
  confirm/edit/cancel, create funds or manage sharing in that channel.
- **FR-008**: Reconcile requested sharing with actual channel access. Grant/revoke success requires
  verification, survives repeated requests/restart and exposes safe pending/failure outcomes.
  Block fresh financial disclosures whenever channel privacy cannot be established.
- **FR-009**: Detect/reconcile deletion, unsafe permission/category changes, member departure and lost
  bot access. Retain journal data, invalidate stale bindings/controls and require explicit recovery;
  never adopt arbitrary channels or silently restore removed viewers.
- **FR-010**: Preserve offline/readiness/authorized-owner DM behavior and existing history. Associate
  legacy data only through a trusted enabled mapping. No automatic account merging or ownership
  based on username/channel-name similarity or requester-supplied owner references.
- **FR-011**: Public onboarding responses contain no financial values, notes, previews, reports,
  viewer lists or internal owner references. Disclose finances only in verified personal channels
  or existing authorized DMs; failed delivery has no public fallback.
- **FR-012**: Disclose administrative bypass and history-access limits during onboarding/sharing.
  Do not promise that channel permissions or a bot password hide data from authorized readers.
- **FR-013**: Bound input/request rates/external operations, respect provider limits and return safe
  errors. Audit access/ownership changes with actor/origin/outcome without logging financial payloads,
  tokens or passwords. No prohibited banking credentials or real-money transfers.
- **FR-014**: Reuse financial rules through explicit application boundaries in one modular application.
  No duplicate per-user bots, generic tenant framework or business rules in permission handlers.
  Planning must justify any new runtime privilege/dependency.
- **FR-015**: Provide credential-free ownership/isolation/lifecycle tests, isolated durable concurrency/
  restart/financial-preservation tests and real adapter permission/UI fixtures. Report optional live
  owner/viewer/outsider and hosted verification separately; never use real finances in automated tests.
- **FR-016**: Document setup, privacy, mention examples, sharing/recovery and legacy compatibility.
  Maintain authoritative feature/session artifacts with implemented/pending/unverified status.
  Specification work makes no changes to the running bot or live Discord resources.

### Key Entities

- **Member identity**: Authenticated account and current membership in the configured server.
- **Personal finance space**: Server/owner, trusted journal association, authoritative active channel,
  lifecycle and verified privacy state; display names never determine identity.
- **Viewer grant**: Owner-approved human member, read-only scope, desired/confirmed state and traceable
  grant/revoke outcome; no ownership transfer.
- **Provisioning attempt**: Traceable onboarding request with recoverable external outcome belonging
  to one space; retry is not permission to create an unrelated channel.
- **Financial source**: Existing auditable origin scoped to the correct owner/space/channel without
  changing financial rules.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Two independent ordinary members each obtain one separate ready channel; ten repeated
  or concurrent start requests for either member leave at most one active channel for them.
- **SC-002**: All tested non-owner/viewer/cross-channel/copied-control financial attempts produce zero
  financial effects and disclose no additional private payload through bot responses.
- **SC-003**: Every financial fixture reconciles independently per owner. Retry, restart, channel or
  sharing failure produces no duplicate or partial financial effects.
- **SC-004**: All tested grants/revocations match actual visibility before success; ordinary outsiders
  cannot view ready channels, and uncertain/denied privacy stops fresh disclosure.
- **SC-005**: Every onboarding/sharing/privacy screen states administrative and history-access limits;
  none offers passwords or promises protection from server Administrators.
- **SC-006**: All sixteen acceptance scenarios have attributable test/review evidence before delivery;
  relevant verification passes and synthetic/local evidence is distinct from live results.

## Assumptions

- User decisions2026-11-14: guests are viewers only; owner alone manages finances/access; financial
  input still mentions this bot after channel creation.
- First increment supports one configured server, designated onboarding channels and a managed finance
  area. Multiple members are supported; arbitrary multi-server rollout is deferred.
- One journal per owner in that server. Existing configured-owner DM and server space reuse their
  trusted journal; others start empty. DM message history is not copied into channels. Owner-requested
  reports may expose existing journal amounts to currently authorized channel viewers.
- Grant/revoke uses explicit member selection and confirmation. Viewers see past/future channel posts
  but cannot independently request reports. Sharing does not confer financial collaboration rights.
- Visibility uses Discord permissions. Passwords/PINs/access codes, expiry links, co-editors, ownership
  transfer, channel-deletion UI and automatic retention/deletion are deferred.
- Current005 handlers are configured-owner-only and reject guild messages. Simply accepting guild
  messages without verified multi-user authorization would violate this specification.
- Backend-only managed persistence remains; no browser DB access, new public API, bank/AI integration,
  deployment or changes to generated Spec Kit tooling belong to this specification step.
