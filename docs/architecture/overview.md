# Architecture

FundGuard is a Feature-First Modular Monolith in one Cargo package. Phase 0 implements local
initialization only. See the [authoritative plan](../../specs/001-project-foundation/plan.md)
and [packaging ADR](decisions/0001-feature-first-modular-monolith.md).

## Responsibilities and dependencies

| Area | Responsibility | Allowed dependencies | Excluded dependencies |
| --- | --- | --- | --- |
| Domain | Entities, value objects, invariants, domain errors | Standard library and justified pure libraries | Application, adapters, Discord, SQLx, HTTP, AI |
| Application | Use cases, workflows, ports when needed | Own domain and explicit application APIs | Concrete providers, framework/transport types |
| Infrastructure | Persistence and external adapters | Own application/domain and provider libraries | Presentation and unrelated feature internals |
| Presentation | Map validated transport input/output | Own application APIs and transport framework | Direct financial persistence or cross-feature internals |
| Platform | Client, pool, scheduler lifecycle | Provider libraries and neutral configuration | Financial policy and feature orchestration |
| Composition root | Construct and connect actual dependencies | Components it wires | Business policy |
| Shared | Cohesive domain-neutral concepts used by multiple features | Pure, justified dependencies | Feature internals and framework convenience APIs |

Cross-feature calls use declared application contracts. Review the dependency graph before adding
an edge; cycles and access to another feature's private adapters are forbidden. Private modules
and small exports help, but a single crate cannot mechanically enforce every logical boundary.
No generic repository framework, event bus, DI container, or provider hierarchy is warranted now.

## Current initialization

main owns the Tokio current-thread runtime and process exit. The library exports only run and
StartupError. bootstrap validates configuration and installs tracing; config privately parses the
single supported environment setting. Startup returns immediately; no clients, pools, or jobs run.
A scoped ERROR tracing subscriber reports startup failures independently of global setup.
See the [runtime contract](../../specs/001-project-foundation/contracts/runtime.md).

## Future interface boundary

Discord is the initial UI. Future feature presentation owns commands and views; platform/discord
owns client lifecycle and registers commands supplied by the composition root. Serenity Context,
transport identifiers, and SQLx records stay out of domain/application APIs. Presentation maps
transport identity to explicit application input only after the identity feature defines binding
and authorization. No assumption that a Discord sender is the owner is allowed.

| Future capability | Owning boundary |
| --- | --- |
| Slash commands and direct messages | Feature presentation calls application use cases |
| Pattern-based input | Presentation validates/parses before application calls |
| Buttons, selects, modals | Presentation maps interactions to explicit use-case inputs |
| Images and attachments | Presentation enforces limits; justified infrastructure handles processing |
| Scheduled reminders | Platform owns scheduling lifecycle; reminders application owns policy |
| Report images | Reports application supplies results; presentation/infrastructure renders them |
| Payment awareness | Payments application/domain enforce future invariants before presentation handoff |

These are design accommodations, not implemented capabilities. Use minimal justified gateway
intents. Message content or attachment access requires a feature-specific privilege review;
do not enable privileged intents by default. No bots or HTTP API are initialized in Phase 0.

A second Discord bot, Web endpoint, or mobile adapter can map its inputs to the same use cases
and outputs without copying rules. A new composition root is added only when a real interface
requires it. Platform database manages pool lifecycle; feature infrastructure owns persistence
and application workflows own transaction boundaries. Multi-user isolation, concurrency,
retention, authentication and deployment remain decisions for owning future specifications.
