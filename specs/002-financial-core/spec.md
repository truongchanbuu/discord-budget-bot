# Feature Specification: Financial Core and Domain Invariants

**Feature directory**: `specs/002-financial-core`
**Feature Branch**: Existing branch `001-project-foundation`; no branch hook configured.
**Created**: 2026-11-12
**Status**: Implemented and verified locally on 2026-11-12; completion evidence is in [tasks.md](tasks.md).
**Input**: User description: "financial core & domain invariants"

## Objective and Scope

Establish exact monetary values and independently verifiable financial rules before financial
workflows are built. The owner needs future fund, ledger, and payment features to agree on
amounts, virtual balance limits, mutation origins, repeated completion, and evidence meanings.

This feature delivers domain values and invariant validation only. A validation result describes
whether a proposed change is permissible; it does not post an entry, change a stored balance,
execute a payment, or establish bank verification. It remains usable without a user interface,
external account, database, clock, network, or environment configuration.

## Clarifications

### Session 2026-11-12

- Scope: Domain values and invariant validation only; defer ledger operations and persistence.
- Currency: VND only, whole đồng; reject fractional amounts and currency conversion.
- Virtual balance policy: Reject mutations that would make a virtual balance negative.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Represent amounts exactly (Priority: P1)

As the owner, I need future financial features to preserve whole-đồng amounts exactly so a
validation step cannot silently change the value I intended.

**Why this priority**: Every later financial rule depends on unambiguous monetary quantities.
**Independent Test**: Validate monetary values and same-currency calculations without any fund,
ledger, payment flow, or running service.

**Acceptance Scenarios**:

1. **AC-01**: **Given** whole-đồng VND amounts of 1000 and 250, **When** added,
   **Then** the exact result is 1250 VND, with no rounding or precision loss.
2. **AC-02**: **Given** a fractional amount or a currency other than VND, **When** supplied,
   **Then** it is rejected; no rounding, reinterpretation, or currency conversion occurs.
3. **AC-03**: **Given** zero, a negative amount, and values at/outside the supported bounds,
   **When** validated, **Then** zero is valid as a value, negative monetary quantities are invalid,
   and out-of-range values fail explicitly rather than wrap, truncate, or saturate.

### User Story 2 - Protect virtual balances (Priority: P1)

As the owner, I need a proposed change to respect virtual fund limits so invalid intent is
identified before any future feature records it.

**Why this priority**: A valid amount alone does not make a financial change permissible.
**Independent Test**: Supply a virtual balance and a proposed increase/decrease and examine the
validation outcome; no balance repository or actual fund is needed.

**Acceptance Scenarios**:

1. **AC-04**: **Given** a virtual balance of 1000 VND, **When** a decrease of 250 is validated,
   **Then** the proposed result is 750; the supplied balance remains unchanged.
2. **AC-05**: **Given** a virtual balance of 1000 VND, **When** decreases of 1000 and 1001 are
   validated, **Then** the first permits zero and the second fails for insufficient virtual funds.
3. **AC-06**: **Given** a zero-sized change or an increase beyond the supported maximum,
   **When** validated, **Then** it fails with a distinct actionable reason and no partial result.

### User Story 3 - Recognize safe retries and conflicts (Priority: P2)

As the owner, I need future repeated completion signals to describe the same financial intent
without allowing a reused identifier to conceal a different change.

**Why this priority**: Duplicate-safety policy must exist before journal or payment operations.
**Independent Test**: Compare a proposed intent with explicitly supplied prior accepted intent;
no journal, persistent registry, event subscriber, or concurrent worker is introduced.

**Acceptance Scenarios**:

1. **AC-07**: **Given** no prior record for an owner's idempotency key, **When** an otherwise
   valid intent is checked, **Then** it is eligible as new intent, without being recorded.
2. **AC-08**: **Given** a prior accepted intent, **When** its owner/key and financial meaning
   are repeated, **Then** the result is a replay decision referencing the prior result,
   without proposing an additional financial effect, even if the current balance has changed.
3. **AC-09**: **Given** the same owner/key but a different amount, direction, target, origin,
   or completion basis, **When** compared, **Then** it fails as an idempotency conflict.
