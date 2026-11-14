# Implementation Plan: Conversational Personal Finance

**Status**: Implemented and locally verified; see tasks.md for exact evidence and limits.
**Date**: 2026-11-13. **Feature**: [spec.md](spec.md). Actual Git branch: main.
**Authorization**: Maintainer explicitly requested specify → analyze → plan → tasks → implement;
review gates are covered by that request. Analysis runs after design/tasks prerequisites exist.

## Summary

Add owner-only finance DMs with finite Vietnamese parsing, persisted 15-minute previews, edit modal,
confirm/cancel buttons, received income/consumption, virtual fund transfers/opening and monthly queries.
Keep existing offline and guild readiness compatible. No AI/banking/real transfer/deployment.

## Technical Context

- Rust 1.99.1 stable/Edition 2024, existing Tokio/Poise0.7.1/Serenity0.12.5/tracing/thiserror retained.
- SQLx0.9.0 defaults disabled: runtime-tokio, tls-rustls-ring, postgres, chrono, migrate. Dynamic query/Row,
  no schema-dependent query macros, no SQLite/Any/JSON/decimal crate. Chrono0.4.45 defaults off, now.
- PostgreSQL unconstrained NUMERIC with whole/nonnegative/u64-max CHECKs; bind decimal strings through
  text::numeric and decode amount::text through existing Money. No float/narrow signed amount mapping.
- Calendar: NaiveDate with explicit YYYY-MM-DD; today's Vietnam calendar uses UTC+07:00 (no scheduling).
  Month range [first day, next month); financial date is local calendar, audit/expiry are UTC timestamps.
- Store is optional: FUNDGUARD_FINANCE_DATABASE_URL enables finance only in Discord mode. Missing URL
  preserves current readiness-only bot and no database I/O. Offline never reads/validates it.
- Local enabled setup runs embedded versioned migration at bounded connection startup. PgPool max4,
  acquire/query transaction deadline5s, initial DB connect/migrate max15s; pool closes after Discord exit.
- Tests: provider-free domain/UI/service tests; explicit ignored persistence suite against disposable
  PostgreSQL18.6, plus CI synthetic PostgreSQL service. No real credentials or existing .env.local.

## Constitution Check

Pre-research and post-design: PASS, no exception. Domain/application use std/chrono/thiserror (Tokio only for application port deadlines) and
existing feature facades, never SDK/SQLx. Application declares one real persistence port with boxed
Send futures; implementation controls SQL transaction mechanics. Pure posting validation and reporting
remain application/domain rules called within the infrastructure transaction. No generic repository engine.
Existing Financial Core/identity invariants preserved. All mutations auditable and durable-idempotent;
manual confirmation never bank proof. Least-privilege DIRECT_MESSAGES only when finance enabled.

## Project Structure

Meaningful new feature: src/features/ledger/ owns this personal-journal vertical slice, including its
small fund register and monthly ledger query. Future separate fund/report features require real
independent workflow needs, not placeholder modules.

- domain/{model,error,parser,posting,summary}.rs: redacted values, bounded grammar, pure chronology/math.
- application/{mod,port}.rs: owner policy, journal commands/queries and atomic persistence contract.
- infrastructure/{mod,mapping,drafts,commit,queries}.rs: SQLx implementation; no provider errors escape.
- presentation/{mod,messages,components,modals,views}.rs: actual SDK inputs/builders and DM responses.
- platform/database.rs: pool/migration lifecycle; migrations/0001_personal_journal.sql: versioned schema.
- config/discord.rs: optional redacted DB URL. runtime.rs/client.rs: optional service composition.
- identity presentation binding becomes Arc<OwnerBinding> for genuine immutable sharing; no ledger dependency.
- scripts/test-postgres.sh and private infrastructure tests: real transactional/restart verification.

Only create files carrying real behavior; tests may use sibling modules to respect 200–250-line guidance.

## Dependency Direction and Discord Wiring

Ledger application uses identity/Financial Core PUBLIC facades. Financial Core and identity never
import ledger. FinanceEvents implements SDK EventHandler in ledger presentation; platform attaches it
alongside existing Poise. It holds Journal, Arc<OwnerBinding>, existing accepting flag; no context trait,
collectors or second bot. Poise retains only readiness slash handling; finance ignores slash/autocomplete.

Finance enabled adds DIRECT_MESSAGES, not MESSAGE_CONTENT/GUILD_MESSAGES. Every message/component/modal
checks DM context, genuine nonbot invoking user and current existing owner policy. Store row is scoped by
resolved owner and original channel; revision/expiry/source never trusted from custom IDs alone.
Confirm/cancel/modal submit defer privately first; failed acknowledgement prevents mutation. Edit cannot
defer then show a modal: bounded authoritative lookup precedes the initial modal response (<2s).
All other DB/action responses bounded5s and SDK sends2s, empty allowed mentions and suppressed embeds.

## Persistence, Integrity and History

Owner row is the serialization point: initialize owner/default funds, SELECT owner FOR UPDATE, then
read latest draft/history. READ COMMITTED avoids stale pre-lock snapshots. All owner mutations use
same lock, canonical fund names and UNIQUE(owner,origin). Date/id ordering is deterministic.

