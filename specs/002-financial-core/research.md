# Financial Core Research and Decisions

Date: 2026-11-12. Research informed the now-implemented design; [tasks.md](tasks.md) records
verification. Research itself added no dependency or lockfile change.
The maintainer selected domain values/invariant validation, whole-đồng VND, and no overdraft.

## 1. Exact values and numeric bounds

**Decision**: Store Money as a private u64 whole-đồng value, currency fixed to VND. Supported range:
**0–18,446,744,073,709,551,615 VND inclusive**. Changes must be positive; balances may be zero.
Use checked_add/checked_sub and explicit failures; no float, decimal library, wrapping, saturation,
or implicit conversion. This full representation range is a technical limit, not a spending cap.

**Rationale**: All selected quantities are nonnegative integers. The standard library provides
exact checked arithmetic. The maximum requires a value boundary test, not overflow construction
inside that same primitive: use the exact maximum-plus-one string as the rejected case.

**Alternatives**: i64 unnecessarily halves the nonnegative range; arbitrary-precision/decimal adds
complexity with no current requirement. A future persistence adapter must preserve the full range
or reject incompatible schema design; never silently convert to a narrower signed database type.
No database column or migration is selected now.

Sources: [u64 bounds and checked arithmetic](https://doc.rust-lang.org/std/primitive.u64.html),
[checked_add](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add),
[checked_sub](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_sub).

## 2. An explicit rejection boundary

**Decision**: Money::parse(currency, quantity) accepts an exact currency tag and borrowed ASCII
whole-đồng text. This is a small domain constructor, not a Discord/localized-input parser.
The exact grammar/error precedence is in [the contract](contracts/financial-core.md).
Zero and leading zeroes are accepted as exact values; signs, spaces, grouping, exponent notation,
fractions, and other currencies are rejected without retaining the input.

**Rationale**: A u64-only API cannot demonstrate runtime rejection of negative/fractional values
required by AC-02/03. The narrow text constructor covers those cases with no floats or new package.
Use standard integer parsing after grammar checks; do not build a generalized number parser.

**Alternatives**: Float input loses the required boundary; a currency catalog or locale library
is premature. A typed integer-only constructor without a rejection path underspecifies acceptance.
Presentation can later translate its own validated user syntax into this canonical domain input.

## 3. Ownership and the application-facing boundary

**Decision**: Keep one package. Add one cohesive features/financial_core feature with private domain
modules and one application.rs containing the pure validation workflow. Its feature facade exports
required value/input/result types and validate_intent, not the domain/application module paths.
Application coordinates prior-context comparison, evidence eligibility, and domain balance preview.

**Rationale**: This pure workflow has meaningful sequencing, not a trivial forwarding wrapper.
It satisfies the constitution's explicit application API rule for future cross-feature callers
while domain primitives remain independent. No async service, trait, provider, repository, or
additional application layer files are needed. Re-exported values are boundary vocabulary;
other features must call the facade rather than import internal domain modules.

**Alternatives**: Putting values in another feature's ledger internals creates the wrong ownership.
Shared finance folders before multiple consumers disperse feature ownership. A domain-only public
internal tree weakens the application-boundary rule. Multiple crates and facade-only wrappers add
no demonstrated benefit. Proposed [ADR 0003](../../docs/architecture/decisions/0003-financial-core-validation.md)
records the adopted decision; implementation approval came from the maintainer's later
$speckit-implement instruction.

Source: [Rust visibility and re-exports](https://doc.rust-lang.org/reference/visibility-and-privacy.html).

## 4. References and immutable result coherence

**Decision**: Use distinct OwnerRef, TargetRef, OriginRef, and IdempotencyKey types with private
owned strings. Reject blank/whitespace-only references; otherwise preserve bytes/case exactly.
A small private nonblank-validation helper may share this actual rule; no macros/generic ID trait.
No identifiers are generated or authenticated here. Future input adapters own size/rate limits.

FinancialIntent is a named-field input record composed only of validated reference, change, and
basis values. This avoids a long positional constructor and duplicate builder/DTO types.
ValidatedChange has private fields and no public constructor; only successful validate_intent
creates it, retaining a cloned intent and before/after virtual balances. The clone is necessary
to retain provenance independently of caller lifetimes. Read-only accessors expose needed values.

**Rationale**: Typed references prevent swapping roles accidentally. Supplied prior context is a
borrowed ValidatedChange, preventing an external caller from pairing arbitrary intent/result data.
It is still only a prior validation candidate; the caller vouches for acceptance, not a durable
commit proof. Reconstruction from future storage requires its own checked adapter specification.

**Alternatives**: Public result fields permit forged/coherence-breaking prior context; a mutable
journal or idempotency map violates scope. Authentication traits and UUID dependencies solve no
current problem. Mutating a caller's input record after validation cannot alter the owned result.

## 5. Replay before balance capacity

**Decision**: First reject prior owner/key mismatch, then compare all financial meaning. Matching
prior input returns a borrowed replay result, changed meaning returns conflict. Only absent prior
context proceeds to completion-basis and fresh balance checks. Constructors already enforce
amount/reference validity. The same replay succeeds when today's balance no longer funds the
original decrease; a same-key conflict must not become an insufficient-funds error.

**Rationale**: Pure replay decisions must not reapply effects. Borrowing the prior result avoids
copying it or suggesting another candidate. No lookup, reservation, shared state, lock, or commit
is performed. Timestamps/observation metadata are not fields in the semantic intent.

**Alternatives**: Checking current funds first breaks retry behavior. Calling a prior validation
result a posted entry or treating absence as proven global uniqueness invents durability claims.
Future persistence must atomically enforce effect/origin/idempotency/audit state at real commit.

## 6. Evidence and sensitive diagnostics

**Decision**: CompletionBasis has HandoffOnly and ManualConfirmation meanings. Handoff-only input
can be represented but cannot yield fresh eligible completion. Manual confirmation may yield an
explicitly virtual candidate; it never becomes bank proof. No BankVerified variant, verified bank
balance constructor, internal bank provider, or payment state machine is added.

Use payload-free CoreError categories and a ReferenceKind category when naming an invalid role.
Sensitive values/records get redacted Debug or omit Debug; no derived formatting that reveals
raw values. Errors retain no raw parse/provider source or input. No raw Display for these values
and no core logging. Accessors enable deliberate use, not permission to log financial payloads.

**Rationale**: Derived Debug displays fields; nested error/debug formatting can leak even when
normal logs appear safe. Tests inspect both Debug forms, Display, and available error sources
for absence of synthetic markers, not exact complete formatting strings.

Source: [Debug field behavior and format stability](https://doc.rust-lang.org/std/fmt/trait.Debug.html).

## 7. Dependencies, tests, and deferred questions

**Decision**: No dependency or toolchain upgrade. Reuse std and thiserror already declared/resolved
at 2.0.21; domain code does not import Tokio, tracing, Discord, SQLx, HTTP, or AI libraries.
Existing bootstrap uses its existing stack unchanged. Test with deterministic boundary/decision
tables and small exhaustive ranges; no new property-testing framework is needed.

**Rationale**: Existing manifest/lockfile supply the required error derive; no external integration
or serialization is needed. This is a compatibility/design review, not a new vulnerability audit
or compilation result. Previous foundation passes do not verify current local edits or new code.

**Alternatives**: Serde, decimal, UUID, clock, storage, mocking and property-test packages expand
scope without demonstrated need. A generalized port/service layer would be speculative.

All Phase 1 design choices are resolved. Representation limits are published, not silently capped.
Authentication, prior lookup authenticity, concurrent commits, durable audit, persistence range
mapping, retention and external input limits remain explicit future-owner obligations. No ledger,
allocation, scheduling, QR/OCR/AI, payment handoff, bank integration, or deployment is included.
