# Validation Guide: Private Finance Channels

Status: planned, not implemented. Configuration/commands/tests below are implementation acceptance
instructions, not a claim006 can run now. Do not use existing live credentials or mutate live channels
while generating planning artifacts. Full contracts: [UI](contracts/discord.md),
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
private PostgreSQL cluster; server/client prerequisites are in005 quickstart. Keep that script and CI
filter updated to include finance_spaces DB tests when implemented; do not accidentally run only005.
Never point tests at FUNDGUARD_FINANCE_DATABASE_URL or remove fixture-host safety checks.
Implement upgrade/security/legacy tests in disposable storage, not the user's configured project.

## Optional synthetic live setup — after authorized implementation

1. Use a dedicated development guild and Supabase development project, with synthetic journals only.
   Retain existing TLS/backend-only DB setup from [005](../005-conversational-finance/quickstart.md).
2. Use three real test accounts: ownerA, memberB/viewer and unrelated ordinary memberC. Test accounts
   must be legitimate Discord accounts; never automate a human account or use a self-bot.
3. Operator creates an onboarding text channel and managed category. Assign the bot guild role the
   exact minimum permissions in security contract; do not grant Administrator. Keep managed children
   unsynchronized and avoid unrelated role overwrites. Native admin visibility is a documented exception.
4. In Developer Portal enable **Server Members Intent** for lifecycle safety; keep Message Content
   and Presence intents OFF. Any provider-required access review is an operator prerequisite.
5. In trusted local environment set both optional006 fields (placeholders, not real IDs):

```text
FUNDGUARD_FINANCE_CATEGORY_ID=MANAGED_CATEGORY_ID
FUNDGUARD_FINANCE_ONBOARDING_CHANNEL_IDS=ONBOARDING_CHANNEL_ID
```

Existing Discord/guild/owner settings and finance DB URL remain required for enabled finance.
Both006 keys absent leaves005 untouched; partial/invalid/cross-guild/category-type mismatch fails
safely. Use process secrets/your own ignored file, never paste a token or URL into chat/history.
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
| 1 | A sends @bot bắt đầu in configured onboarding channel | Private space, welcome/privacy warning, no public financial payload |
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
separate evidence. No verification was executed against a live provider by this planning step.

## Remediation acceptance checks

- Finance stays denied in suspended/retired state, while the current stored owner can request safe
  status and explicit recovery from onboarding; guest/admin/old-channel controls cannot recover.
- Full Spaces authorization precedes Ledger routing: changed incarnation, wrong channel/owner,
  unsafe privacy and disabled legacy binding all deny financial calls/output.
- Abandon prepared sharing for5min, then request a new operation: one expiry audit, freed slot,
  old confirmation denied, no native write. Concurrent confirmation/expiry has one winner.
- Dispatched/uncertain requests never expire into a free slot. Cleanup cannot promote unsafe privacy.
- Each port-expansion checkpoint compiles with all declared methods implemented; no future stubs.