4. **AC-10**: **Given** prior intent from a different owner, **When** supplied for comparison,
   **Then** it fails as an owner-scope mismatch instead of disclosing or reusing that result.

### User Story 4 - Preserve origin and evidence meaning (Priority: P2)

As the owner, I need virtual records and manual observations to retain their actual meaning
so future presentation cannot mistake an intended payment for a verified bank transfer.

**Why this priority**: Financial correctness includes provenance and honest claims about evidence.
**Independent Test**: Validate intent provenance and distinguish evidence categories without
connecting to Discord, banks, or payment providers.

**Acceptance Scenarios**:

1. **AC-11**: **Given** a missing/blank owner, target, origin, or idempotency reference,
   **When** a financial intent is validated, **Then** it is rejected before a proposed effect.
2. **AC-12**: **Given** handoff-only or manually confirmed evidence, **When** its meaning is
   examined, **Then** neither asserts bank verification; handoff alone is not completion.
3. **AC-13**: **Given** a virtual balance and a valid intent, **When** validation succeeds,
   **Then** the result remains explicitly virtual and traceable to that intent's origin.

### Edge Cases

- Empty/whitespace-only references, zero changes, and negative starting balances are invalid.
- Exact depletion to zero is permitted; any decrease below zero is rejected.
- Arithmetic at supported upper/lower bounds must reject overflow/underflow explicitly.
- New-intent eligibility does not prove absence of prior records; callers supply that context.
- Prior matching-key records from another owner cannot satisfy a retry or expose their contents.
- Prior context with a different key is a context mismatch, not evidence that no prior intent exists.
- A replay returns its prior result; it does not subtract again or re-check current spending capacity.
- A changed non-financial observation timestamp must not make an otherwise identical retry conflict.
- Validation failures leave all supplied values and prior decisions unchanged.
- The feature makes no durability, concurrent-commit, or crash-recovery guarantee.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Provide independently usable domain values and invariant validation only; do not
  post journal entries, maintain mutable fund state, or persist accepted intent/results.
- **FR-002**: Monetary quantities MUST represent VND in exact whole đồng. Reject fractional,
  non-VND, and negative quantities; zero is valid as a value. No floats or implicit rounding.
- **FR-003**: Same-currency comparison, addition, and checked subtraction MUST preserve exact
  value. A finite supported range MUST be explicitly documented before implementation; reject
  out-of-range input and arithmetic results without wraparound, truncation, or saturation.
- **FR-004**: A virtual balance MUST be explicitly distinguishable from a verified bank balance,
  nonnegative, and evaluated without changing the original value.
- **FR-005**: Fresh proposed increases/decreases MUST have strictly positive amounts. Reject decreases
  exceeding the virtual balance and increases outside supported bounds; exact depletion is valid.
- **FR-006**: Financial intent MUST include nonblank owner, logical target, origin, and idempotency
  references, direction, exact VND amount, and relevant completion basis. References remain opaque;
  do not create owner accounts, funds, or external identities to validate them.
- **FR-007**: An idempotency key is scoped to an owner. Compare explicitly supplied prior context:
  absent prior intent yields new eligibility; identical financial meaning yields replay of the
  prior result; a changed meaning under the same key yields conflict. Comparing a prior record
  from another owner or with a different key MUST fail as context mismatch without returning its
  financial details. Replay returns the supplied prior result without evaluating another balance
  effect; insufficient-funds checks apply to new intent, not an already accepted replay.
- **FR-008**: Financial meaning for replay comparison MUST include owner/key, target, direction,
  amount/currency, origin, and completion basis. Observation metadata such as a retry timestamp
  MUST NOT independently create a new effect or conflict. No hidden deduplication registry exists.
- **FR-009**: Validation results MUST distinguish eligible new intent, replay, insufficient funds,
  invalid references/amounts, unsupported currency/precision, range failure, scope mismatch, and
  idempotency conflict. No failed validation may return a partial accepted change.
- **FR-010**: Successful validation MUST preserve traceable origin and explicitly virtual effect.
  Define what a later journal must retain for audit, but create no journal or audit storage now.
