# Feature Specification: Phase 0 Project Foundation

**Feature Branch**: `001-project-foundation`

**Created**: 2026-11-10

**Updated**: 2026-11-10

**Status**: Phase 0 complete and locally verified; hosted CI execution unverified

**Input**: User description: "Establish a production-minded, secure, maintainable foundation
for FundGuard, a Discord-first personal financial management and payment-awareness application."

## Objective and Scope

Phase 0 establishes architecture and developer experience for incremental feature delivery.
It must provide a safe executable, clear boundaries, practical documentation, repeatable quality
checks, and persistent development context. The foundation must accommodate the documented future
interfaces through reusable business modules without requiring their initial implementation.
Production-minded means explicit security and maintenance practices; it does not mean production
readiness, production integrations, or deployment in this phase.

The product begins as a Feature-First Modular Monolith guided by Clean Architecture principles.
One Cargo package is preferred initially. Concrete directory layouts, module declarations,
dependency versions, and wiring choices belong in the technical plan.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Initialize and verify safely (Priority: P1)

As a contributor, I can install the documented toolchain, build, test, and run the foundation
without service accounts so I can start development without exposing secrets or contacting real
integrations.

**Why this priority**: A safe executable and repeatable checks are the minimum useful foundation.

**Independent Test**: Follow the README from a fresh checkout, execute all four quality gates,
and run the application without credentials or service processes.

**Acceptance Scenarios**:

- **AC-01 — Repository Initialization**: **Given** an initialized Spec Kit repository,
  **When** Phase 0 is implemented, **Then** it contains a valid Rust project that builds successfully.
- **AC-04 — Testing**: **Given** a fresh checkout with the documented Rust toolchain,
  **When** the developer runs the four commands in Verification Requirements,
  **Then** all pass without production credentials, Discord tokens, or a running database.
- **AC-05 — Documentation**: **Given** the repository README, **When** a developer follows setup
  instructions, **Then** they can install dependencies, run the foundation, and execute verification.
- **AC-10 — Safe Bootstrap**: **Given** no credentials or service processes, **When** the executable
  runs with default configuration, **Then** it records successful initialization, makes no external
  service calls, and exits successfully without waiting for input.
- **AC-11 — Invalid Configuration**: **Given** an invalid supported configuration value,
  **When** the executable starts, **Then** it exits unsuccessfully with an actionable error that
  identifies the setting without echoing its value.
- **AC-12 — Ambient Credentials**: **Given** unrelated Discord or database credential variables,
  **When** the executable runs, **Then** it neither uses them to activate integrations nor includes
  their values in diagnostics.

### User Story 2 - Extend features within explicit boundaries (Priority: P2)

As a contributor, I can identify the intended home and dependency rules for future business
behavior and interface adapters without navigating speculative abstractions or duplicating rules.

**Why this priority**: Feature ownership and framework independence protect future domain integrity.

**Independent Test**: Use the architecture and coding guides to locate a hypothetical domain rule,
application use case, persistence adapter, and Discord interaction, then explain how another
interface would reuse the same business behavior.

**Acceptance Scenarios**:

- **AC-02 — Architecture**: **Given** the project foundation, **When** a developer inspects the
  module structure and its architecture guide, **Then** feature boundaries and dependency rules are
  clearly documented, with intended future structure distinguished from implemented modules.
- **AC-03 — Clean Code Standards**: **Given** a new developer or Codex session, **When** they read
  coding standards, **Then** practical good/bad examples identify naming, function design, error
  handling, visibility, ownership, and testing conventions without arbitrary function-length rules.
- **AC-08 — Minimal Foundation**: **Given** successful Phase 0 completion, **When** the repository
  is inspected, **Then** it contains no premature financial business features, empty module
  directories, placeholder source files, or speculative service abstractions.
- **AC-09 — Extensibility**: **Given** the architecture documentation, **When** a future Discord
  bot or Web interface is proposed, **Then** the design explains how it reuses application and domain
  logic without duplicating business rules or requiring those interfaces to be built in Phase 0.

