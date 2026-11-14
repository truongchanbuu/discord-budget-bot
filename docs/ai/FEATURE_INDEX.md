# Feature Index

| Feature | Authoritative specification | State |
| --- | --- | --- |
| 001 Project Foundation (Phase 0) | [Specification](../../specs/001-project-foundation/spec.md) | [Plan](../../specs/001-project-foundation/plan.md), [tasks](../../specs/001-project-foundation/tasks.md): Phase 0 complete; all 36 tasks verified locally; hosted CI unverified |
| 002 Financial Core and Domain Invariants | [Specification](../../specs/002-financial-core/spec.md) | [Plan](../../specs/002-financial-core/plan.md), [tasks](../../specs/002-financial-core/tasks.md): pure core complete; all 26 tasks verified locally; 27 total tests pass |
| 003 Owner Identity and Access Boundaries | [Specification](../../specs/003-owner-access/spec.md) | [Plan](../../specs/003-owner-access/plan.md), [tasks](../../specs/003-owner-access/tasks.md): pure owner policy complete; all 23 tasks verified locally; 37 total tests pass |
| 004 Trusted Discord Owner Entrypoint | [Specification](../../specs/004-discord-owner-entry/spec.md) | [Plan](../../specs/004-discord-owner-entry/plan.md), [tasks](../../specs/004-discord-owner-entry/tasks.md): all 38 tasks locally verified; opt-in readiness implemented; 69 total tests pass; live auth/registration/shutdown passed; private owner reply confirmed; other-account/disabled denial, recovery and hosted CI unverified |

| 005 Conversational Personal Finance | [Specification](../../specs/005-conversational-finance/spec.md) | [Plan](../../specs/005-conversational-finance/plan.md), [tasks](../../specs/005-conversational-finance/tasks.md): all 37 tasks verified including Supabase amendment; 90 ordinary +10 explicit PostgreSQL tests pass; Supabase/finance live/hosted CI unverified |

| 006 Private Finance Channels | [Specification](../../specs/006-private-finance-channels/spec.md) | [Plan](../../specs/006-private-finance-channels/plan.md), research/model/contracts/guide ready for plan review; quality16/16; no tasks/code yet |

Active feature: 006 Private Finance Channels, technical plan review pending.
Feature005 remains delivered: manual private journal and optional managed PostgreSQL.006 proposes
self-service private server channels, isolated per-member journals and read-only invited viewers;
none of006 is implemented yet. Specifications own requirements, tasks own implementation progress,
and [handoff](SESSION_HANDOFF.md) owns current state. Use these links if the local selector is absent.
Constitution1.1.0 applies; no generated tooling or runtime configuration changed by006 planning.
