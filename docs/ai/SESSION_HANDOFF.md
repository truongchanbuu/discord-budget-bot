# Session Handoff

Updated:2026-11-14. Phase:006 Private Finance Channels — completed and locally verified.
Active [spec](../../specs/006-private-finance-channels/spec.md),
[plan](../../specs/006-private-finance-channels/plan.md),
[authoritative65-task checklist](../../specs/006-private-finance-channels/tasks.md).
All65 tasks, including automatic setup and the live-found permission fix, are locally verified; no pending implementation tasks. Local configuration was subsequently read under explicit live-test authorization; values were not
printed or changed. Read-only Discord REST checks and a readiness-only Gateway session ran; no
finance/database operations, deployment, commit or push performed.

## Delivered behavior

Financial database configuration enables `@bot bắt đầu` personal channels by default; location IDs
are independent optional overrides.
owner-only existing financial interactions and `@bot quyền` private viewer management. Each member
gets one stable journal association; the enabled exact legacy owner retains existing DM data.
Viewers can read native history/posts but cannot invoke finances or manage access. No passwords,
shared editing, banks/AI, new financial rules, per-user bots or HTTP API were introduced.
Prepared confirmation expiry, durable dispatch/CAS/audit, restrictive initial masks, lifecycle
invalidation/native cleanup and explicit fresh-channel recovery fail closed on uncertainty.

## Decisions and boundaries

Accepted [ADR0008](../architecture/decisions/0008-private-finance-spaces.md) records two real ports,
acyclic ledger → finance_spaces → identity/core and shared client.http/lifecycle ownership.
Spaces uses only the approved shared fg_owners anchor, never entries/funds; Ledger SQL never reads
space tables. Application/domain have no Discord/SQLx imports. Source files stay within250 lines.
Accepted [ADR0009](../architecture/decisions/0009-lossless-member-incarnation.md) stores membership
incarnations as canonical UTC RFC3339 TEXT with9 fractional digits. Exact owner/viewer/target
comparisons remain intact; audit/expiry timestamps remain TIMESTAMPTZ. Unreleased0003 and additive0004 contain006 storage;
0001/0002 and locked dependencies remain unchanged. This does not repair precision already lost
in any external database; no external migrations were applied during this work.

## Verification evidence

- cargo fmt --all -- --check:PASS.
- cargo clippy --all-targets -- -D warnings:PASS, no warning suppression.
- cargo test:148 ordinary PASS;27 DB tests intentionally ignored there.
- cargo check:PASS.
- FUNDGUARD_MODE=offline cargo run --locked:PASS, application_initialized, exit0.
- scripts/test-postgres.sh:PASS,12 Ledger+15 Spaces cases on disposable socket-only PostgreSQL18.6;
  cluster cleaned by trap. CI and local scripts share nonempty suite discovery/checks.
- Shell syntax, documentation links, diff whitespace, source sizes, dependency direction, migration
  preservation and credential/logging review:PASS. All16 ACs have attributable local test/review
  evidence in [quickstart](../../specs/006-private-finance-channels/quickstart.md).

Historical partial checkpoints in devlog are superseded by this completion. Tests specifically
cover nanosecond persistence, two-owner isolation/legacy DM preservation, crash recovery/no blind
create, expiry/dispatch races, native viewer cleanup, quotas, SDK controls and owned worker shutdown.
Temporary tool output/server tools remain under /tmp; no evidence logs were added to the repository.

## Unverified limits and next action

Hosted Actions, live Supabase migration, three real Discord personas/native visibility, real
modal/history/recovery behavior are NOT RUN. Configured CI is not a hosted passing run.
Administrator visibility, copies of already-read history and the non-atomic provider/database race
remain explicit limitations. Unknown native writes block disclosure; old-channel cleanup may remain
uncertain and recovery never inherits viewers or silently deletes old channels.
Next: review local diff and optionally authorize synthetic live validation through the006 quickstart.
Use documented environment keys/privileges and enable GUILD_MEMBERS; MESSAGE_CONTENT stays off.
Do not start another feature without its own Spec Kit scope; do not deploy automatically.

## Authorized live smoke — 2026-11-14

Maintainer requested real Discord testing. Authenticated bot REST and configured-guild access PASS;
readiness-only Gateway startup reports discord_ready. No existing local bot process was running.
Database URL and006 keys were excluded from the child process, so no Supabase connection/migration
or financial operations occurred. Token/config values were not logged or copied into tracked files.
The configured bot role lacks Manage Roles; the application does not advertise Server Members intent
flags. At that smoke both006 location settings were absent and the prior revision required them. The
automatic-setup amendment removes that requirement; permission/intent prerequisites still apply.
A human owner slash-command check was requested; no real user response is yet confirmed in this
record. Native three-persona privacy, financial DM/guild operations and Supabase remain unverified.
Read-only REST also confirmed exactly one configured-guild /fundguard command of chat-input type.
Owned readiness session shut down cleanly with SIGINT, exit0; no bot process left running.
Only documentation changed during this smoke; diff whitespace check PASS. Prior Cargo results above
remain the implementation baseline, not newly rerun tests. Next action is enabling the bot permission/intent prerequisites,
then repeat live006 tests; no implementation fix was needed for this readiness smoke.

## Automatic-setup amendment complete

