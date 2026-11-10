# ADR 0001: Feature-first modular monolith in one package

Date: 2026-11-10. Status: Accepted for Phase 0.

## Context

FundGuard needs incremental development, domain integrity, and future Discord/Web/mobile reuse.
There is no business behavior requiring separate releases or crates yet.

## Decision

Use one unpublished Rust package with library and executable. Organize future business work by
feature, with inward Clean Architecture dependencies and private implementation modules. Keep
provider types out of domain/application APIs and cross-feature calls explicit. Create only
meaningful layers. The current four source files implement lifecycle/configuration only.

## Alternatives

A multi-crate workspace would offer stronger compile-time boundaries but adds packaging and
public API maintenance without current need. Global technical layers disperse feature ownership.
Empty feature skeletons supply no behavior and increase navigation cost.

## Consequences and revisit triggers

Logical boundaries require code review as well as privacy; Cargo does not prohibit all sibling
imports in one crate. Revisit crate separation if independent releases, repeated boundary breaches,
compile isolation, or a concrete second executable require it. Another interface must reuse use
cases through explicit input/output types, not copy business rules. No generic framework is added.

See the [architecture guide](../overview.md) and authoritative
[plan](../../../specs/001-project-foundation/plan.md).