### User Story 3 - Resume development and review security consistently (Priority: P3)

As a maintainer or future Codex session, I can find authoritative requirements, task state,
standards, security constraints, and verification results without relying on chat history.

**Why this priority**: Persistent context and explicit safeguards prevent contradictory changes and
unsupported completion claims before sensitive financial data is introduced.

**Independent Test**: Follow AGENTS.md and referenced documents without chat history to identify
scope, architecture, progress, next action, and security boundaries; inspect CI configuration and
the repository for real secrets and personal financial records.

**Acceptance Scenarios**:

- **AC-06 — Session Continuity**: **Given** a new Codex session without previous chat history,
  **When** it reads AGENTS.md and referenced project documents, **Then** it can identify architecture,
  agreed scope, current progress, active specification and tasks, and the next action.
- **AC-07 — Security**: **Given** a clean checkout, **When** the repository is inspected,
  **Then** no real banking credentials, Discord tokens, personal financial records, or other secrets
  are present; environment examples contain only safe illustrative values.
- **AC-13 — CI Independence**: **Given** a pull request, **When** CI executes, **Then** formatting,
  lint, tests, and build validation run using committed dependency resolution without access to
  banking, Discord, database, or other external service accounts.
- **AC-14 — Security Review**: **Given** the security baseline and constitution, **When** reviewed,
  **Then** they define the required security principles and prohibited banking data, distinguish
  payment handoff from confirmation, and prohibit direct real-money transfers in the MVP.
- **AC-15 — Honest Product State**: **Given** the product roadmap and session handoff, **When**
  reviewed, **Then** planned capabilities are distinguished from implemented functionality and
  Financial Core domain invariants precede Discord payment features.

### Edge Cases

- Missing optional configuration uses a documented safe default. Empty, malformed, or non-Unicode
  values are either explicitly supported or rejected without disclosing their contents.
- Credentials present in the environment must not enable an integration implicitly.
- Noninteractive execution must not wait for user input, Discord, or database availability.
- Initial toolchain installation and dependency downloads may require network access; runtime
  initialization and tests must not call external services.
- CI configuration can be reviewed locally; hosted CI success requires evidence of a hosted run.
- Personal-use assumptions do not justify implicitly trusting every Discord user or storing
  prohibited banking credentials; identity and authorization decisions remain explicit boundaries.
- Future message patterns, image inputs, and reminders may require additional privileges or
  services. Their specifications must justify those needs; Phase 0 enables no such access.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001 — Safe Initialization**: The repository MUST provide a minimal executable that validates
  its supported configuration, records successful initialization at INFO when permitted by the
  configured log level (including the default), and exits normally without
  credentials, service processes, external calls, or business side effects.
- **FR-002 — Configuration**: Supported settings MUST have documented defaults or explicit
  validation requirements. Invalid settings MUST fail with actionable, redacted errors. Environment
  examples MUST contain no real secrets; ambient service credentials MUST NOT activate integrations.
- **FR-003 — Architecture**: The foundation MUST document feature-first modular-monolith ownership
  and Clean Architecture dependency rules. Domain contains pure entities, value objects, invariants,
  and domain errors without Discord, SQLx, external API, or infrastructure dependencies. Application
  coordinates use cases and owns ports where required without knowing provider implementations.
  Infrastructure implements adapters; presentation translates interactions into application use
  cases. Platform owns shared integration lifecycle; the composition root initializes and connects
  dependencies. Cross-feature calls MUST use explicit application boundaries, never another
  feature's internal adapters, and MUST NOT form circular dependencies.
- **FR-004 — Interface Evolution**: The architecture MUST describe a replaceable Discord adapter
  boundary and reusable application/domain logic for future multiple Discord bots, Web, and mobile
  interfaces. It MUST accommodate the Future Capabilities below as design requirements only.
  Discord access MUST use least privilege and minimal justified gateway intents. Phase 0 MUST NOT
  create multiple deployable bots or require a separate HTTP API; any proposed exception requires
  explicit justification and scope approval before implementation.