T057–T063 verified. Location overrides are independent; no IDs means automatic category setup and
unrestricted onboarding/recovery in accessible ordinary text channels. Startup uses one shared Http,
private initial category masks, a durable0004 reservation and stored ID reuse across restarts.
Unknown native create/save outcome or missing/hidden bound category stops startup; operator can
supply a verified category override. Existing personal channels/journals are never moved or merged.
Tests include exclusive concurrent category reservation, backend-only grants/RLS, persisted ID reuse,
configuration defaults/independent overrides, unrestricted recovery and public financial denial.
No live006 retest or Supabase migration performed for this amendment; prior live-smoke findings are
historical. Only latest observed Manage Roles/Server Members capability remain setup prerequisites;
no category/onboarding ID is required. Existing four Cargo gates/offline and updated27 DB cases pass.

## Live006 attempt after operator permission update — 2026-11-14

Explicit operator request continued the real Discord test. Read-only REST confirms Manage Roles,
Manage Channels and all required view/send/history/thread bits; Administrator absent; Server Members
intent application flag enabled. No pre-existing local bot process. Current binary loaded ignored
local configuration without printing/changing secrets. This attempt included the configured Supabase
connection: migrations1–4 are now successfully applied (verified read-only). Do not describe live
migrations as unexecuted for the current state; earlier NOT RUN statements are historical evidence.

Full startup exited1 with safe client_build_failed before Gateway readiness. Read-only database
inspection confirms this attempt durably reserved automatic category creation, with category_id NULL.
The underlying creation error was mapped to a generic category and no safe HTTP status/code was
retained, so the provider cause is UNKNOWN. REST listing shows no recently created visible channel;
this does not prove native absence and does not justify clearing/resubmitting the reservation.
No personal channel or financial mutation was executed by this test. No bot process remains running.

Current live blocker: unresolved automatic-category reservation. Do not restart blindly, clear its
row, adopt a same-name category, or claim a successful live006 flow. A human inspection/category-ID
recovery question is pending. A verified category override is the documented recovery mechanism.
Future diagnostics should retain safe status/code for create failures without provider bodies or
secrets. Hosted CI/native three-persona/financial interactions remain unverified. Local63-task/147+27
verification remains unchanged; only documentation changed in this live-test turn; diff check PASS.

## Live-found failure diagnosed and fixed — 2026-11-14

User confirmed no new channel visible and explicitly authorized ONE diagnostic creation attempt.
That single POST returned HTTP403/Discord50013; no category was created by this request. Guild
permission bits/intents are correct; operator is server owner and guild MFA requirement is0.
Official Discord Create Guild Channel docs say setting MANAGE_ROLES in a channel overwrite requires
Administrator. The category builder redundantly set that bit; this is a code defect, not another
operator permission prerequisite. Removed that channel-level assignment, retaining inherited guild
MANAGE_ROLES and initial privacy. Added numeric/status-only safe create diagnostics and regression.
Expected red reproduced the forbidden bit; final fmt/Clippy/test/check/offline PASS,148 ordinary+27
isolated PostgreSQL PASS. T064/T065 done. No additional migration or financial changes.

The original pending automatic-category row is preserved. The ONE authorized live trial is used;
do not send another create until operator approves a corrected single recovery attempt. Once approved,
create with corrected private profile, verify returned ID/guild/type/exact overwrites, CAS-bind only
the matching pending guild row, then start the corrected bot. Never delete/reset the row or adopt
by name. No bot currently running. Human channel/privacy/finance flow remains unverified.

## Approved corrected live recovery — 2026-11-14

Operator explicitly approved one further corrected create and verification/binding/startup.
Corrected private category POST returned201; GET verified exact guild/type/everyone-deny/bot profile.
Guild-scoped CAS populated exactly one pending category row; it was never deleted/reset. No financial
mutation. No new location env values were required or written. A separately started old local bot
using a deleted binary was detected, gracefully stopped to avoid duplicate handlers, then current
binary started successfully: application_initialized, discord_ready. Owned current session36986;
log /tmp/fundguard-live-spaces-corrected.log (private, sanitized events only in tracked evidence).
Bot is intentionally left running for the authorized interactive check; do not start another one.

User reported a private channel/welcome. Independent read-only check found0 finance_spaces for the
configured guild, so personal-channel onboarding is NOT YET CORROBORATED. The category's creation
must not be counted as a personal channel. User was asked to send an actual leading bot mention plus
bắt đầu in an accessible ordinary text channel and check for #tai-chinh inside the category.
Help/access-panel response and actual owner/viewer/outsider visibility remain pending. No financial
entries were posted by these checks. Prior blocker is resolved for category startup only; update
this record after observable human/DB/native-channel evidence agrees.

## Final live onboarding evidence — 2026-11-14

Maintainer confirmed the corrected `@bot bắt đầu` flow completed. A subsequent read-only Supabase
query found exactly one configured-guild finance space in ready state with no pending operation;
it preserves the configured legacy journal association. Authoritative Discord readback confirms the
stored personal channel belongs to the stored category and has the exact everyone-deny plus owner/bot
allow profile. This supersedes the earlier zero-space checkpoint. Category recovery, Gateway startup
and owner onboarding are verified live. No financial entry was created during this validation.

Bot process72637 remains intentionally online from this authorized live session. Do not start a
second local process. Viewer grant/revoke with a second account, outsider visibility, help/access
controls, and hosted CI remain unverified. The repository is ready for the maintainer-authorized
commit and push; no credentials or local `.env.local` content belong in the commit.
