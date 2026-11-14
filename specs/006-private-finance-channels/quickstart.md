# Validation Guide: Private Finance Channels

Status: implemented; required local checks are recorded in [tasks](tasks.md). Live/native Discord
visibility and hosted CI are NOT RUN by implementation tests. Full contracts: [UI](contracts/discord.md),
[security](contracts/security.md), [recovery](contracts/operations.md), [model](data-model.md).

## Existing credential-free baseline

From repository root, with committed Rust toolchain:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check
scripts/test-postgres.sh
```

Ordinary tests require no live Discord/Supabase account. The separate DB script provisions its own
private PostgreSQL cluster; server/client prerequisites are in005 quickstart. Local script and CI share verify-postgres-suites.sh, which rejects zero matches and runs both
Ledger and Spaces suites.
Never point tests at FUNDGUARD_FINANCE_DATABASE_URL or remove fixture-host safety checks.
Implement upgrade/security/legacy tests in disposable storage, not the user's configured project.

## Optional synthetic live verification — separate operator opt-in

1. Use a dedicated development guild and Supabase development project, with synthetic journals only.
   Retain existing TLS/backend-only DB setup from [005](../005-conversational-finance/quickstart.md).
2. Use three real test accounts: ownerA, memberB/viewer and unrelated ordinary memberC. Test accounts
   must be legitimate Discord accounts; never automate a human account or use a self-bot.
3. Use any accessible ordinary text channel; bot prepares a private category on startup by default.
   Optional overrides can select an existing category or restrict onboarding channels. Assign the bot guild role the
   exact minimum permissions in security contract; do not grant Administrator. Keep managed children
   unsynchronized and avoid unrelated role overwrites. Native admin visibility is a documented exception.
4. In Developer Portal enable **Server Members Intent** for lifecycle safety; keep Message Content
   and Presence intents OFF. Any provider-required access review is an operator prerequisite.
5. No location IDs are required. If desired, set either independent override (placeholders):

```text
FUNDGUARD_FINANCE_CATEGORY_ID=MANAGED_CATEGORY_ID
FUNDGUARD_FINANCE_ONBOARDING_CHANNEL_IDS=ONBOARDING_CHANNEL_ID
```

Existing Discord/guild/owner settings and finance DB URL remain required for enabled finance.
Both006 keys absent enables automatic category setup and unrestricted text-channel onboarding.
Either key works independently; invalid/cross-guild/category-type mismatches fail safely. Use process secrets/your own ignored file, never paste a token or URL into chat/history.
The application does not automatically load env files. After authorized test setup:

```bash
set -a
source .env.local
set +a
FUNDGUARD_MODE=discord cargo run --locked
```

Do not start a second bot alongside an existing process. Root must bind the shared HTTP adapter,
validate006 setup, and reconcile a space before accepting its guild finances. Readiness of the
Discord session is distinct from privacy readiness of every space. No deployment is implied.

## Guided acceptance scenarios

| Step | Action | Expected |
| --- | --- | --- |
| 1 | A sends @bot bắt đầu in any accessible text channel (or configured allowlist) | Private space, welcome/privacy warning, no public financial payload |
| 2 | A repeats; send ten controlled concurrent starts in isolated fixtures | Same ready/pending space; at most one active binding |
| 3 | B onboards separately; C inspects | Separate zero journal; ordinary outsiders cannot see the private channels |
| 4 | A sends @bot nhận lương 16tr in A's channel, reviews and Lưu | One received-income effect; B's own funds remain zero |
| 5 | A records expense6tr, savings3tr, investment2tr through reviews | Total closing10tr, spending-fund5tr; reserve transfers not expenses |
| 6 | A sends @bot quyền, selects B, reviews history warning and confirms | Verified viewer grant; B can read A's past/future posts |
| 7 | B tries post/click/query/manage in A's space | Native posting denied; all bot finance/access actions denied |
| 8 | A revokes B and checks B's client | Revoked only after actual denial; B's own space unaffected |
| 9 | Copy controls to B's space; replay old A confirmation | No cross-owner action or duplicate financial entry |
| 10 | Change permissions/delete a test channel/restart/rejoin in dedicated guild | Fail closed, preserve journal, no old-channel control resurrection |

Also test owner Sửa/Hủy/expiry and actual modal/private responses; public/unmentioned/reply-only input
must not create previews or persist financial notes. Verify earlier configured-owner DM history and
current DM commands remain intact. Explicit new-channel recovery preserves balances and starts with
no viewers carried over; old channel native history/cleanup status must be reported honestly.

## Fault and security evidence

Automated fixtures cover crash before/after dispatch, post-create DB failure, zero/multiple/hidden
marker candidates, stale operation revisions, uncertain grant then revoke, out-of-order Remove/Add,
changed joined_at, elevated recipients, unexpected role overwrites and provider deadline/rate limits.
They prove application safety; fixtures do not establish live provider delivery or exact-once channel
creation. Manual uncertainty runbook is in operations contract; never reset state blindly.

Record only sanitized outcomes/versions/counts. No tokens, real account/channel IDs, channel links,
financial notes or raw provider bodies in repository evidence. Stop/join owned test processes and
clean only explicitly authorized synthetic resources; do not delete private channels automatically.
Hosted CI status, real owner/viewer/outsider visibility and actual modal/recovery checks require
separate evidence. No006 live-provider verification was executed during local implementation.

## Remediation acceptance checks

- Finance stays denied in suspended/retired state, while the current stored owner can request safe
  status and explicit recovery from onboarding; guest/admin/old-channel controls cannot recover.
- Full Spaces authorization precedes Ledger routing: changed incarnation, wrong channel/owner,
  unsafe privacy and disabled legacy binding all deny financial calls/output.
- Abandon prepared sharing for5min, then request a new operation: one expiry audit, freed slot,
  old confirmation denied, no native write. Concurrent confirmation/expiry has one winner.
- Dispatched/uncertain requests never expire into a free slot. Cleanup cannot promote unsafe privacy.
- Each port-expansion checkpoint compiles with all declared methods implemented; no future stubs.

## Local acceptance evidence

All evidence below is local test/review evidence, not a live Discord visibility claim. Tests live
under src/features/finance_spaces/{domain,application,infrastructure,presentation} and the Ledger
scope/guild test files; names below identify the corresponding executable test cases or reviewed path.

| AC | Attributable local evidence |
| --- | --- |
| 01 | successful_onboarding_reuses_the_same_journal_and_channel; production_create_payload_is_private_from_the_first_request |
| 02 | two_guild_journals_preserve_exact_totals_replay_and_postcommit_revocation |
| 03 | concurrent_repeated_reservations_allocate_one_space_and_distinct_journals; concurrent_distinct_members_cannot_exceed_guild_creation_quota |
| 04 | crash_after_provider_success_or_before_ready_recovers_without_another_create; zero_multiple_hidden_or_replaced_candidates_stay_quarantined |
| 05 | guild_authorization_ignores_forged_binding_and_denies_foreign_channel; Ledger request-scope review |
| 06 | two_guild_journals_preserve_exact_totals_replay_and_postcommit_revocation; existing Ledger rollback/history regressions |
| 07 | guild_start_and_access_cannot_fall_through_to_financial_parser; SDK leading-mention fixtures; reviewed pre-parser channel/owner guard |
| 08 | finance_checks_owner_channel_incarnation_and_nonready_management; SDK owner-only select/confirmation fixtures; reviewed absence of admin bypass |
| 09 | existing_legacy_dm_entries_remain_owned_after_guild_onboarding; changing_disabled_configuration_cannot_recover_an_existing_legacy_association |
| 10 | confirmed_viewer_reads_only_and_revocation_restores_owner_only_profile; SDK send/thread masks and one-user selection |
| 11 | elevated_viewer_is_removed_and_uncertain_writes_cannot_be_reversed; durable_viewer_departure_removes_native_overwrite_without_regrant_on_add |
| 12 | sdk_select_requires_one_human_and_confirmation_stays_owner_only; malformed_foreign_or_oversized_control_references_are_rejected |
| 13 | exact_privacy_rejects_drift_missing_and_unknown_facts; known_deleted_channel_recovers_but_unknown_visibility_does_not; SDK disconnect fixture |
| 14 | retired_binding_preserves_journal_and_rejects_old_channel_adoption; fresh_recovery_uses_current_category_and_rejects_old_controls |
| 15 | durable_viewer_departure_removes_native_overwrite_without_regrant_on_add; exact-incarnation nanosecond round-trip/mismatch tests |
| 16 | routes_and_generic_views_never_echo_financial_input; SDK select warning assertion; common privacy text reviewed on every screen |

Additional gates: confirmation_and_expiry_serialize_with_no_expired_dispatch; bounded command counter;
owned_worker_cancels_and_joins_without_provider_access; nonempty DB-suite discovery guard in local/CI.
Three-persona live channel/history/modal/recovery checks and hosted Actions remain explicitly NOT RUN.

## Automatic category recovery

A backend-only0004 record reserves creation before the SDK call and stores the confirmed category ID.
Restart reuses that ID. Unknown create/save outcome leaves a pending reservation and stops startup;
never infer ownership from a category name, clear its row or repeatedly restart to create another.
Verify any resulting category in Discord and set FUNDGUARD_FINANCE_CATEGORY_ID to its ID, or supply
a separate operator-created category. A deleted/hidden stored category also requires a verified
override. This does not move existing personal spaces or expose financial history publicly.
