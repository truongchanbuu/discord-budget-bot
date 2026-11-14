# Feature Index

| Feature | Authoritative specification | State |
| --- | --- | --- |
| 001 Project Foundation (Phase 0) | [Specification](../../specs/001-project-foundation/spec.md) | [Plan](../../specs/001-project-foundation/plan.md), [tasks](../../specs/001-project-foundation/tasks.md): Phase 0 complete; all 36 tasks verified locally; hosted CI unverified |
| 002 Financial Core and Domain Invariants | [Specification](../../specs/002-financial-core/spec.md) | [Plan](../../specs/002-financial-core/plan.md), [tasks](../../specs/002-financial-core/tasks.md): pure core complete; all 26 tasks verified locally; 27 total tests pass |
| 003 Owner Identity and Access Boundaries | [Specification](../../specs/003-owner-access/spec.md) | [Plan](../../specs/003-owner-access/plan.md), [tasks](../../specs/003-owner-access/tasks.md): pure owner policy complete; all 23 tasks verified locally; 37 total tests pass |
| 004 Trusted Discord Owner Entrypoint | [Specification](../../specs/004-discord-owner-entry/spec.md) | [Plan](../../specs/004-discord-owner-entry/plan.md), [tasks](../../specs/004-discord-owner-entry/tasks.md): all 38 tasks locally verified; opt-in readiness implemented; 69 total tests pass; live auth/registration/shutdown passed; private owner reply confirmed; other-account/disabled denial, recovery and hosted CI unverified |

| 005 Conversational Personal Finance | [Specification](../../specs/005-conversational-finance/spec.md) | [Plan](../../specs/005-conversational-finance/plan.md), [tasks](../../specs/005-conversational-finance/tasks.md): all 37 tasks verified including Supabase amendment; 90 ordinary +10 explicit PostgreSQL tests pass; Supabase/finance live/hosted CI unverified |

| 006 Private Finance Channels | [Specification](../../specs/006-private-finance-channels/spec.md) | [Plan](../../specs/006-private-finance-channels/plan.md), [65 tasks](../../specs/006-private-finance-channels/tasks.md): complete locally;148 ordinary+27 isolated PostgreSQL tests pass; category/startup/owner onboarding verified live; viewer/outsider personas and hosted CI unverified |

Active feature:006 Private Finance Channels, completed locally. No pending implementation tasks.
Delivered automatic category setup with independent optional location overrides, private server
channels, isolated per-member journals and read-only
invited viewers; existing configured-owner DM journal stays compatible. Live category recovery,
Gateway startup and one ready owner space are verified; no deployment or push performed. Next:
operator-authorized synthetic viewer/outsider validation using
[quickstart](../../specs/006-private-finance-channels/quickstart.md), then a separately specified feature.
Specifications own requirements, tasks own detailed progress and [handoff](SESSION_HANDOFF.md) owns
current state. Use these links if the local selector is absent. Constitution1.1.0 applies;
generated tooling, dependency lockfile and migrations0001/0002 are unchanged.

The live403/50013 caused by a redundant category MANAGE_ROLES overwrite was corrected without
Administrator. Corrected category creation, durable binding and owner onboarding now pass live;
viewer grant/revoke, outsider visibility and hosted CI remain unverified. See handoff for evidence.
