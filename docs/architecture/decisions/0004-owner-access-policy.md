# ADR 0004: Pure single-owner access policy

Date: 2026-11-12. Status: Proposed — planning recommendation, not implementation approval.

## Context

Financial Core owner references express scope without authentication. The next increment needs
explicit owner binding and fail-closed scoped decisions, while live Discord/persistence remain deferred.

## Proposed decision

Keep actor/account values in identity domain. Let identity application compose an immutable binding
with the existing Financial Core OwnerRef through its public facade. Resolve/authorize using supplied
current inputs and return a borrowed bound owner or one generic denial. Do not duplicate/move the
owner type, introduce session/grant capabilities or change financial validation.

Document that actor construction is a caller assertion. Future adapters must authenticate provider
identity, load operator-controlled bindings, retrieve authoritative ownership and check access before
every private use case. Pure decisions provide no proof these integration steps occurred.

## Consequences and alternatives

The application edge identity → Financial Core preserves exact owner semantics with no reverse
edge. Identity domain remains independent. Duplicating owner values adds conversion/validation drift;
shared extraction broadens scope. A later reverse dependency needs an explicit ownership review.

Borrowed results avoid cloning and grant-lifecycle machinery. Revocation applies to subsequent
evaluations; in-flight revocation, account transfer and persistence are separate decisions. No provider
trait, auth SDK, database or new dependency is needed. This is a bounded policy, not a production
identity system. Existing owner inputs remain validated and diagnostic output remains redacted.

See [plan](../../../specs/003-owner-access/plan.md),
[research](../../../specs/003-owner-access/research.md) and
[contract](../../../specs/003-owner-access/contracts/owner-access.md).
