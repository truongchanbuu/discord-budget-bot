# Financial Core Data Model

Implemented in-memory domain model; no schema, journal, stored balance, identity account, or bank evidence is created.
The [spec](spec.md) owns requirements; the [contract](contracts/financial-core.md) owns input,
result, comparison and error semantics. All public types stay within Financial Core's facade.

## Monetary values

| Type | Fields / meaning | Construction and invariants |
| --- | --- | --- |
| Money | Private u64 whole đồng, implicit VND | 0–18,446,744,073,709,551,615 inclusive; canonical constructor rejects other currencies/fractions/negatives |
| VirtualBalance | Private Money | Nonnegative by construction; explicitly virtual, no bank-verification meaning |
| Direction | Increase or Decrease | Named enum; no ambiguous boolean |
| BalanceChange | Private Direction and Money | Strictly positive amount; zero is invalid |

Money exposes a deliberate whole_dong accessor and exact comparison/checked_add/checked_sub.
Currency is always VND; no general Currency type/catalog or conversion method is needed.
Money/VirtualBalance/BalanceChange can be Copy because their payloads are small scalar values;
this is distinct from cloning owned references. No unchecked arithmetic, floats, rounding,
saturation, or hidden conversion is allowed. Constructor errors retain no rejected text.

Money::parse consumes borrowed canonical text and retains only the exact value. Leading zeros
do not change mathematical value. VirtualBalance::preview(change) returns a new virtual value;
0−1 fails as InsufficientFunds, exact depletion succeeds, overflow fails as ArithmeticOutOfRange.
No setter/mutator, overdraft, bank-balance constructor, or persisted record exists.

## Typed references

| Type | Role |
| --- | --- |
| OwnerRef | Owner scope supplied by the caller, not proof of authentication |
| TargetRef | Logical target supplied by caller, not a loaded or owned fund |
| OriginRef | Stable source/origin identifier, not raw financial data |
| IdempotencyKey | Identifier unique within an owner scope by future caller/storage policy |

Each has a private owned String; reject empty or Unicode-whitespace-only input. Preserve all
other bytes/case, including surrounding whitespace, without implicit trimming/normalization.
Compare exactly. No UUID generation, authorization, repository lookup, credential collection,
or arbitrary new length limit is introduced. External entry points must impose justified limits
in their own specs. Callers supply non-sensitive identifiers, not secrets or financial payloads.

Distinct types prevent accidental role swaps. A common private nonblank check is justified by
the shared invariant, not generic syntax reuse. Raw strings are accessible only through deliberate
borrowed accessors; no raw Display. Debug is redacted. Clone only when ownership must be retained.

## Intent and evidence

FinancialIntent is a named-field input record, with these fields only:

| Field | Type | Semantic comparison |
| --- | --- | --- |
| owner | OwnerRef | Exact owner scope |
| target | TargetRef | Exact logical target |
| origin | OriginRef | Exact source reference |
| key | IdempotencyKey | Exact key within owner scope |
| change | BalanceChange | Direction and exact quantity; VND implicit |
| completion_basis | CompletionBasis | HandoffOnly versus ManualConfirmation |

Its fields may be public because each field's local invariants are encapsulated by its validated
type; it is an input record, not an accepted financial record. Validation borrows it without
mutation. No redundant input DTO/builder or long positional constructor is needed. Later caller
mutation cannot alter an already owned ValidatedChange. Compare the listed semantic fields only;
observation timestamps are not stored in FinancialIntent or read from a clock.

CompletionBasis describes supplied meaning, not authenticated fact. HandoffOnly cannot produce
fresh eligible completion. ManualConfirmation may produce a virtual candidate, never bank proof.
No BankVerified variant/type/provider exists; absence is deliberate. Actual payment lifecycle,
awareness evaluation and handoff sequencing enforcement are outside this feature.

## Validated candidate and decisions

ValidatedChange, owned by application.rs, has private fields:

| Field | Meaning |
| --- | --- |
| intent | Owned snapshot of FinancialIntent; all origin/identity/key/basis fields preserved |
| before | Original VirtualBalance used for that validation |
| proposed | Coherent result of previewing the change against before |

Only a successful validate_intent call constructs this type. Provide read-only accessors, no
public constructor/struct literal/setter, no unchecked deserialization/restoration, and no API
accepting an arbitrary claimed result. The private type fields prevent external prior-context
forgery, while caller truthfulness about prior acceptance remains an explicit trust assumption.
The type proves internal validation coherence only, not an applied or committed financial entry.

Decision has exactly two successful shapes:

- Eligible(ValidatedChange): owned new validation candidate; no stored mutation.
- Replay(&ValidatedChange): borrow the supplied prior result; no new candidate/effect or clone.

Rejection is Result::Err(CoreError), not a partial successful record. There is no mutable state
transition: candidate + supplied inputs → decision. A prior candidate produced by earlier
validation may be reused as context, even when current capacity is lower than its original change.

## Errors and diagnostic safety

CoreError categories cover UnsupportedCurrency, UnsupportedPrecision, InvalidAmountFormat,
NegativeAmount, AmountOutOfRange, ArithmeticOutOfRange, NegativeResult, ZeroChange,
InvalidReference(ReferenceKind), InsufficientFunds, PriorContextMismatch, IdempotencyConflict,
and HandoffNotCompletion. ReferenceKind names only Owner/Target/Origin/Idempotency roles.

Use existing thiserror. Retain no raw input, amount, identifier, balance, intent, or provider error
in error payloads/source chains. Messages are static/categorical and actionable. For sensitive
values/containers use redacted Debug (including pretty/nested forms) or omit Debug; never derive
exposing field values. No ordinary monetary/reference Display and no logging in the pure core.
Deliberate getters are not authorization to log sensitive information.

## Trust and later persistence

None prior means the caller supplies no prior context, not that global absence is proven.
Repeated fresh calls can both be eligible. A future adapter must bind/authorize owners, fetch
truthful prior context, validate restoration, preserve the complete u64 amount range, and atomically
commit effect/origin/idempotency/audit state. This design adds no ledger, persistence contract,
concurrency protection, bank verification, clock, environment dependency, or integration port.