- **FR-005 — Coding Standards**: A practical standards document MUST define idiomatic, type-safe
  Rust conventions with acceptable and unacceptable examples. It MUST cover meaningful names;
  cohesive function, module, and feature responsibilities; clear control flow; appropriate command
  and query separation; explicit Result/error handling; private-by-default APIs; ownership and
  borrowing; minimal clones and mutable state; explicit side effects; testable domain logic;
  and comments explaining why. It MUST also cover async boundaries, dependency injection, justified
  traits/generics, domain value objects, database transactions, structured logging, and test
  organization. It MUST reject arbitrary function-length limits, excessive tiny functions,
  unnecessary patterns, speculative abstractions, unclear boolean parameters, deep avoidable
  nesting, and unjustified unwrap/expect in runtime-critical code.
- **FR-006 — Security Governance**: The constitution and security baseline MUST define security
  by design, least privilege, safe environment configuration, no hardcoded or committed real
  secrets, sensitive-data-free structured diagnostics, validation at external boundaries, explicit
  user identity and authorization boundaries, secure Discord token handling, and auditability of
  future financial operations. They MUST prohibit collecting or storing banking passwords, OTPs,
  card PINs, CVVs, or banking session credentials, and prohibit directly executing real-money
  transfers in the MVP. They MUST preserve the Financial Integrity Constraints below.
- **FR-007 — Developer Tooling**: Phase 0 MUST provide a Cargo manifest and committed lockfile,
  documented Rust toolchain configuration, rustfmt and Clippy conventions, meaningful unit tests
  and integration test organization, .gitignore, .env.example, basic developer commands, and
  GitHub Actions CI running the required checks with minimal permissions and no external accounts.
  Additional tooling MUST have a demonstrated maintenance or verification benefit.
- **FR-008 — Documentation**: All paths in Required Documentation MUST contain useful content by
  Phase 0 completion. README MUST be concise, practical, and accurate, covering purpose, vision,
  technology choices, architecture, installation, local development, testing, development workflow,
  Spec Kit usage, current status, and roadmap. Documents MUST link to authoritative detail rather
  than create competing feature requirements or claim unimplemented capabilities exist.
- **FR-009 — Session Continuity**: AGENTS.md MUST require future agents to read the constitution,
  product overview, and handoff; identify active specification and tasks; inspect related source;
  respect architecture and standards; implement only agreed scope; run verification; update the
  authoritative implementation checklist and handoff; and record significant decisions in ADRs.
  The handoff MUST identify current phase, active specification, completed work, pending tasks,
  blockers, decisions, verification results, and recommended next action. It MUST distinguish
  completed, partial, blocked, and unverified work and link to detailed Spec Kit tasks rather than
  duplicate them. Tasks MUST NOT be marked complete without verification.
- **FR-010 — Spec-Driven Delivery**: Each feature MUST have one authoritative Spec Kit
  specification. Constitution, specification, necessary clarification, planning, task breakdown,
  and cross-artifact analysis MUST precede approved implementation. Review gates MUST be honored;
  quality checks do not imply approval. Generated Spec Kit tooling MUST remain unchanged unless a
  concrete defect requires correction. Significant technical decisions MUST be captured in ADRs.
- **FR-011 — Dependency Discipline**: Planning MUST verify maintained releases and compatibility
  for the selected stack. Only dependencies used by actual Phase 0 functionality may be added.
  Integrations and future providers MUST remain deferred until their feature requires them.
- **FR-012 — Completion Evidence**: Completion MUST include all required checks, bootstrap success
  and failure validation, security review, module visibility and dependency-direction review, and
  repository diff review. Failures, unavailable checks, and coverage limitations MUST be reported
  accurately. Phase 0 MUST NOT deploy to production.
- **FR-013 — Minimal Structure**: One Cargo package MUST be the starting preference; a multi-crate
  workspace requires documented justification and approval. Organization MUST follow business
  features rather than global repositories/services/handlers/models layers. Create feature layers,
  platform modules, shared code, traits, and abstractions only when they contain meaningful Phase 0
  functionality. Do not create empty directories or placeholder files to match a proposed tree.
