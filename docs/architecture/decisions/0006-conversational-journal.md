# ADR 0006: Owner-serialized personal journal and private conversational drafts

Date: 2026-11-13. Status: Accepted under maintainer's full-workflow implementation authorization.

Use one ledger feature for the first fund register, immutable journal and monthly query. Reuse pure
Financial Core arithmetic and identity policy; no reverse imports or speculative funds/reports shells.
One PostgreSQL owner row lock serializes commits; unique source plus replay-before-capacity, pending
revision/expiry and whole/range constraints protect durable idempotency. Derive balances from dated
entries and reject any intermediate historical overdraft/overflow. One transfer row conserves money.
Opening is baseline, not income; monthly opening label includes in-period initializations explicitly.

A single persistence port is justified by application/provider separation. SQLx implements atomic
storage mechanics and invokes the pure application/domain posting plan under its lock. No generic
unit-of-work framework, mutable balance cache or database types in application rules.

Concrete FinanceEvents alongside Poise avoids identity-presentation→ledger→identity feature cycles.
Arc shares immutable owner binding with existing readiness. DM-only DIRECT_MESSAGES needs no privileged
message content. Legacy five-input modal builders remain supported by pinned SDK; no custom HTTP.
Owner/channel/revision checks and private acknowledgment precede financial actions. Unknown/ambiguous
language requires correction; no AI or guessed amount. Failed reply never creates a new mutation key.

Optional explicit database configuration preserves offline/readiness-only behavior. Embedded versioned
migration is for this authorized local feature; no production operation. Full financial smoke uses only
synthetic disposable PostgreSQL, never real .env.local. Financial values are virtual/manual, not bank proof.
