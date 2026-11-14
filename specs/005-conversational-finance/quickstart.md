# Validation and Local Use

Status: implemented; actual local verification belongs in tasks.md. Live finance/hosted CI unverified.
Do not load the existing real .env.local during development/automated checks.

Run normal fmt, Clippy, cargo test and cargo check without Discord/PostgreSQL credentials. DB-only
cases are explicitly ignored by the ordinary suite and run separately with scripts/test-postgres.sh.
The script creates and removes its own cluster, private Unix socket and synthetic owner data; it must
never use DATABASE_URL/FUNDGUARD_FINANCE_DATABASE_URL or an existing database.

## Supabase setup

1. Create a dedicated development Supabase project, select an appropriate region, and store its DB
   password securely. FundGuard uses the PostgreSQL password, not anon/publishable/service_role API keys.
2. In the Dashboard's Data API settings, disable Data API **before the first bot startup**. This backend
   uses SQLx only; REST/GraphQL access is unnecessary. Also keep automatic exposure of new tables off.
3. Open **Connect**. Prefer Direct connection5432 if your network supports its IPv6 endpoint; otherwise
   copy **Session pooler5432** for IPv4. Copy the displayed hostname and username verbatim (the pooled
   username includes the project ref). Do not use Transaction pooler6543: prepared statements and
   session migration locks need a session-compatible connection. No Supabase CLI or SDK is required.
4. Append `?sslmode=require` (or `&sslmode=require` if a query already exists). Percent-encode reserved
   password characters. Remote `disable`/`allow` are rejected and default `prefer` upgrades to Require.
   For server identity verification, download the project's CA from Supabase and use
   `sslmode=verify-full&sslrootcert=/absolute/path/to/project-ca.crt`. Never disable TLS to fix an error.
5. Set FUNDGUARD_FINANCE_DATABASE_URL in your trusted ignored local environment. A safe template is
   in .env.example. Keep existing Discord/owner fields; no new Supabase API key is needed. The root
   never auto-loads .env.local. From Bash/Zsh, load only your own trusted file:

```bash
set -a
source .env.local
set +a
FUNDGUARD_MODE=discord cargo run --locked
```

Startup applies versioned migrations using the trusted database/table-owner credential; no manual
pasting of migrations is needed. Migration0002 enables RLS without user policies and revokes all
application-table, sequence and trigger-function grants from API roles. Table owner still accesses
rows; Discord/application authorization is authoritative. No multi-tenant Supabase Auth policy exists.
Do not manually expose financial tables or use this DB credential in a frontend. Keep credentials
server-side; operator owns connection restrictions, backups, data retention and provider logging.

No URL means readiness-only as before; offline ignores database settings. A paused/unreachable project,
wrong password, blocked network or certificate error fails with the safe storage category, never a URL.
Remote startup still has a15s deadline. Check project readiness/network settings privately before retry.
Do not post a connection string to chat or commit it. Actual project/TLS connectivity must be verified
separately; local tests do not certify the hosted service.

Existing local financial data is **not automatically moved** by changing the URL. Preserve the old DB;
a reviewed backup/restore with journal/count/balance checks is required before cutting over real data.
This change neither connects to nor migrates any existing local or hosted records.

Sources: [Supabase connections](https://supabase.com/docs/guides/database/connecting-to-postgres),
[Data API security](https://supabase.com/docs/guides/api/securing-your-api).

## Private finance smoke

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

All four Rust gates passed;90 ordinary tests passed with10 DB cases explicitly ignored in that command.
All10 DB cases were separately executed and passed on disposable PostgreSQL18.6, including duplicate/
overspend races, injected post-insert rollback, full-u64/fraction checks, immutable history and restart.
API-role denial and v1 upgrade/restart also passed. Test clusters were stopped/removed.
Actual Supabase connectivity and Dashboard settings remain unverified. No existing .env.local or real financial records were consumed.
Live finance DM/modal/reply, induced network recovery and hosted CI remain unverified. Prior readiness
live smoke is separate historical evidence and does not certify these new financial interactions.
