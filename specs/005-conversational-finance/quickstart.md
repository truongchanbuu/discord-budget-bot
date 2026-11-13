# Validation and Local Use

Status: design/task artifacts precede implementation; passing results belong in tasks.md only.
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
