# Durable Operations and Recovery Contract

Design only. SpaceStore and ChannelDirectory are the two genuine application ports; neither exposes
SDK/SQLx types. Never combine network waits with a held database transaction.

## Store operations

- Reserve/find space by authenticated guild/member; allocate stable journal association and fresh
  generation as needed. Unique conflicts return the same authoritative space, not a second journal.
- Prepare operation and audit atomically; check quotas, expected revision and no other nonterminal op.
  Serialize guild creation-quota/capacity reservation in a short transaction; count pending slots.
- Mark dispatched and audit atomically BEFORE calling provider. Crash between this commit and actual
  HTTP still means uncertain: availability is sacrificed rather than guessing a create was never sent.
- Record candidate/outcome only against current space/operation/revision/generation. Late results
  cannot activate retired or newer generations. Reads never infer ownership from UI names. Replaying ready still checks current membership,
  binding and privacy; a historical ready result cannot revive a retired/unsafe channel.
- Confirm ready/grant/revoke only after full safe observation and settled prior operations. Audit
  change and state transition share one transaction; operation source replay returns prior outcome.
- Invalidate/retire synchronously in durable state before attempting provider cleanup. Financial
  reads/writes authorize anew through application, not a stale grant cached by presentation.

Journal queries/commands stay in ledger. The existing store never directly queries these feature
internal tables; application authorization is an explicit cross-feature dependency, not hidden SQL.

## Creation protocol

| Boundary | Required outcome after failure |
| --- | --- |
| Before reservation commit | No provider create; new request can retry |
| Reserved/prepared, not dispatched | Resume the SAME prepared operation safely |
| Dispatched before network, then crash | Uncertain; reconcile, no blind new create |
| Provider success, DB candidate save fails | Recover exact marker/guild/category/profile, then CAS |
| SDK timeout/ambiguous failure | Uncertain; no claim of absent channel or failed creation |
| Known candidate, privacy verification fails | Suspended/pending, no finance or ready link claim |
| Two/multiple matching artifacts | Quarantine; do not choose first, delete or attach arbitrarily |
| Empty/hidden/obfuscated listing | Unknown, not proof nothing was created |
| Ready, response send fails | Same durable ready binding; retry returns that channel |

Use opaque fgspace:<generation> topic marker and a bounded operation-only audit reason. Marker is
correlation, not authorization: require durable current intent plus exact scope/member/privacy.
All SDK create attempts use private overwrite payloads. There is at most one application dispatch
per dispatched intent; SDK-internal rate-limit retries can still produce multiple private artifacts.
Database uniqueness guarantees at most one active bound channel, not exactly-once physical creation.
Known channel404 or retired-owner recovery can start a fresh generation only on an explicit owner
request; never reuse a retired physical ID. Financial history is preserved, old UI cannot authorize. Retired-generation operations are locally
retired, never described as remote cancellation; new channel grants start empty. Old channel history/
permission cleanup may remain unknown, is disclosed and cannot affect the new physical channel ID.

## Viewer changes and ordering

Prepare desired grant/revoke first; mark space access_pending before mutation. Grant preparation
alone never changes native permissions; confirmation expires five minutes after operation.created_at; owner confirmation must still be current and unexpired.
Only one pending space operation; no opposite write/newer dispatch over an unresolved prior write.
Grant uses exact idempotent member-overwrite PUT; revoke removes only that stored viewer overwrite.
Observe complete canonical profile and member incarnation after write; record confirmed result with
current CAS before reporting success. Timeout is uncertainty; a matching GET alone does not prove an
older remote write is settled. Database fencing cannot stop an already submitted Discord request.
If prior writes may still complete out of order, leave blocked/manual and suppress finance output.
Never display revoked merely because the DB says desired_state=revoked.

## Membership and structural recovery

- Remove owner: retire binding immediately, keep journal; no Add auto-reactivation. Explicit rejoin
  onboarding revalidates identity and creates a new physical channel/generation.
- Remove viewer: invalidate its grant first; reconcile permission removal. Changed joined_at cannot
  restore old access on Add or startup; only new explicit owner approval may grant again.
- Role elevation/permission drift: suspend. Do not silently normalize arbitrary new overwrites.
  Operator restores approved configuration, then bot verifies the complete profile before readiness.
- Delete channel: retain space/journal/history, invalidate original channel and controls. A same-name
  channel has no authority. Explicit owner recovery creates a fresh binding after known deletion.
- Lost visibility: pause. Current Discord behavior can hide channels from listings; restore required
  bot access to an exact managed resource and reconcile. Do not create a replacement merely on absence.

## Manual uncertainty runbook

Owner receives only a safe pending/recovery response. Operator inspects stored operation category,
current generation/candidate and Discord management state without dumping financial content or tokens.
If restoring known managed-channel visibility reveals exactly one safe candidate, normal reconciliation
may resume. Multiple candidates, unresolved late writes or no reliable evidence stay quarantined.
Do not issue raw state resets, delete arbitrary channels or create again to clear a timeout. A later
maintenance decision must record explicit evidence/expected revision and preserve one active binding;
this increment does not add a generic admin console or promise automatic recovery for every outage.
Channel deletion by the operator requires separate explicit authorization and never deletes ledger
history. Private orphan cleanup is not automatic and never deletes channels identified only by name.

## Shutdown/restart

Stop accepting new work before cancellation. Own/join reconciliation tasks within existing bounded
cleanup; interrupted dispatched operations remain uncertain, not failed. Build/startup failure must
not leak background tasks or a pool. On restart, invalidate ready space observations before guild
requests, resume only safe prepared/reconciliation work and never replay dispatched creates blindly.
