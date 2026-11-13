# Feature Index

| Feature | Authoritative specification | State |
| --- | --- | --- |
| 001 Project Foundation (Phase 0) | [Specification](../../specs/001-project-foundation/spec.md) | [Plan](../../specs/001-project-foundation/plan.md), [tasks](../../specs/001-project-foundation/tasks.md): Phase 0 complete; all 36 tasks verified locally; hosted CI unverified |
| 002 Financial Core and Domain Invariants | [Specification](../../specs/002-financial-core/spec.md) | [Plan](../../specs/002-financial-core/plan.md), [tasks](../../specs/002-financial-core/tasks.md): pure core complete; all 26 tasks verified locally; 27 total tests pass |
| 003 Owner Identity and Access Boundaries | [Specification](../../specs/003-owner-access/spec.md) | [Plan](../../specs/003-owner-access/plan.md), [tasks](../../specs/003-owner-access/tasks.md): pure owner policy complete; all 23 tasks verified locally; 37 total tests pass |
| 004 Trusted Discord Owner Entrypoint | [Specification](../../specs/004-discord-owner-entry/spec.md) | [Plan](../../specs/004-discord-owner-entry/plan.md), [tasks](../../specs/004-discord-owner-entry/tasks.md): all 38 tasks locally verified; opt-in readiness implemented; 69 total tests pass; live smoke/hosted CI unverified |

Active feature: 004 Trusted Discord Owner Entrypoint, implementation complete and locally verified.
Initialization/tooling, pure Financial Core and owner-access policy are implemented. Real provider authentication/delivery remain unverified. Resource integration, funds, ledger,
persistence and Discord financial workflows remain future features. Specifications own requirements; tasks own detailed
completion evidence. The [handoff](SESSION_HANDOFF.md) owns current state, limits and next action.
Use these tracked links when .specify/feature.json is absent. Constitution 1.1.0 applies.
