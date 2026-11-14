# Architecture

FundGuard is a Feature-First Modular Monolith in one Cargo package. The executable defaults to local
initialization and supports explicitly selected Discord readiness. The library retains pure Financial Core
validation and single-owner access policy, independent of the adapter. See the [authoritative plan](../../specs/001-project-foundation/plan.md)
and [packaging ADR](decisions/0001-feature-first-modular-monolith.md).

## Responsibilities and dependencies

| Area | Responsibility | Allowed dependencies | Excluded dependencies |
| --- | --- | --- | --- |
| Domain | Entities, value objects, invariants, domain errors | Standard library and justified pure libraries | Application, adapters, Discord, SQLx, HTTP, AI |
| Application | Use cases, workflows, ports when needed | Own domain and explicit application APIs | Concrete providers, framework/transport types |
| Infrastructure | Persistence and external adapters | Own application/domain and provider libraries | Presentation and unrelated feature internals |
| Presentation | Map validated transport input/output | Own application APIs and transport framework | Direct financial persistence or cross-feature internals |
| Platform | Client, pool, scheduler lifecycle | Provider libraries, supplied command definitions and validated configuration | Financial policy and feature orchestration |
| Composition root | Construct and connect actual dependencies | Components it wires | Business policy |
| Shared | Cohesive domain-neutral concepts used by multiple features | Pure, justified dependencies | Feature internals and framework convenience APIs |

Cross-feature calls use declared application contracts. Review the dependency graph before adding
an edge; cycles and access to another feature's private adapters are forbidden. Private modules
and small exports help, but a single crate cannot mechanically enforce every logical boundary.
No generic repository framework, event bus, DI container, or provider hierarchy is warranted now.

## Current initialization

main owns the current-thread runtime and process exit. Async start selects validated offline/Discord
configuration, then calls existing offline run initialization before any provider access. Synchronous
run/StartupError remain compatible and never connect. ApplicationError adds safe root reporting.
Both primary and fallback diagnostics allow only application targets, suppressing SDK payloads even
at trace/debug. Offline creates no pools or persistent jobs; optional finance creates the bounded database pool. See the
[runtime contract](../../specs/004-discord-owner-entry/contracts/runtime.md).

## Discord readiness adapter (implemented, live smoke unverified)

Identity presentation owns the real slash definition, SDK actor/guild mapping, existing owner-policy
call, fixed private builders and one bounded send. It never imports platform. Platform composes the
framework/client, named guild upsert and a concrete supervisor. It does not implement owner policy.
Credentials stay in validated root configuration/SDK construction, absent from command data.
Financial Core and identity domain/application remain unchanged; no reverse feature dependency.

Disable gateway intents and framework owner discovery/bypass. Named upsert occurs once in consumed
setup, with a separate Ready/Failed signal because setup Err does not terminate Client::start.
The builder itself must disable initialize_owners; it overwrites the corresponding options field.
Startup/registration/reply budgets are 30/10/2 seconds. Sample current runner stage every second;
30 seconds of observed loss fails. Stop acceptance before closing, limit SDK shutdown to eight seconds
and total close/cancel/join to ten. SDK internal retries remain inside logical operation deadlines.
No outer reconnect loop, public fallback, resource lookup, persistent grant or financial use case.

Tests use real builders and controlled operation futures. Poise dispatch internals require SDK context;
unknown-command exclusion is verified from the real table plus published source, not by constructing
a fake full dispatcher. Live provider claims require separate optional smoke evidence. See
[ADR 0005](decisions/0005-trusted-discord-entrypoint.md).

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

## Interface boundaries

Discord is the initial UI. Future feature presentation owns commands and views; platform/discord
owns client lifecycle and registers commands supplied by the composition root. Serenity Context,
transport identifiers, and SQLx records stay out of domain/application APIs. Presentation maps
transport identity to explicit application input using the identity feature's explicit binding and scoped-authorization policy, with authenticated
provider identity and authoritative resource ownership supplied by future integrations. No assumption that a Discord sender is the owner is allowed.

| Capability | Owning boundary |
| --- | --- |
| Slash commands and direct messages | Feature presentation calls application use cases |
| Pattern-based input | Presentation validates/parses before application calls |
| Buttons, selects, modals | Presentation maps interactions to explicit use-case inputs |
| Images and attachments | Presentation enforces limits; justified infrastructure handles processing |
| Scheduled reminders | Platform owns scheduling lifecycle; reminders application owns policy |
| Report images | Reports application supplies results; presentation/infrastructure renders them |
| Payment awareness | Payments application/domain enforce future invariants before presentation handoff |

Readiness and private journal DMs are implemented; other interfaces remain design accommodations.
Use minimal justified gateway
intents. Message content or attachment access requires a feature-specific privilege review;
do not enable privileged intents by default. No bots or HTTP API are initialized in Phase 0.

A second Discord bot, Web endpoint, or mobile adapter can map its inputs to the same use cases
and outputs without copying rules. A new composition root is added only when a real interface
requires it. Platform database manages pool lifecycle; feature infrastructure owns persistence
and application workflows own transaction boundaries. Multi-user isolation, concurrency,
retention, authentication and deployment remain decisions for owning future specifications.

## Conversational personal journal (005)

features/ledger owns this vertical slice's fund register, immutable entry workflow and monthly query.
Its domain uses existing core Money/BalanceChange/VirtualBalance and pure calendar/rule logic. The
application facade rechecks identity policy and declares one real JournalStore port with Send futures.
SQLx lives only in infrastructure/platform; the adapter locks an owner row, loads authoritative state,
invokes pure posting validation, and atomically saves entry/audit/draft state. No mutable balance cache.
Ledger imports existing facades; Financial Core/identity do not import ledger. Arc shares immutable
owner binding with both SDK handlers. A crate-private numeric identity accessor exists for audit only,
not as permission to log identity. FinanceEvents is a concrete separate SDK EventHandler, not a second
bot or generic service framework. It handles only owner DMs and finance component/modal prefixes;
Poise retains guild readiness. DIRECT_MESSAGES is enabled only with optional explicit finance DB URL.

Source/channel/revision/expiry are authoritative. Edit lookup+initial modal is bounded2s; other actions
privately acknowledge before DB work, then edit response. Five-field legacy SDK modals remain supported;
no custom HTTP/AI, privileged message-content scan, public financial replies, attachments or collectors.
The enabled readiness reply points the owner to DMs; disabled finance retains old readiness text.

PostgreSQL constraints preserve whole full-u64 values (no NUMERIC scale rounding); owner serialization
and immutable history guard race/retry/restart. All dated effects are replayed for historical balance
validation. Opening in the month is included in “Đầu kỳ/khởi tạo”, not income; transfers are separate
from consumption. See [ADR0006](decisions/0006-conversational-journal.md),
[contract](../../specs/005-conversational-finance/contracts/finance.md) and task evidence.

## Managed storage

Supabase-managed PostgreSQL is the application setup; SQLx and journal transactions remain unchanged.
Direct/session pooler connections require remote TLS; application/domain have no Supabase dependency.
Data API is disabled before first startup, with migration0002 adding API-role revocation and RLS.
See [ADR0007](decisions/0007-supabase-postgres.md) and the
[setup guide](../../specs/005-conversational-finance/quickstart.md#supabase-setup).
Disposable local PostgreSQL remains an isolated test fixture, not an application hosting requirement.
