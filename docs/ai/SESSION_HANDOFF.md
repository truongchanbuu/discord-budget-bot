# Session Handoff

Updated: 2026-11-14. Active delivered feature:
[005 Conversational Personal Finance](../../specs/005-conversational-finance/spec.md).

## Phase, authority and progress

Maintainer explicitly authorized specify/review/plan/tasks/analyze/implement in one request. Spec
quality16/16, constitution review and cross-artifact coverage passed. All37
[tasks](../../specs/005-conversational-finance/tasks.md) are verified; that file owns detailed evidence.
Plan/research/model/contracts/guide and ADRs0006/0007 reflect delivered behavior. Supabase storage amendment authorized2026-11-14; no new feature started.

## Implemented behavior and decisions

Optional FUNDGUARD_FINANCE_DATABASE_URL in explicit Discord mode enables Supabase-managed PostgreSQL journal and
owner-only DM FinanceEvents. Without it, readiness-only behavior remains; offline ignores bot/DB
settings. Root never auto-loads .env files. Existing real .env.local was never read or changed.
Readiness points to DMs only when finance enabled. Prior core semantics and identity policy retained.

Finite Vietnamese phrases produce persisted 15min previews; Lưu/Sửa/Hủy require current owner,
original DM channel, revision and authoritative state. Missing/ambiguous money requires correction.
Actual SDK builders, private acknowledgment-before-action and five-field modal handling are tested.
No collectors, public finance, Message Content privilege, attachments, AI or reverse identity dependency.

Ledger feature owns fund register/journal/month query. Default Chi tiêu/Tiết kiệm/Đầu tư start zero;
manual income/expense, one-time opening and equal two-sided internal transfer use whole VND. SQLx
owner lock + unique source + immutable entry + one transaction prevents duplicate/partial effects.
Full historical fold rejects backdated underflow/overflow. Original and confirming actors/source/time
are audited without logs. Bank-secret labels rejected from notes/fields; no universal detection claim.

Monthly view separates received/consumed/reserve transfers, initial+carryover and total/spending closing.
Core amount u64, checked report u128, integer floor ratios or unavailable for zero income. Vietnam
calendar UTC+07; canonical dates only. Views show max12 funds/10 categories with disclosed omissions.
No committed corrections/deletion, portfolio valuation, planned/recurring salary or bank proof.

## Verification and limits

PASS: cargo fmt --all -- --check; cargo clippy --all-targets -- -D warnings; cargo test; cargo check;
explicit offline cargo run --locked (stdout empty), locked dependency feature tree and parsed CI YAML.
90 ordinary tests passed;10 DB tests are explicitly ignored there and all10 passed separately through
scripts/test-postgres.sh on private disposable PostgreSQL18.6. Includes same/different-source races,
rollback after entry insert, whole-u64/fractional constraints, immutable entries, committed/pending
restart, owner/channel/revision/expiry, monthly reconciliation, v1 upgrade and API-role denial. Test clusters cleaned completely.

Checks used one Cargo job and disabled dev/test debug info. Server/client prerequisites apply only to disposable DB tests;
locally server binaries/libraries were extracted non-root outside repo (no system installation).
Script only creates its own socket-only fixture and never consumes an existing app URL/database.
CI now has a separate synthetic PostgreSQL job; hosted execution remains unverified.

Reviewed diff, sizes<=250, links, forbidden SDK/SQLx imports in domain/application, source-key/scoping,
no feature cycle, safe diagnostics and minimal features. Generated tooling/constitution and Financial
Core implementation unchanged. Identity gained only a crate-private audit accessor and immutable Arc
sharing/capability text; no ledger import. Branch main; initial clean HEAD2b3361f, spec commit02647e7.

New finance DM/component/modal delivery is NOT live-tested. Previous004 live auth/registration/private
readiness/shutdown evidence does not verify new financial interactions. No actual local secret file,
personal financial data, bank integration, real transfer or production deployment was used.

## Pending work and next action

No partial/blocked implementation tasks. To use finance, configure a dedicated development Supabase
project, disable Data API before first startup and set its direct/session5432 DB URL, then run the bot; [guide](../../specs/005-conversational-finance/quickstart.md).
Optional live finance smoke must use synthetic transactions in a dedicated development database and
report owner/denial/modal/private delivery/restart separately. Do not claim hosted CI passed locally.
Next separate spec may add immutable reversal/correction and richer category/fund administration.
Do not silently extend payments/bank/AI/production scope. Read FEATURE_INDEX and current code/tasks
at the next session; chat history is not project state.

## Supabase amendment

ADR0007 retains SQLx/schema/financial rules, with no Supabase SDK/Auth/API key. Remote TLS required;
VerifyFull with project CA is supported. Known Supabase transaction6543 endpoints are rejected.
Migration0002 enables RLS and revokes PUBLIC/API-role grants on backend objects and migration metadata;
table-owner credential remains backend-only. Existing migration0001 unchanged. No real .env.local read,
no hosted connection or data migration. Preserve any old database until a reviewed backup/restore.
Actual Supabase project settings/connectivity remain unverified; this is operator setup pending,
not an unfinished code task. All37 tasks complete;90 ordinary +10 isolated DB tests passed locally.
