# Research: Conversational Personal Finance

Date: 2026-11-13. Decisions resolve all planning unknowns; no real environment file used.

## Persistence

Decision: SQLx0.9.0 (MSRV1.94) and Chrono0.4.45 (MSRV1.62), minimal reviewed features in plan.
Rationale: compatible with pinned Rust1.99.1; PostgreSQL is preferred persistent financial store.
Dynamic SQL avoids database-dependent compilation. Alternatives: memory is not durable; SQLite
changes the approved preferred persistence stack; decimal crates duplicate existing whole-VND values.
Sources: https://crates.io/crates/sqlx/0.9.0 , https://docs.rs/sqlx/0.9.0/sqlx/ ,
https://docs.rs/chrono/0.4.45/chrono/ .

Decision: unconstrained NUMERIC + explicit whole/range CHECK, text casts at binding/reading.
Rationale: NUMERIC(20,0) rounds fractions before CHECK; signed BIGINT narrows the core u64 range.
Source: https://www.postgresql.org/docs/18/datatype-numeric.html .

Decision: owner FOR UPDATE under READ COMMITTED; check committed replay before expiry/capacity;
fresh post-lock clock, unique owner/origin, immutable entries, one transfer row.
Rationale: owner serialization avoids concurrent overspend and opposite-order fund deadlock;
now() is transaction-start time, so use fresh confirmation time after lock acquisition.
Sources: https://www.postgresql.org/docs/18/explicit-locking.html ,
https://www.postgresql.org/docs/18/transaction-iso.html .

A read-only planning probe provisioned PostgreSQL18.6 from distribution .debs extracted into /tmp,
including missing libraries, without root/system install. Private 0700 Unix socket, no TCP, synthetic
user, fresh cluster. Actual version is tested18.6, not a claim of newest production patch.

## Discord

Decision: separate concrete FinanceEvents beside Poise, enabled DIRECT_MESSAGES only with DB config.
Rationale: avoids identity→ledger→identity cycle; components/modals ignored by Poise except existing
slash handling. DM content is available without privileged MESSAGE_CONTENT. No collectors needed.
Source: https://github.com/discord/discord-api-docs/blob/main/developers/events/gateway.mdx .

Decision: Edit opens initial modal after bounded lookup; confirm/cancel/modal submit defer privately
before DB work. Five inputs and no financial/identity content in component IDs. Empty mentions,
suppress embeds, no raw provider errors. Modal message is optional; lookup from validated IDs/channel.
Current SDK supports legacy ActionRow inputs; Discord retains legacy behavior while recommending
new Label inputs not supported by this pinned SDK. Do not invent custom JSON or upgrade framework
without need. Sources: https://github.com/discord/discord-api-docs/blob/main/developers/components/reference.mdx ,
https://github.com/discord/discord-api-docs/blob/main/developers/interactions/receiving-and-responding.mdx .

## Business and interface

Decision: bounded deterministic Vietnamese grammar and editable confirmations, no AI dependency.
Rationale: financial amounts must not be invented; uncertain fields require correction. Keep original
note and explicit received/manual evidence. Finite phrases documented in contracts, not general NLP.

Decision: one ledger feature owns register/history/monthly query, reusing core Money and identity policy.
Rationale: cohesive first personal-journal slice without empty future feature modules. No asset valuations.
Calendar uses date-only accounting and UTC+07 current Vietnam date; audit/expiry UTC. Opening in a new
tracking month is labeled initial/carryover, never salary. Report totals use checked wide integers.