Store original actor/source/note, transaction date, UTC creation/confirmation, revision/status and
immutable entry. Entries have DB UPDATE/DELETE rejection trigger. Confirm committed source BEFORE
checking expiry or spending capacity (after current authorization), returning original receipt. A stale
uncommitted revision fails; changed posted payload cannot overwrite history. Drafts can be cancelled
or edited only pending/current/unexpired. Confirmation uses fresh time after owner lock acquisition.

Posting plan uses Financial Core Money/BalanceChange/VirtualBalance for every fund movement. Fold
all historical entries in date/id order including the candidate; reject any intermediate underflow or
overflow, including backdated effects. Transfer creates ONE entry with source and destination: same
amount debit/credit in one DB transaction. No mutable balance cache that can diverge from immutable history.
Opening allowed once per fund. Queries derive authoritative balances from validated entries.

Amounts stay full u64. Aggregated report sums use checked u128; integer percent=expense*100/income
with checked arithmetic, unavailable for zero income. Monthly opening = pre-month closing plus initial
opening entries in the month, labeled “Đầu kỳ/khởi tạo”; these are not received income. Thus opening +
received − consumption = total closing. Internal movements have separate saving/investment inflow
lines, never income/expense. Available = closing spending-role funds, including custom spending funds.

## Privacy, Limits and Parsing

Messages/notes <=1000 characters, fund/category <=80, date<=10, money input<=32. Custom IDs contain
only prefix/action/draft integer/revision (<=100). At most five edit inputs: amount, category, route,
date, note. Modal route syntax source or source -> destination. Kind fixed for a draft; cancel/resend
to change it. Unknown financial phrases return help; unsupported chatter performs no storage mutation.

Finite commands: “quỹ”, “tạo quỹ <name>”, “tổng kết [YYYY-MM]”/“báo cáo [YYYY-MM]”, “trợ giúp”.
Income keywords nhận/lương/freelance/thưởng (ASCII aliases too), expense chi/mua/ăn/donate/dnt,
transfers tiết kiệm/đầu tư/chuyển, opening số dư đầu kỳ. Explicit date via “ngày YYYY-MM-DD”.
Amounts integer or k/nghìn/ngàn/tr/triệu; multiple/decimal/negative/out-of-range require correction.
Future-income phrasing rejected; unknown category visibly Khác. Fund route defaults Chi tiêu,
recognized savings/investment target fixed; manual route field supports any owned fund.

New funds are spending-role and zero balance; three default roles fixed. No rename/delete/category
administration, AI inference, market prices, recurring income, committed corrections or bank balance.

## Verification, Delivery and Scope

[Tasks](tasks.md) map FR-001–014, AC-01–14 and SC-001–006 to tests/reviews. Test default/offline without
secrets; parser/whole amounts/future dates; preview revision/expiry; wrong actor/channel; actual SDK
modal/private builders; mathematical example/carryover; DB CHECKs, duplicate race, backdated overdraft,
transaction rollback, restart and immutable history. All four Cargo gates and locked verification.

Disposable server may be extracted non-root outside repository; no existing server/credentials.
Default tests explicitly ignore DB-only suite; run it separately and in CI with synthetic service.
Live Discord/hosted CI remain unverified for this feature unless separately exercised, and no real
financial data is used. Document setup/commands/limits and update ADR/index/handoff/devlog after checks.

## Complexity Tracking

Justified: one persistence port, one owner lock and one immutable journal; concrete SDK event handler.
No bank provider, AI, event bus, general account tree, double-entry accounting framework, multi-bot,
background scheduler, complex generic unit of work or premature standalone funds/reports modules.

## Implementation refinements

2026-11-14: Supabase-managed PostgreSQL replaces the self-hosted application setup. Keep SQLx,
one backend pool and the existing URL setting; use direct5432 or Supavisor session5432 (IPv4).
Reject Supabase transaction6543, which is incompatible with session migration locks/prepared queries.
Parse PgConnectOptions before I/O, upgrade remote opportunistic TLS to Require, reject disabled TLS;
preserve VerifyCa/VerifyFull when configured. Local disposable fixtures may use plain sockets.
Add migration0002 (never edit applied0001): deny Data API roles and enable default-deny RLS on
financial and SQLx metadata tables. Backend connects as migration/table owner; no user-facing SQL
credential or Supabase Auth policy. Disable Data API before first start. See ADR0007 and guide.
No cloud resources or real-data migration is performed; local upgrade/security tests remain required.

Conservative UTF-16 UI bounds; unsupported message kinds/attachments ignored. Mixed income+allocation
and explicit natural “từ … sang …” are not guessed. Posted kinds require cancel/resend to change kind.
Readiness retains the same command/scope and updates owner capability text only when finance enabled.
Views cap categories/funds with disclosed omissions, keeping exact aggregate totals. Clear bank-secret
labels rejected before note storage. Numeric identity accessor is crate-private and used only for audit.
Pool closure participates in the bounded Discord cleanup; failed pre-client startup also closes the pool.
