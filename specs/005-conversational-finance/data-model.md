# Data Model

All values redact Debug; errors retain categories, never values/provider sources. Money uses existing
whole-VND core; nonzero changes and chronological balance validation are mandatory.

| Value | Fields / constraints |
| --- | --- |
| Owner | Existing OwnerRef, exact nonblank bytes; one serialized owner row; no bank identity |
| Fund | Canonical trimmed lowercase name1–80 chars, role spending/saving/investment; unique owner/name; default Chi tiêu/Tiết kiệm/Đầu tư; immutable name; new custom role spending |
| Proposal | Kind income/expense/transfer/opening, amount Option<Money> (required >0 to commit), category1–80, fund1–80, optional distinct destination, local date not future, note<=1000; no unsupported currency |
| Draft | Positive database id, owner, authenticated actor audit ref, origin1–128 unique per owner, DM channel ref, proposal, revision>=0, UTC expires15min, status pending/posted/cancelled, optional entry id |
| Entry | Immutable positive id and unique owner/origin, complete proposal, actor/source/created/confirmed timestamps; transfer one source+destination; no unchecked restoration |
| Month | Strict YYYY-MM, valid supported Gregorian month; [first,next-first) date-only interval |
| Summary | checked u128 received/consumption/initial+carryover and categories, core-u64 fund closing; spending-role available, reserve inflows, integer percent or unavailable for zero denominator |

Lifecycle: incomplete pending → edited current pending → confirmed posted OR cancelled. Pending
expires at15min; committed replay remains identifiable after expiry/restart. Stale pending revision
fails. Every state transition rechecks authenticated owner and original DM channel. No committed edits.

Opening only once per fund. Fold history ordered (date,id); candidate cannot create any historic
negative/overflow balance. Income credits, expense debits, transfer equal debit/credit, opening credits
without earned-income classification. No mutable persisted balance cache or concurrent duplicate effect.
