# Financial Core Public Contract

Implemented library contract for fundguard::features::financial_core.
This surface provides pure validation; REST/Discord protocols, journals and payment APIs remain future work.

## Surface and visibility

The facade exports Money, VirtualBalance, BalanceChange, Direction, OwnerRef, TargetRef,
OriginRef, IdempotencyKey, FinancialIntent, CompletionBasis, ValidatedChange, Decision, CoreError,
ReferenceKind, and validate_intent. Keep domain/application paths and implementation details private.
No service trait, provider, mutable store, serialization framework, or general currency catalog.

Constructors and accessors below are intended API names. Implementation may refine private names;
public semantic changes require contract review. No builder or duplicate request record is needed.

## Money construction

Money::parse(currency: &str, quantity: &str) → Result<Money, CoreError>.
It borrows input and retains only a u64 whole-đồng value; never retain raw text in errors.

Currency is exactly VND. Quantity accepts one or more ASCII digits, including leading zeros.
Supported numeric range is 0–18,446,744,073,709,551,615 VND inclusive. This is a representation
limit, not a business cap or bank balance guarantee. No floating input or localized parsing API.

Apply this deterministic classification order:

1. Currency other than exact VND → UnsupportedCurrency (before examining quantity).
2. Minus followed by one or more ASCII digits → NegativeAmount, including -0.
3. A decimal notation with nonempty ASCII digits on each side of one dot → UnsupportedPrecision,
   including 1.0. Decimal notation is unsupported even if it denotes an integral value.
4. Empty text or any remaining nondigit → InvalidAmountFormat. Reject +1, whitespace, grouping,
   exponent forms, Unicode digits, malformed decimals and signed fractional notation as format errors.
5. Parse unsigned digits with checked standard parsing; beyond MAX → AmountOutOfRange.

| Input (currency VND unless shown) | Outcome |
| --- | --- |
| 0 / 000 / 1250 / 001250 | Exact 0 / 0 / 1250 / 1250 |
| 18446744073709551615 | Exact maximum |
| 18446744073709551616 | AmountOutOfRange |
| -1 / -0 | NegativeAmount |
| 0.5 / 1.0 | UnsupportedPrecision |
| empty / +1 / padded text / 1,000 / 1e3 / 1.2.3 | InvalidAmountFormat |
| USD, 1000 / vnd, 1000 | UnsupportedCurrency |

Money::whole_dong() returns u64 deliberately; no raw monetary Display. Same-type comparison is
exact VND comparison. checked_add returns ArithmeticOutOfRange on overflow. checked_sub returns
NegativeResult on underflow. Never wrap, saturate, truncate, round, or silently narrow values.

## Balance and reference construction

VirtualBalance::new(money) creates an explicitly virtual value. A negative starting balance cannot
be constructed because Money rejects it. BalanceChange::new(direction, money) rejects zero as
ZeroChange. Direction is Increase or Decrease, not a boolean.

VirtualBalance::preview(change) borrows inputs and returns a new VirtualBalance:
Increase uses checked addition; Decrease returns InsufficientFunds if greater than the balance.
Exact depletion to zero is valid. No actual fund is changed and no bank balance can be asserted.

OwnerRef/TargetRef/OriginRef/IdempotencyKey::new(value: String) rejects empty or whitespace-only
strings as InvalidReference with only a ReferenceKind category. Preserve every other byte/case;
never trim/case-fold implicitly. No generation, identity authentication, target lookup or extra
length cap. Read-only as_str accessors expose a deliberate borrowed reference, not a logging policy.

## Intent and callable validation

FinancialIntent uses named fields owner, target, origin, key, change, completion_basis. Types and
ownership are defined in [the model](../data-model.md). There is no timestamp or bank-proof field.
All field-local validation occurs in value constructors; no raw primitive bypass constructor exists.
HandoffOnly and ManualConfirmation are the only CompletionBasis variants.

validate_intent<'prior>(
    intent: &FinancialIntent,
    current: VirtualBalance,
    prior: Option<&'prior ValidatedChange>
) → Result<Decision<'prior>, CoreError>.