- **FR-011**: Handoff, manual confirmation, and bank verification MUST have distinct meanings.
  Handoff alone cannot justify completion; manual confirmation cannot claim bank verification.
  This feature MUST NOT create bank-verified evidence or an actual payment workflow.
- **FR-012**: Record future enforcement obligations: awareness precedes handoff; an actual financial
  commit must atomically preserve effect, origin, idempotency, and audit state. Pure eligibility
  validation MUST NOT be represented as enforcing durable or concurrent exactly-once commits.
- **FR-013**: Validation MUST be deterministic for supplied inputs, independent of transport,
  persistence, system time, environment, or external providers, and require no real credentials.
- **FR-014**: Expose small intention-revealing financial concepts without speculative interfaces,
  generic service frameworks, or empty feature layers. Preserve feature ownership and inward
  dependencies; concrete placement and numeric representation belong to the technical plan.
- **FR-015**: Errors and diagnostics MUST NOT disclose sensitive financial payloads, bank data,
  credentials, or another owner's details. Do not collect banking passwords, OTPs, card PINs,
  CVVs, or banking-session credentials, or execute direct real-money transfers.
- **FR-016**: Verify all scenarios and boundary rules, including exactness, invalid inputs,
  no-overdraft behavior, replay/conflict/scope decisions, provenance, unchanged inputs, and
  evidence distinctions. Keep the existing foundation behavior and verification gates intact.

### Key Entities

- **Monetary value**: Exact whole-đồng VND quantity within documented bounds; no bank claim.
- **Virtual balance**: Nonnegative virtual monetary position, never proof of available bank funds.
- **Financial intent**: Proposed directed amount with owner, target, origin, key, and evidence basis.
- **Origin reference**: Stable reference describing where intent came from without raw sensitive data.
- **Prior accepted context**: Explicitly supplied prior intent/result for replay comparison; not
  a repository or proof that a durable financial commit occurred.
- **Validation decision**: Eligibility/replay or explicit rejection, without an applied mutation.
- **Evidence meaning**: Distinguishes handoff-only and manual confirmation from future bank proof;
  none constitutes a bank verification capability in this feature.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every accepted monetary value and supported calculation preserves exact whole-đồng
  value; all fractional, unsupported-currency, negative, and out-of-range cases are rejected.
- **SC-002**: Every decrease that would make a virtual balance negative is rejected, while exact
  depletion is accepted; no validation changes supplied balances or prior context.
- **SC-003**: Every owner/key replay/conflict scenario returns the specified decision, and, with matching prior context supplied, no
  repeated validation proposes a second effect for an identical accepted intent.
- **SC-004**: All accepted intents preserve required provenance, and zero handoff/manual-confirmed
  cases are represented as bank-verified payments or verified bank balances.
- **SC-005**: All 13 acceptance scenarios have automated verification before feature completion;
  the core requires zero live services or real credentials and the foundation checks still pass.
- **SC-006**: Review finds zero journal operations, persisted financial records, payment execution,
  implicit external dependencies, or unsupported durability/concurrency claims in this feature.

## Assumptions

- The initial product is single-owner, but explicit owner scope avoids conflating future users.
  Owner references express context; authentication and authorization enforcement remain future work.
- Callers provide prior accepted context truthfully. Durable idempotency lookup, concurrent writes,
  transactions, crash recovery, and authoritative audit history require a later persistence feature.
- No currency exchange, fractional allocation, fees, interest, percentage splitting, debt/overdraft,
  reversal/correction journal, or accounting standard is selected by this specification.
- Presentation parsing/localization and identifier serialization/generation are future concerns;
  no locale-dependent reinterpretation or silent normalization determines financial value here.
- Numeric bounds, minimal APIs, and feature ownership will be made concrete in planning. Any
  business restriction beyond whole-VND/no-overdraft requires explicit review, not an arbitrary cap.
- Fund CRUD, income allocation, ledger posting, report calculations, scheduling, Discord commands,
  QR/OCR/AI, bank integration, payment handoff, production financial migrations, and deployment are
  out of scope. Existing Phase 0 implementation and unrelated working-tree edits remain untouched.
