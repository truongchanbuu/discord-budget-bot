# Feature Specification: Conversational Personal Finance

**Feature Branch**: 005-conversational-finance (feature selector; current Git branch main)
**Created**: 2026-11-13
**Status**: Authorized full workflow: specify → requirement review → plan → tasks → analyze → implement.
The maintainer's explicit request authorizes continuing through existing review gates in this scope.
**Input**: Record Vietnamese conversational income/expenses with notes/categories, virtual funds,
savings/investment allocation and accurate monthly income/spending/remaining summaries.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Reviewing and recording a private transaction (Priority: P1)

As the configured owner, I send a private message such as “tôi vừa donate 100k cho vtuber A”,
review the extracted amount, kind, category, fund, date and original note, then confirm saving.
**Why this priority**: Natural entry is useful only when missing/ambiguous values cannot silently mutate money.
**Independent Test**: Submit synthetic private messages, edit/confirm/cancel, and inspect durable entries.

**Acceptance Scenarios**:
1. **AC-01**: Given owner and 200,000 available, when a 100k donation is confirmed, then one 100,000 VND
   expense, original note, Giải trí category and auditable source are stored; available becomes 100,000.
2. **AC-02**: Given “tôi vừa dnt tiền cho vtuber”, when parsed, then no amount is invented and no money
   changes; the owner can complete missing fields and review again before saving.
3. **AC-03**: Given a preview, when edited, then only its latest revision can be confirmed; Cancel or
   expiry changes no balance. A stale button never commits an older interpretation.
4. **AC-04**: Given duplicate delivery, repeated clicks, concurrency or a restart after successful
   commit, when the same source is confirmed, then at most one entry/effect exists and a retry identifies it.
5. **AC-05**: Given another account, disabled binding or a server/public interaction, when requesting
   finance entry/query/action, then no private data or financial operation is available.

### User Story 2 - Recording income and allocating virtual funds (Priority: P1)

As owner, I track received salary/other income separately from expenses and move available money
into savings/investment funds without reporting internal allocation as spending or extra income.
**Why this priority**: Remaining totals must conserve money and never conflate savings with consumption.
**Independent Test**: Start at zero, confirm opening/income/expense/transfers, and compare every balance.

**Acceptance Scenarios**:
6. **AC-06**: Given a new owner, when funds are viewed, then Chi tiêu, Tiết kiệm and Đầu tư exist at zero;
   a confirmed opening balance can be set once per fund and is not salary/monthly income.
7. **AC-07**: Given “nhận lương 15tr” and “nhận freelance 1tr”, when confirmed, then actual received
   income totals 16 million. A message describing expected/future salary cannot be posted as received.
8. **AC-08**: Given enough Chi tiêu money, when moving 3 million to Tiết kiệm or 2 million to Đầu tư,
   then both sides change atomically, total tracked principal is conserved, and income/expense totals do not change.
9. **AC-09**: Given insufficient source balance, same source/destination, invalid amount or overflow,
   when confirming, then the entire operation fails with no entry, partial transfer or negative balance.
10. **AC-10**: Given the default funds, when owner creates an additional named fund, then its balance
    starts at zero; duplicate/invalid names do not create competing fund identities.

### User Story 3 - Understanding monthly cash flow (Priority: P2)

As owner, I request a monthly summary with received income, actual expenses, categories, allocations,
opening/closing tracked balances and available spending money, including carryover from earlier months.
**Why this priority**: “Remaining” needs explicit meaning rather than misleading salary-minus-expense totals.
**Independent Test**: Use dated entries over month boundaries and verify exact aggregate arithmetic.

**Acceptance Scenarios**:
11. **AC-11**: Given zero opening, 16m received, 6m expenses and 5m moved to reserve funds, when
    requesting that month, then income=16m, expenses=6m, tracked closing=10m and Chi tiêu closing=5m.
12. **AC-12**: Given prior-month funds, when requesting a month, then opening carryover is included,
    only that month's received income/expenses appear in flow totals, and later entries do not leak in.
13. **AC-13**: Given category totals and an income denominator of zero, when displaying percentages,
    then category sums agree with expenses and ratios are exact whole-percent integer displays or unavailable,
    with no division by zero, floating-point amounts or fabricated income.
14. **AC-14**: Given committed entries and pending drafts, when the process restarts, then committed
    balances/history and unexpired previews remain recoverable; financial values are never retained in logs.

## Edge Cases

- All money is positive whole VND for mutations; no fractions, negative/zero changes or conversion.
  Accept unambiguous integers and k/nghìn/ngàn/tr/triệu suffixes; ambiguous/multiple amounts require correction.
- Default entry date is the current date in Asia/Ho_Chi_Minh; explicit YYYY-MM-DD permits historical
  dates. Future received-income/expense dates and expected-income language require correction, not posting.
- Default spending fund is Chi tiêu. Savings/investment phrases mean an explicit internal transfer,
  never a consumption category. Unknown categories use a visible editable Khác suggestion.
- Drafts expire after 15 minutes. UI references contain no money, notes or owner identity. Resource
  owner/source/revision are loaded authoritatively on every action; prior success is not a grant.
- A successful durable commit followed by a failed Discord reply stays committed. Never retry a
  financial mutation using a new source key just because a response timed out.
- Reject operations whose historical source balance would become negative at any later entry date;
  backdating must not make an earlier validated spending sequence inconsistent.
