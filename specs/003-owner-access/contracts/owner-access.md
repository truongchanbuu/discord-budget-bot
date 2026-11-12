# Owner Access Public Contract

Planned library surface: fundguard::features::identity. No code is implemented by this contract.

## Exports and type construction

The facade exports DiscordAccountId, TrustedActor, BindingState, OwnerBinding, AccessError,
resolve_owner and authorize_scope. Domain/application paths remain private. Reuse OwnerRef from
fundguard::features::financial_core; do not re-export/duplicate it under identity.

- DiscordAccountId::parse(input: &str) → Result<DiscordAccountId, AccessError>.
- TrustedActor::new(account: DiscordAccountId) → TrustedActor.
- OwnerBinding::new(account: DiscordAccountId, owner: OwnerRef, state: BindingState) → OwnerBinding.
- BindingState has exactly Enabled and Disabled.
- AccessError has exactly InvalidAccountId and Denied, with static payload-free messages.

Binding fields remain private. No public raw account constructor, number accessor, Display,
unchecked restoration, grant constructor, setters, grant/session object or provider verification API.
The account constructor validates supplied text; TrustedActor construction merely asserts caller
trust and must never be described as authenticating Discord users.

## Identifier validation

Input must contain 1–20 ASCII digits, first digit 1–9; checked u64 parsing must succeed. All rejected
inputs yield InvalidAccountId without retaining text or exposing parse errors as sources.

| Account input | Expected result |
| --- | --- |
| 1, 42, 18446744073709551615 | Accepted exact identifier |
| empty, ASCII or Unicode whitespace | InvalidAccountId |
| 0, 00, 01, +1, -1 | InvalidAccountId |
| padded digits, Unicode digits, decimal, exponent, comma, mention, name | InvalidAccountId |
| 18446744073709551616 or any input longer than 20 bytes | InvalidAccountId |

This is the application grammar from FR-002, not proof an account exists. OwnerRef construction
uses the unchanged Financial Core contract: blank owner fails; nonblank bytes/case are preserved.

## Owner resolution

resolve_owner<'binding>(
    actor: Option<&TrustedActor>,
    binding: Option<&'binding OwnerBinding>
) → Result<&'binding OwnerRef, AccessError>.

Deny with the same Denied value when actor is absent, binding is absent/disabled, or account does
not match. Otherwise return a reference to the binding's existing owner. Do not read global state,
clone owner data, infer ownership, or register an actor. Inputs remain unchanged.

## Scoped access

authorize_scope<'binding>(
    actor: Option<&TrustedActor>,
    binding: Option<&'binding OwnerBinding>,
    resource_owner: Option<&OwnerRef>
) → Result<&'binding OwnerRef, AccessError>.

First resolve the owner through the current binding. Then require resource_owner to be present and
equal to that owner using existing exact OwnerRef equality. Return the resolved bound owner,
never the supplied scope reference. Every rejection is the same Denied value, regardless of reason.
No missing-scope fallback or caller-selected owner overrides resolution. No detail is revealed about
whether a binding, account or foreign owner exists. This does not promise constant-time execution.

The API accepts no prior success, permission mode, interaction kind, channel/server role, raw
message, lookup callback or financial intent. Repeated evaluation uses only current inputs.

## Trust obligations for future callers

1. Obtain actor identity from an authenticated provider event, never message content, names,
   mentions, submitted owner fields, or arbitrary caller-controlled account identifiers.
2. Obtain the binding from trusted operator configuration, never from the requesting actor.
3. For resource access, fetch ownership from authoritative data independently of client scope.
4. Resolve/authorize for every private use case, including retries/replays. Check current binding
   before acting; never accept a prior returned OwnerRef as proof of permission.
5. Map Denied uniformly to safe user-visible failure. Never echo configured or foreign identifiers.
6. Preserve these rules when adding real commands, messages, components or additional interfaces.
   Future non-Discord identity sources need their own specification, not fabricated Discord IDs.

Pure policy cannot verify any of these integration steps. The public constructors intentionally
support synthetic offline tests, not a cryptographic trust capability. No private use case or live
adapter is wired by this increment. In-flight revocation and transactional enforcement are future
integration decisions.

## Privacy and error behavior

Sensitive values/actor/binding Debug and pretty Debug must redact raw identity data. Results inherit
OwnerRef's redacted Debug. BindingState and AccessError may print safe state/category names only.
Errors contain no raw source, identifiers or provider payloads; error source is None. No logging,
environment access, network, storage, clock, financial action or hidden remembered grant is allowed.

## Acceptance coverage

| Scenario | Planned verification |
| --- | --- |
| AC-01 | Matching enabled binding returns the same bound owner on repeated calls |
| AC-02 | One-digit account mismatch returns Denied with no owner payload |
| AC-03 | None binding always denies, no registration or fallback |
| AC-04 | Full lexical/range matrix; blank owners rejected by existing OwnerRef; missing actor denies |
| AC-05 | Matching actor/binding/resource scope succeeds, original values unchanged |
| AC-06 | Different/missing owner scope denies, including case/padding differences |
| AC-07 | Unknown actor plus correct owner scope still denies |
| AC-08 | Success followed by disabled/absent binding denies; changed owner uses current mapping |
| AC-09 | Same synthetic input used for message/command/component-labelled test cases yields same result; no transport parameter |
| AC-10 | Review all caller obligations above; no live-adapter claim or content-as-identity path |
| AC-11 | Debug, pretty/nested result formatting and every error/source contain no sensitive markers |
| AC-12 | Public tests plus full source/import/API review establish pure policy with stated proof limits |

Use external compiler probes during implementation to reject raw account construction, private
module imports and binding field modification. Do not create an unused compile-test framework.