- **FR-014 — Preliminary Threat Model**: The security baseline MUST identify anticipated assets,
  actors, external entry points, trust boundaries, threats, initial mitigations, and unresolved
  risks. It MUST address token exposure, sensitive-data logging, untrusted Discord input,
  unauthorized access, and future forged/replayed completion signals without implementing identity,
  payment, or banking systems or claiming those future threats have already been eliminated.
- **FR-015 — Roadmap and Scope Control**: The feature map MUST identify intended ownership for
  identity, funds, income, payments, ledger, reports, and reminders. It MUST include deferred product
  capabilities and exclusions below as planned work only. Financial Core and its domain invariants
  MUST be the recommended next specification before Discord payment features.

### Technology Constraints

- Rust stable, Edition 2024, is the primary implementation baseline.
- Use Tokio for asynchronous execution, thiserror for explicit errors, and tracing for structured
  production diagnostics where those responsibilities exist in the foundation.
- Poise with Serenity is the selected future Discord integration; PostgreSQL is the future primary
  database, SQLx provides persistence and migrations, and Serde provides serialization when needed.
  Their selection does not require unused Phase 0 dependencies, connections, schemas, or adapters.
- Use rustfmt, Clippy, Cargo test, and GitHub Actions. Dependency versions, feature flags, toolchain
  selection, and concrete module layouts are planning decisions, not fixed by this specification.
- Building and testing MUST NOT require a running PostgreSQL instance or Discord credentials.

### Financial Integrity Constraints

These rules govern future design and documentation; Phase 0 implements no financial engine.

- Financial amounts MUST NOT use floating-point arithmetic.
- Virtual fund balances MUST be clearly distinguished from verified bank balances.
- Every financial mutation MUST have a traceable origin and support auditable payment history.
- Payment awareness MUST occur before payment handoff.
- Payment handoff MUST NOT be treated as payment confirmation or successful bank transfer.
- Manual payment confirmation MUST NOT be treated as bank verification.
- Duplicate completion MUST NOT create duplicate financial entries; future mutation design must
  preserve atomicity and idempotency.
- Sensitive financial information MUST be minimized and protected. The prohibited banking data
  and no-direct-transfer MVP boundary in FR-006 apply regardless of user count or interface.

### Future Capabilities — Design Accommodation Only

The architecture must accommodate slash commands, DMs, pattern-based message input, buttons,
select menus, modals, image and attachment processing, scheduled reminders, report images, and
payment-awareness interactions. Command handling must remain separate from business behavior.
No capability in this list is authorization to implement it during Phase 0.

### Required Documentation

- `README.md`
- `AGENTS.md`
- `docs/product/overview.md`
- `docs/product/feature-map.md`
- `docs/architecture/overview.md`
- `docs/architecture/decisions/` containing meaningful Architecture Decision Records
- `docs/development/coding-standards.md`
- `docs/development/project-structure.md`
- `docs/security/security-baseline.md` including the preliminary threat model
- `docs/ai/FEATURE_INDEX.md`
- `docs/ai/SESSION_HANDOFF.md`

### Verification Requirements

