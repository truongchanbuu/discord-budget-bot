# Owner Access Data Model

Implemented in-memory model; no accounts, bindings or grants are persisted. The [specification](spec.md) owns
requirements and [contract](contracts/owner-access.md) owns exact callable/error semantics.

## Values and ownership

| Concept / implemented type | Fields and ownership | Invariants and visibility |
| --- | --- | --- |
| DiscordAccountId | Private NonZeroU64 | Canonical positive ASCII decimal, no leading zeros, bounded by u64::MAX; checked parse only; Copy/Eq; redacted Debug |
| TrustedActor | Private DiscordAccountId | Caller assertion of authenticated external identity; construction does not verify provenance; Copy/Eq; redacted Debug |
| BindingState | Enabled or Disabled | Named state, no unclear boolean parameter; safe Debug/Copy/Eq |
| OwnerBinding | Private account, OwnerRef, BindingState | Application-owned mapping of validated values; constructor owns owner, values remain immutable; redacted Debug |
| OwnerRef | Existing Financial Core value | No new type or duplicate validator; reject blanks, preserve exact remaining bytes; borrowed result |
| Resource-owner scope | Option<&OwnerRef> parameter | Authoritative caller-supplied scope; absence denies; no lookup or resource entity |
| AccessError | InvalidAccountId or Denied | Payload-free thiserror enum, static messages, no source payload; Debug/Eq/Copy |

Invalid owner construction continues returning Financial Core's existing CoreError. Binding accepts
an already validated OwnerRef; it needs no additional error or raw owner constructor.

TrustedActor exposes only construction publicly; application can read its account using a narrow
crate-visible accessor. DiscordAccountId requires no public raw-number or Display accessor.
OwnerBinding::new accepts validated account, owner and state, so it is infallible. It needs no
public account/owner/state getters, setters, serialization, Clone, builder or mutation API for this
increment. Integration tests observe policy results rather than private fields.

## Relationships and boundaries

Identity application owns OwnerBinding because it composes identity domain values with another
feature's public OwnerRef. The only new cross-feature dependency is:
identity application → Financial Core public facade. No private Financial Core module is imported.
Identity domain uses std/thiserror only. Financial Core stays unaware of identity. No shared-kernel
migration, provider trait, repository or financial rule is introduced.

## Decision results

Successful resolution and scoped authorization return a reference to the binding's exact OwnerRef.
The returned lifetime is tied to the binding, not the actor/scope. This avoids cloning identity
strings and prevents treating a second owner representation as the authoritative result.

A result means only that supplied inputs satisfied policy at evaluation. It is neither a credential
nor proof of provider authentication, resource ownership lookup, durable grant or bank verification.
Existing OwnerRef Debug is safe even when nested in Result; deliberately calling as_str is not
permission to log identifiers.

## State and revocation

No internal state transitions occur. To evaluate changed policy, callers supply a new immutable
binding or None. Enabled → Disabled/absent denies the next call even after previous success;
changed owner maps the actor only to that new owner and denies the previous owner scope.
This does not implement account transfer, hot reload, financial migration or in-flight revocation.

No API accepts previous success as an authorization credential. Future private use cases must
re-evaluate current policy immediately before acting; storage/concurrency features must separately
resolve the interval between a policy check and an external action.
