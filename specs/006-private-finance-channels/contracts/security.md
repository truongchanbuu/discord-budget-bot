# Security and Authorization Contract

See [research](../research.md) for primary sources. Product privacy is conditional on Discord's
permission system; it is not encryption or secrecy against server Administrators.

## Bot capabilities and exact overwrite profiles

Operator configures a managed category; bot does not create/mutate unrelated server roles/categories.
At guild level, bot needs MANAGE_CHANNELS/MANAGE_ROLES and the bits it will set in allowed/denied masks.
Required message/read bits: VIEW_CHANNEL, SEND_MESSAGES, READ_MESSAGE_HISTORY. To deny viewer thread
posting also require CREATE_PUBLIC_THREADS, CREATE_PRIVATE_THREADS, SEND_MESSAGES_IN_THREADS.
These requirements are preflighted; missing capability gives a safe setup error. Never grant Admin,
request Manage Messages/View Audit Log as a convenience, or construct a deny-all permission mask.
MANAGE_ROLES remains inherited; it is not included in initial channel overwrite payloads.

| Principal | Allow | Deny |
| --- | --- | --- |
| everyone guild role | none | VIEW_CHANNEL |
| bot member | VIEW_CHANNEL, SEND_MESSAGES, READ_MESSAGE_HISTORY | none |
| owning member | VIEW_CHANNEL, SEND_MESSAGES, READ_MESSAGE_HISTORY | none |
| each confirmed human viewer | VIEW_CHANNEL, READ_MESSAGE_HISTORY | SEND_MESSAGES, CREATE_PUBLIC_THREADS, CREATE_PRIVATE_THREADS, SEND_MESSAGES_IN_THREADS |

Create GUILD_TEXT with this restrictive set in the initial request; never create publicly then edit.
New channel is deliberately unsynchronized with the parent category. Do not copy other role/member
overwrites. No additional overwrite is acceptable, even if its target appears harmless from a cache.
Exact set equality, guild/type/parent/marker and required bot access prove ordinary outsider denial;
role ordering alone does not. Fresh member roles and current guild role facts supplement this proof.
Unknown/obfuscated/partial/mismatched data cannot establish a safe channel.

A viewer's native read-only grant is not enforceable against server owner/Administrator or effective
MANAGE_CHANNELS/MANAGE_ROLES/MANAGE_WEBHOOKS recipients; reject these targets with truthful explanation.
Role elevation invalidates the grant and blocks fresh disclosure while reconciling. An Administrator
still has independent native access even if their stored viewer grant is removed. Do not claim native
read-only prevents all reactions/UI actions; it prevents messages and financial application operations.

## Application authorization independent of visibility

Authorization has three explicit entry policies; there is no universal ready-state guard:

- Onboarding requires a current authenticated human in the configured guild/onboarding channel.
  Reserve ownership from that actor, never from an owner reference submitted in a message.
- authorize_finance requires current membership/incarnation, authoritative active channel/space,
  ready privacy, enabled trusted journal association and exact owner equality. Apply it to every
  financial query/mutation/control and again before fresh guild financial output.
- authorize_owner_management requires current human membership and stored owner equality, but may
  return nonfinancial status/list/cancel/recovery context while the space is pending, suspended or
  retired. It returns no journal binding and cannot authorize any Ledger call. Status/list responses
  containing access metadata use owner-only interactions; public guidance stays generic.

Recovery starts only in a configured onboarding channel with a fresh owner request. It may prepare
recovery for a known retired/deleted binding, never treat lost visibility as deletion or reuse old
channel controls. Preparation records the fresh owner incarnation and bumps space revision without
making the space ready; confirmation must still match that incarnation/operation/generation/revision.
A membership mismatch rejects the old confirmation; only a new explicit request can prepare again.
Grant requires ready privacy/no unresolved operation. Revoke may reduce a stored viewer's access in
ready/suspended/verifying state if the physical binding is known and no older write is unresolved;
otherwise return pending/manual. Management authority never overrides access-write serialization or financial readiness.
Trusted lifecycle reconciliation is an internal service entry, not user management authority; it may
invalidate/reconcile access but never create financial entries or auto-approve an invitation.

Guests/other owners cannot invoke finance, list another space's grants or manage its access. Server
Administrators cannot bypass application ownership. Member selections/control refs never establish
identity or permission. Reject foreign/stale controls uniformly; expired prepared cancellation/status
cannot disclose financial payload. Existing configured-owner disabled binding denies both policies.
Conflicting operator remap of an established legacy association fails enabled connected startup
before financial handling. Existing authorized DM policy remains unchanged.

JournalStore continues authoritative owner/source/channel checks and immutable atomic posting. No
space operation writes financial entries. Recheck space/privacy before fresh guild financial output;
if blocked after commit, retain commit and identify source replay later without repeating effect.
Checks are point-in-time: Discord permission changes cannot be atomic with database commit/send.
Already-authorized in-flight work may finish; newly started requests always recheck current access.

## Membership and lifecycle

Use GUILD_MEMBERS only for tracked lifecycle/role events, not full roster retention. Fresh point GET
validates current membership, joined_at and screening status. Unknown/missing incarnation or pending
screening denies guild authorization. Remove invalidates before remote reconciliation. Add never
restores a viewer; changed incarnation needs explicit new grant. Duplicate/out-of-order events may
cause conservative suspension, never a late authorization restoration.
Startup/Ready/reconnect marks tracked spaces unverified; fresh observations are required before use.
Current channel/role/category changes invalidate affected spaces. Recheck all tracked owner/viewer
incarnations during verification, bound concurrency and fail closed on deadline. No negative inference
from absent cache/list entries. Owner departure retires the channel binding, not journal data.
Never reuse a retired channel ID; explicit recovery preserves journal but creates a fresh channel.

## Operational trust and residual risks

- Server owner/Admin can view/change private channels, and authorized readers can copy history.
- No bot can retroactively undo exposed posts or guarantee native access revocation during downtime.
- Unexpected permission changes between verification and send are an unavoidable provider race;
  invalidate promptly, stop new disclosure and never describe this as atomic confidentiality.
- Discord permission writes have no revision/CAS; unresolved older writes require pending/manual state.
- Creation has no remote idempotency key; private orphan artifacts can exist but cannot become a
  competing active financial space. Restore bot visibility/reconcile exact candidate; never adopt by name.
- Optional live evidence must use a dedicated development guild/project and synthetic data. No global
  permission/intent change, cloud provisioning, token use or channel operation occurs in planning.
- Backend credentials/RLS and existing secret/financial logging policy remain unchanged. Audit tables
  hold necessary actor/source/access outcomes; application logs contain only safe event categories.
