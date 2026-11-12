# Architecture

FundGuard is a Feature-First Modular Monolith in one Cargo package. The executable implements local
initialization only; the library also provides pure Financial Core validation and single-owner access policy. See the [authoritative plan](../../specs/001-project-foundation/plan.md)
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

main owns the Tokio current-thread runtime and process exit. The library retains run and
StartupError and exposes the Financial Core feature facade. bootstrap validates configuration and installs tracing; config privately parses the
single supported environment setting. Startup returns immediately; no clients, pools, or jobs run.
A scoped ERROR tracing subscriber reports startup failures independently of global setup.
See the [runtime contract](../../specs/001-project-foundation/contracts/runtime.md).

## Financial Core (implemented)

The public boundary is fundguard::features::financial_core. It exposes validated whole-đồng VND
values, typed references, immutable virtual balance previews, intent/evidence vocabulary, and
validate_intent. Private domain modules depend on std/thiserror; the private application module
coordinates scope/meaning comparisons before fresh basis and capacity checks. It has no I/O,
clock, environment, provider, registry, journal or mutable state. Other features use this facade,
not the internal module paths. Existing bootstrap behavior does not call the financial core.

Money uses the full u64 range as a representation limit, not a business spending cap. New
candidates own coherent snapshots; matching replay borrows its supplied prior candidate and does
not re-evaluate current spending capacity. Private candidate fields prevent unchecked construction,
but caller-supplied context proves no authentication or durable commit. Manual confirmation stays
virtual; handoff alone is rejected and no bank-proof value can be constructed.

See [ADR 0003](decisions/0003-financial-core-validation.md),
[feature requirements](../../specs/002-financial-core/spec.md), and the
[public contract](../../specs/002-financial-core/contracts/financial-core.md).
Actual financial commits must later atomically preserve origin, effect, idempotency and audit state.

## Identity owner policy (implemented)

features::identity exposes validated Discord account identifiers, caller-asserted actors, immutable
owner bindings and resolve_owner/authorize_scope. Its private domain module uses std/thiserror only.
Identity application owns the binding and imports the existing OwnerRef through Financial Core's
public facade. Financial Core has no reverse dependency; no financial behavior or owner validator
is duplicated or moved into shared. Future reverse edges require an explicit ownership review.

Successful policy returns the binding's borrowed OwnerRef; all policy failures use Denied. The
returned reference is not a permission credential. No actor authentication, binding loading, resource
lookup, cache, session, in-flight revocation or external action occurs. Future adapters must provide
trusted actor/configuration/ownership and re-check current policy before each private use case.
See [ADR 0004](decisions/0004-owner-access-policy.md) and the
[contract](../../specs/003-owner-access/contracts/owner-access.md).

## Future interface boundary

Discord is the initial UI. Future feature presentation owns commands and views; platform/discord
owns client lifecycle and registers commands supplied by the composition root. Serenity Context,
transport identifiers, and SQLx records stay out of domain/application APIs. Presentation maps
transport identity to explicit application input using the identity feature's explicit binding and scoped-authorization policy, with authenticated
provider identity and authoritative resource ownership supplied by future integrations. No assumption that a Discord sender is the owner is allowed.

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
