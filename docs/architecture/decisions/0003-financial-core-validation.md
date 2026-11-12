# ADR 0003: Exact VND values and pure financial validation

Date: 2026-11-12. Status: Accepted — approved for implementation on 2026-11-12.

## Context

The maintainer selected domain values/invariants only, whole-đồng VND, and no virtual overdraft.
Core must preserve exactness and replay semantics without implying a bank or journal integration.

## Decision

Use private u64 monetary values (0–18,446,744,073,709,551,615 VND inclusive), canonical text
construction, checked arithmetic, typed references, and virtual-only previews. Keep Financial Core
feature-owned; its private application function coordinates pure domain validation and exposes an
explicit feature facade. No service/trait/provider layer or new dependency is needed.

Coherent prior context is obtainable only from previous successful validation. Compare owner/key
and financial meaning before fresh capacity checks; replay borrows its prior candidate. Store
no registry/journal and make no durable/concurrent exactly-once claim. Keep errors and diagnostic
formats free of sensitive payloads. Manual confirmation cannot become bank proof.

## Consequences and alternatives

A narrow canonical constructor enables rejection tests without a general locale/decimal library.
Using the full unsigned range avoids an arbitrary cap, but future persistence must preserve it
rather than blindly narrowing to a signed column. Typed references prevent role confusion;
validated private candidate fields prevent unchecked result pairing. The application function
is justified by decision ordering, not by a wrapper pattern or future interface speculation.

A database, ledger, decimal/UUID framework, shared finance directory, or extra crate would exceed
current need. All-in-domain public internals weaken the constitution's application-boundary rule.
Actual commit/authentication/audit enforcement remains future work. The maintainer authorized the analyzed task list. This acceptance is not a claim of compilation;
verified results belong in the feature task checklist.

See [research and alternatives](../../../specs/002-financial-core/research.md),
[model](../../../specs/002-financial-core/data-model.md), and
[public contract](../../../specs/002-financial-core/contracts/financial-core.md).
