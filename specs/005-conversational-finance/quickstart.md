# Validation and Local Use

Status: implemented; actual local verification belongs in tasks.md. Live finance/hosted CI unverified.
Do not load the existing real .env.local during development/automated checks.

Run normal fmt, Clippy, cargo test and cargo check without Discord/PostgreSQL credentials. DB-only
cases are explicitly ignored by the ordinary suite and run separately with scripts/test-postgres.sh.
The script creates and removes its own cluster, private Unix socket and synthetic owner data; it must
never use DATABASE_URL/FUNDGUARD_FINANCE_DATABASE_URL or an existing database.

For local finance after implementation, provision a dedicated development PostgreSQL database and
set FUNDGUARD_FINANCE_DATABASE_URL in the trusted process environment. Embedded migrations run at
explicit enabled startup. Without this setting the previous readiness-only bot works unchanged.
Use existing safe local token loading and FUNDGUARD_MODE=discord cargo run --locked; no deployment.

DM “quỹ”; all defaults zero. DM “số dư đầu kỳ 200k”, review and Lưu. DM donation100k, review/correct and
Lưu. Duplicate confirm must return same receipt. Missing amount uses Sửa modal; Hủy changes no money.
Test income16m, expenses6m, savings3m, investment2m: closing10m, available5m, spending6m. Use separate
synthetic testing owner/database; no real financial records. Test monthly carryover and historical
insufficient fund rejection, restart recovery, disabled/unknown account, wrong channel, stale buttons.

Live finance/provider interaction and hosted CI remain unverified unless actually run, separate from
existing readiness smoke. Store only sanitized outcomes; never tokens, account IDs or financial notes.

## Persistence test tools

Install PostgreSQL server/client tools locally or set FUNDGUARD_TEST_PG_BIN to a separately provisioned
server bin directory. Run scripts/test-postgres.sh with the pinned Rust toolchain. The script uses a
private socket-only temporary cluster, synthetic fundguard_test user/database and no password or
existing app URL. On systems with client-only PostgreSQL, it exits with a clear missing-server-tools
message rather than silently skipping DB coverage. CI separately provisions a synthetic service.

Conservative UI limits count UTF-16 units: note1000, category/fund80. Summary displays ten categories,
fund view twelve funds, explicitly labeling omitted rows while totals include everything. Modal kind
is fixed: cancel/resend to change income versus expense. For explicit natural source/destination,
complete the route field rather than relying on inferred direction. Clear credential-like notes
(OTP/password/CVV/PIN labels) are rejected before persistence. No general NLP/secret-detection claim.

## Current verification

All four Rust gates passed;87 ordinary tests passed with8 DB cases explicitly ignored in that command.
All8 DB cases were separately executed and passed on disposable PostgreSQL18.6, including duplicate/
overspend races, injected post-insert rollback, full-u64/fraction checks, immutable history and restart.
Test clusters were stopped/removed. No existing .env.local or real financial records were consumed.
Live finance DM/modal/reply, induced network recovery and hosted CI remain unverified. Prior readiness
live smoke is separate historical evidence and does not certify these new financial interactions.