The following commands MUST pass without actual Discord tokens, production database credentials,
real banking access, or other external service accounts:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check
```

Unit and integration checks must exercise meaningful foundation behavior. Future service-dependent
checks are outside this phase. Do not claim CI passed merely because a workflow file exists.

### Key Entities *(include if feature involves data)*

- **Runtime configuration**: Only settings needed by initialization; no persisted financial data.
- **Feature delivery record**: Authoritative specification, plan, tasks, and verified status.
- No financial entities, production financial migrations, user records, or business persistence
  are created during Phase 0.

## Explicitly Out of Scope

Do not implement fund CRUD, income allocation, the financial ledger, payment-awareness evaluation,
QR scanning or parsing, OCR, AI agents/integrations, payment handoff, bank integrations, financial
reporting calculations, salary scheduling logic, production financial migrations, or production
deployment. These belong in the roadmap and future specifications, never Phase 0 implementation
tasks. Discord interactions, reminders, image/attachment processing, live persistence, multiple
bots, and additional user interfaces likewise remain future work.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A credential-free default run succeeds, records initialization, makes no external
  calls, and exits normally; all documented invalid configuration cases fail without value disclosure.
- **SC-002**: All four verification commands pass locally with zero Clippy warnings and without
  service accounts or running external services; CI is configured to run the same gates.
- **SC-003**: Every requirement and acceptance scenario maps to planned work or an explicit review
  before implementation; no critical or high cross-artifact consistency issue remains unresolved.
- **SC-004**: Every required documentation path contains useful content. A session following
  AGENTS.md can identify scope, architecture, current state, and next action without chat history.
- **SC-005**: Source and dependency review finds zero premature financial operations, external
  connection attempts, empty placeholders, circular feature dependencies, or framework dependencies
  in domain logic; absent future layers do not require placeholder implementations.
- **SC-006**: Repository review finds no real secrets or personal financial records. Constitution
  and security guidance cover FR-006, FR-014, and every Financial Integrity Constraint; unresolved
  risks are recorded with future owning scope, not represented as implemented controls.
- **SC-007**: Architecture review can place a future Discord bot, Web, or mobile adapter around the
  same application/domain boundary without duplicating business rules; no extra interface is built.

## Assumptions

1. The initial product is personal-use and single-owner; future multi-user support remains possible.
   This does not establish an authorization mechanism or make inbound Discord users trusted.
2. Discord is the initial primary UI; future multiple bots, Web, and mobile interfaces may reuse
   the backend business modules without requiring those interfaces or an HTTP API in Phase 0.
3. Rust is the primary language. PostgreSQL will persist future financial data; Phase 0 persists
   no business records and requires no deployment or production credentials.
4. The project begins as a modular monolith, preferably one Cargo package. Repository evidence
   contradicting this assumption must be recorded and resolved explicitly before changing topology.
5. Features are delivered incrementally using Spec Kit. Financial correctness and maintainability
   take priority over unnecessary performance optimization.
6. The minimal executable initializes and exits. Long-running clients, scheduler lifecycle,
   connection pools, and graceful shutdown for real integrations wait for their owning feature.
7. Local configuration uses process environment variables with safe defaults; implicit .env loading
   is not required. .env.example explains only settings actually supported by the foundation.
8. Currency representation, owner identity binding, multi-user isolation, retention, and transaction
   semantics belong to future owning specifications. No production uptime or throughput is claimed.
9. Unresolved architectural assumptions must be recorded rather than answered with speculative
   infrastructure. Concrete structure, dependency versions, and runtime configuration details are
   deferred to planning; they do not block definition of this scope.

## Constitution Consistency Review

Reviewed against `.specify/memory/constitution.md` version 1.1.0 on 2026-11-10.
The additive amendment adopts FR-006's prohibited banking data, MVP no-direct-transfer rule,
input/identity/authorization boundaries, token protection, and awareness-before-handoff ordering.
All earlier financial invariants and structured-tracing requirements remain in force.

Analysis C1 is resolved in the plan/runtime contract by a scoped fallback tracing subscriber for
startup errors, without adding an unstructured-diagnostic exception to the constitution. I1 is
resolved by making FR-001's INFO filtering explicit. E1 and U1 are reflected in the documentation
coverage and bounded-child verification tasks. The initial implementation instruction approved T001–T006; the latest maintainer instruction
authorizes completion of all remaining Phase 0 tasks. No financial feature or deployment is authorized.

## Definition of Done

Phase 0 is complete only when the Rust foundation compiles; formatting, Clippy, tests, and cargo
check pass; CI is configured; all required documentation exists; AGENTS.md establishes session
behavior; the constitution includes the required architectural and security principles; Spec Kit
artifacts are consistent; architecture and open decisions are documented; verification and diff
review results are recorded; and no out-of-scope business features or deployment were introduced.
Completing this specification refinement alone does not complete Phase 0 or authorize implementation.