This is synchronous pure validation. It reads no external/process state and mutates no input.
A successful new candidate owns its intent snapshot, before and proposed virtual balances.
No side effect, lookup, reservation, journal posting, audit storage, real payment or bank verification.

### Decision order

With Some(prior):

1. Compare prior.intent.owner and key with the candidate. Either mismatch → PriorContextMismatch.
   No error may include the prior's owner, target, key, financial details, or amount.
2. Compare all semantic fields: owner, key, target, origin, direction, exact amount/implicit VND,
   and completion basis. Any difference → IdempotencyConflict.
3. All match → Replay borrowing the same prior candidate. Do not preview current capacity, clone
   the result, create another effect, or reinterpret the prior basis.

With None:

1. HandoffOnly → HandoffNotCompletion. A handoff says nothing about completed payment.
2. ManualConfirmation → preview the virtual balance. InsufficientFunds/ArithmeticOutOfRange fail.
3. Preview succeeds → Eligible with a coherent owned ValidatedChange preserving the entire intent.

Error precedence follows that order after value constructors succeed. In particular a same-key
changed amount/basis returns conflict even if today's balance would be insufficient; a prior key
mismatch is never treated as None. A matching replay can succeed with current balance zero.
Changes to caller observation metadata cannot change this decision because it is not in the intent.

### Prior context coherence

ValidatedChange fields are private; no public constructor, raw result setter, or unchecked restore
API exists. External callers obtain it only from Eligible, then may supply it by reference as prior.
It proves a previous coherent validation result. The caller asserts acceptance; it proves no
durable commit or authenticity of a lookup. Replay borrows the supplied object; it is not a newly
recorded result. Read-only intent/before/proposed accessors support traceability and future adapters.

A later storage feature must define checked reconstruction and atomically enforce real financial
commit. It must not treat this API as a concurrent exactly-once mechanism or a journal receipt.

## Errors, security, and proof limits

The model lists all categorical error variants. Error Display/Debug/source chains retain no raw
payload. Sensitive value/input/result/decision formatting is redacted, including nested and pretty
Debug; no raw Display. The core emits no tracing/logging/console output. Tests use synthetic data.
Owner references express scope, not authorization; callers cannot infer identity authentication.

No BankVerified completion or verified bank-balance constructor exists. ManualConfirmation can
only support an explicitly virtual result, never prove a bank transfer. Awareness-before-handoff
is an obligation of future payment workflows, not a state machine added here. Never collect bank
passwords, OTPs, card PINs, CVVs, or session credentials, or execute real-money transfers.

## Contract verification mapping

| Scenario | Public behavior to verify |
| --- | --- |
| AC-01 | 1000+250 = 1250 exactly; compare and read whole-đồng values |
| AC-02 | Reject fractional and non-VND construction; no conversions/rounding |
| AC-03 | Zero valid; negative/MAX+1 invalid; MAX arithmetic overflow explicit |
| AC-04 | Preview decrease 250 from 1000 → 750; before/input unchanged |
| AC-05 | Decrease 1000 from 1000 → zero; 1001 → InsufficientFunds |
| AC-06 | BalanceChange zero → ZeroChange; MAX increase by 1 → overflow |
| AC-07 | Valid manual intent and None → Eligible without state update |
| AC-08 | Matching prior → borrowed Replay, even with current zero |
| AC-09 | Change each amount/direction/target/origin/basis independently → conflict |
| AC-10 | Different owner or key prior → generic mismatch, no leaked details |
| AC-11 | Each reference role rejects empty and whitespace-only input |
| AC-12 | Fresh handoff-only → HandoffNotCompletion; manual result not bank proof |
| AC-13 | Candidate/replay retains owner/key/target/origin/basis and virtual before/proposed |

Also test equal numeric values from different leading-zero lexemes, no implicit reference
normalization, type/privacy boundaries, redacted nested formats/error sources, small exhaustive
arithmetic domains, unchanged snapshots, and absence of environmental/provider imports.
