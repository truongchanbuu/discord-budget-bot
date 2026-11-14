# ADR 0009: Lossless member incarnation storage

Date: 2026-11-14. Status: Accepted for the scoped incarnation repair.
Authority: maintainer requested repairing the reproduced precision failure.

## Context

Member incarnation is exact joined_at, used to distinguish departure/rejoin. Chrono preserves
nanoseconds; SQLx/PostgreSQL TIMESTAMPTZ preserves microseconds. A synthetic .123456789Z value
returned as .123456Z, causing correct exact-owner validation to deny after reservation.

## Decision

Keep DateTime<Utc> and exact equality in domain/application. Persist owner_joined_at,
target_joined_at and member_joined_at as canonical UTC RFC3339 TEXT with exactly nine fractional
digits, for example 2026-11-14T00:00:00.123456789Z. One infrastructure codec handles every read/write.
It rejects malformed or noncanonical persisted values with a static storage error. Database CHECKs
validate canonical syntax and calendar validity; casting for validation does not replace stored text.

Audit timestamps and operation created/dispatched/completed times remain TIMESTAMPTZ. They are
used for ordering and expiry, rather than exact external identity. Memory adapters keep the same
DateTime values; no normalization or relaxed ownership comparison is introduced.

Adjust unreleased migration0003, which has only been exercised in disposable clusters in this work.
Do not rewrite0001/0002. This is a fresh-schema repair, not recovery of precision previously lost in
an already-applied database; deployed migrations must receive a separate additive upgrade.

## Validation and consequences

Codec tests preserve adjacent nanoseconds, normalize equivalent offsets to UTC and reject invalid
stored values without retaining input. Real private PostgreSQL tests preserve owner, recovery,
operation and viewer incarnations. The prior exact-incarnation and two-owner Ledger regressions
both pass; a one-nanosecond membership mismatch still denies authorization.

No new dependency, public API, financial rule or provider credential is needed. Full006 acceptance
and remaining lint/implementation work are separate; ADR0008 remains proposed.
