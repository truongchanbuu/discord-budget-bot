# Discord Interaction Contract

Design only; examples describe future behavior, not commands already implemented.

## Routing and context

Accept regular human messages in the configured guild, no webhook/attachments/stickers. Require an
explicit leading `<@BOT_ID>` or `<@!BOT_ID>` token matched to the real Ready bot identity and actual
mention metadata. Prefix must be followed by whitespace/end. A reply ping, role/everyone mention,
quoted mention, other bot, or unmentioned message never triggers006. Empty content fails safely.
SDK IDs map to provider-neutral validated refs before application calls; never trust submitted owner IDs.

| Context | Input after mention | Outcome |
| --- | --- | --- |
| Designated onboarding text channel | empty, bắt đầu, bat dau | Reserve/recover/return requester's private space |
| Any public/unrelated channel | financial text | No parsing/persistence/preview; generic private-entry guidance only |
| Requester's ready personal channel | existing005 help/fund/finance/month grammar | Existing review/query semantics on that owner's journal |
| Requester's ready personal channel | quyền, quyen | Owner access-management launcher |
| Retired/suspended/unknown channel | any finance input/control | No financial payload/effect; safe recovery/error guidance |
| Guest or other owner in this channel | financial/access request or control | Ignore ordinary message; controls get generic ephemeral denial; no financial operation |
| Existing configured-owner DM | existing005 input | Unchanged005 route and original DM origin/channel binding |
| Other member DM | existing005 input | Existing denial; multi-user DM is not added in006 |

Ordinary message events cannot receive ephemeral replies. Generic onboarding/status replies are
channel messages; private denials/management responses refer only to interaction callbacks. Do not
send a financial DM fallback or advertise a plain-message reply as private.

New members use guild spaces; existing /fundguard remains the legacy readiness slash command.
No new financial slash command is introduced. Use generic onboarding replies with empty allowed
mentions/suppressed embeds; a private-channel URL is safe metadata, not financial data. Do not echo
user text. Caller-posted public financial text was already public: never claim it was erased.
When a known binding is retired, start presents an owner-only recovery confirmation before a fresh
channel/generation is created. State that old channel access/cleanup may remain unresolved and no
viewers are carried over; no financial history is deleted. Bare/start mention may post a generic pending status before channel creation; no ready claim until
verification. Source replay returns existing ready/pending outcome and never reissues uncertain create.

## Welcome and financial messages

Ready welcome contains: channel owner scope, existing journal reuse/new zero journal, mention examples,
read-only sharing, administrator visibility, full-history sharing and no password guarantee. No
financial report is automatically generated from legacy data during onboarding. Owner-requested
reports in this channel can reveal legacy journal amounts to current viewers; disclose this explicitly.

Examples: @bot trợ giúp; @bot quỹ; @bot nhận lương 15tr; @bot chi 100k ăn uống;
@bot tiết kiệm 3tr; @bot tổng kết YYYY-MM. Strip only the verified bot prefix. Reuse005 bounded
parser/value/date rules. Preview appears in verified personal channel and is visible to its viewers.
Owner alone can Lưu/Sửa/Hủy; all actions re-authorize actor, resource and current active channel.
New source is discord:guild:<message-id>. Existing discord:dm sources remain untouched.

Confirm/cancel/modal submit privately defer before application work (2s). Edit initial modal cannot
follow a defer: complete bounded authorization/lookup+initial response within2s or respond generically
and ask retry. Never fall back publicly. Private interactions still recheck current ownership and
space state; ephemeral delivery is not permission to skip policy. Committed receipt delivery failure
does not retry money with a new source. Privacy changes before output suppress financial content.

## Owner-managed read-only sharing

1. Owner sends @bot quyền in their ready channel; bot posts a generic management launcher, no viewer list.
2. Clicking it opens an ephemeral owner-only management response. Show current confirmed/pending
   grants and Thêm người xem / Thu hồi actions. Guest clicks get generic private denial.
3. Add uses actual SDK User select with one recipient. Server resolves human/current membership,
   joined_at and non-elevated eligibility; user selection never establishes ownership.
4. Before grant, show recipient and warning: they can read past/future posts, cannot write/manage;
   Administrators have independent native access. Owner explicitly confirms or cancels within5min.
5. Revoke selects only stored grants for this space and requires confirmation. Mark pending before
   remote changes; success only after safe readback/current revision. Repeat returns current outcome.
6. Pending/unknown results give owner generic recovery/status guidance. Do not expose member lists
   in public onboarding, trust cached success or permit conflicting late operations.

Confirmation/session state is durable in the prepared operation. Before confirmation, no native
access changes. Cancel/expiry marks that prepared operation failed/cancelled outcome and leaves access
unchanged. Dispatch/timeout cannot be cancelled as if never sent; show uncertain instead. Owner cannot
remove their own/bot required visibility or grant editor/role/everyone access through this interface.

## Component references and output limits

Keep fgfinance for ledger and use fgspace:<action>:<operation-or-space-ref>:<revision> for space UI.
References<=100 characters, no financial data, passwords or account identity embedded. IDs are lookup
references, never credentials; load authoritative actor/space/operation, type, revision and expiry.
Reject malformed/unknown/foreign/stale/used references uniformly. Kind/action is allowlisted.

Retain005 UTF-16 bounds and summary truncation disclosures. Viewer maximum8, select exactly1 human;
no paginated framework needed. Generic failures include denied, pending, configuration/permission
unavailable, timeout/uncertain and rate limited; no token/host/provider JSON/SQL/request content.
No financial data is sent by DM as a fallback for failed guild output or to any unverified channel.