- No silent corrections/deletion of committed history in this increment; entries and audit origins are immutable.
- Long notes/names, malformed dates, unsupported chatter and non-owner messages fail safely with bounded input/output.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Preserve offline and existing guild readiness behavior; finance is opt-in, private-owner DM only.
  No guild/public financial replies; ignore bot-authored messages and unsupported message types.
- **FR-002**: Parse a documented bounded Vietnamese phrase set into received income, expense, internal
  transfer or opening balance; retain original note. Do not claim unrestricted language/AI understanding.
- **FR-003**: Produce an editable preview with amount, kind, category, source/destination funds and date.
  Missing/ambiguous data cannot commit. Require explicit Lưu; support Sửa/Hủy and 15-minute expiry.
- **FR-004**: Reuse exact whole-VND Financial Core arithmetic/invariants and existing configured-owner policy.
  Keep business code independent of provider/persistence frameworks; no reverse cross-feature dependency.
- **FR-005**: Persist drafts, funds, immutable entries, traceable origin/actor/time and idempotency state.
  Commit each entry/balance effect/audit transition atomically; replay never duplicates effects, even after restart.
- **FR-006**: Initialize three virtual funds at zero, permit unique additional funds, and support one confirmed
  opening per fund. Opening is separate from earned/received monthly income; no fund deletion/rename in this slice.
- **FR-007**: Record actual received income with salary/freelance/bonus/other categories and ordinary expenses
  with Giải trí, Ăn uống, Di chuyển, Học tập, Sức khỏe, Mua sắm, Hóa đơn or Khác suggestions/editable labels.
- **FR-008**: Treat savings/investment allocation as two-sided internal transfer, conserve tracked principal,
  prohibit overdraft/overflow and use authoritative source/destination ownership. No market valuation or trades.
- **FR-009**: Scope confirmation, edits, cancellation, fund operations and reports to the authenticated owner
  on every action. Enforce source/revision/expiry and treat foreign/unknown references uniformly.
- **FR-010**: Monthly reports use local-calendar dates, distinguish opening carryover, received income,
  consumption, internal reserve movements, tracked closing and spending closing; provide category totals
  and integer percentages with defined zero-income behavior. All balances are virtual/self-reported.
- **FR-011**: Expose private help, fund listing/creation, monthly summary and natural-message previews with
  clear Vietnamese responses. Financial data must survive restart; discarded/expired drafts cannot affect totals.
- **FR-012**: Validate external sizes/values, minimize privileges, suppress raw provider/database errors,
  and redact all financial/identity data from diagnostics. Never collect banking credentials or execute transfers.
- **FR-013**: Add only verified compatible persistence/date dependencies needed now. Use synthetic isolated
  tests, including real transaction/concurrency/restart coverage against disposable local storage, without
  production credentials/accounts. Existing credential-free fmt/Clippy/test/check and offline behavior remain intact.
- **FR-014**: Document setup, local storage/migrations, phrase limits, dates, category/fund semantics, recovery,
  verification limits and session state. Preserve generated tooling and never read/alter existing real .env.local
  or real financial data for implementation tests. No production deployment or bank integration.

### Key Entities

- **Virtual fund**: Owner-scoped name, spending/saving/investment role and derived exact balance.
- **Draft**: Original source/note, proposed kind/amount/category/funds/date, revision, expiry and lifecycle state.
- **Ledger entry**: Immutable manually confirmed financial fact with durable source/idempotency and audit metadata.
- **Transfer**: One immutable entry with equal source debit/destination credit, never two separately committed operations.
- **Monthly summary**: Calendar-bounded opening/flow/closing, per-category spending and explicit virtual-balance semantics.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All supported complete phrases yield reviewable exact values; every incomplete/ambiguous
  supported phrase performs zero financial mutations before valid explicit confirmation.
- **SC-002**: Repeated/concurrent confirmations of one source produce exactly one financial effect;
  interruption/failure leaves no partial transfer or unmatched audit/idempotency state.
- **SC-003**: Every tested fund/history balance stays within the exact nonnegative range; internal transfers
  conserve total principal and opening adjustments never inflate earned income.
- **SC-004**: Every monthly fixture reconciles opening + income − expenses = total closing, with internal
  transfers excluded from income/spending and correct category totals/zero-denominator handling.
- **SC-005**: Foreign/disabled/public-context attempts expose no private data; synthetic sensitive markers
  are absent from logs/errors; every UI action rechecks current ownership and authoritative draft state.
- **SC-006**: All fourteen acceptance scenarios have test/review evidence, required Rust gates pass,
  durable restart/concurrency is verified locally, and optional live/hosted evidence is explicitly separated.

## Assumptions

- User approved the complete workflow in one request; review gates are satisfied by explicit scope
  authorization plus recorded quality/consistency reviews, not requests to approve the same work again.
- DM is the recommended private interface selected for this first slice. VND only, one configured owner;
  no privileged message-content collection, public message scanning, attachments, OCR or AI provider.
- Three default funds and deterministic category suggestions make the first flow usable without a full
  finance settings UI. Categories may be corrected on drafts; global category administration is deferred.
- Default timezone is Asia/Ho_Chi_Minh; backdating requires the historical nonnegative rule above.
  No recurring salaries, forecasting, debt, portfolio valuation, export, charts or committed-entry editing.
- A local disposable database may be used for integration verification; no production/existing credential
  file is consumed. No claim that manually reported payments or virtual balances are bank-verified.
