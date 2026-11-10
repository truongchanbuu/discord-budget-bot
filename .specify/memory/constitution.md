# FundGuard Constitution

## Core Principles

### I. Human-readable, idiomatic Rust

Code MUST favor clear names, cohesive responsibilities, explicit control flow, and Rust ownership
and type safety. Clean Code is guidance, not a mandate for arbitrary function lengths or tiny
wrappers. Abstractions, clones, traits, and generics MUST solve a demonstrated problem. Runtime
errors MUST use Result and structured error types; panic-based handling requires a documented
invariant. Production diagnostics MUST use structured tracing.

### II. Feature-first boundaries

FundGuard MUST remain a modular monolith organized by business feature. Domain code MUST be pure
and independent of Discord, persistence, HTTP, and AI frameworks. Application code coordinates
use cases and declares ports only at real boundaries. Infrastructure implements adapters;
presentation maps user interactions to use cases. Platform owns integration lifecycle and the
composition root wires dependencies. Cross-feature access MUST use explicit application APIs;
circular dependencies and access to another feature's internal adapters are prohibited.
Only meaningful implemented modules and layers may be created.

### III. Financial integrity

Financial amounts MUST NOT use floating-point arithmetic. Future financial specifications MUST
distinguish virtual funds from verified bank balances, awareness from payment handoff, handoff
from confirmation, and manual confirmation from bank verification. Financial mutations MUST have
traceable origins, auditable history, and atomic idempotency so duplicate completion cannot
produce duplicate entries. These are design constraints in Phase 0, not permission to implement
financial behavior. Financial Core and its invariants MUST be specified before payment features.

### IV. Minimal dependencies and replaceable integrations

Dependencies MUST serve current functionality, with maintained releases and compatibility
verified before introduction. Prefer standard-library facilities. Discord handling MUST stay
outside business code and use minimal gateway intents and least privilege. Multiple future
Discord applications MUST be able to reuse the business modules without duplicating them.
Persistence, schedulers, QR, OCR, AI, and payment providers MUST wait for a feature that needs them.

### V. Verified delivery and persistent context

Each feature MUST have one authoritative Spec Kit specification. Changes MUST trace to the
active spec, plan, and tasks. Relevant checks and diff review MUST precede completion claims;
unverified checks and incomplete work MUST be labeled honestly. Future sessions MUST consult
constitution, product overview, handoff, active specification, related source, and task checklist.
Tasks and handoff MUST be updated after work; significant architecture choices require an ADR.

## Security and Technology Constraints

Rust stable with Edition 2024 is the baseline. Tokio, Poise/Serenity, PostgreSQL/SQLx, Serde,
thiserror, and tracing are preferred when required by current scope, not a dependency checklist.
Secrets and sensitive financial information MUST NOT appear in logs, committed configuration,
error messages, or test fixtures. Configuration MUST validate input and fail without exposing it.
External inputs MUST be validated at their trust boundary. Future use cases MUST authorize the
bound user before acting; personal-use scope MUST NOT imply trust in all inbound users. Discord
bot tokens MUST be protected as secrets and MUST NOT be committed or logged. The application
MUST NOT collect or store banking passwords, OTPs, card PINs, CVVs, or banking session credentials,
and MUST NOT directly execute real-money transfers in the MVP. Payment awareness MUST precede
payment handoff; handoff MUST NOT imply successful payment or bank verification. These are future
design constraints, not permission to implement identity, authorization, or banking in Phase 0.
CI MUST have minimal permissions. Tests MUST run without real credentials or external services.
Phase 0 MUST initialize locally without Discord or database calls and MUST NOT deploy anything.

## Delivery Workflow

Use installed Spec Kit skills: constitution, specify, clarify when needed, plan, tasks, analyze,
and implement. Honor review gates and record approval separately from automated quality checks.
Generated tooling MUST remain unchanged unless a concrete tooling defect requires it.
Every implementation MUST pass cargo fmt --all -- --check, cargo clippy --all-targets -- -D warnings,
cargo test, and cargo check, or explicitly report failures. Commit the application lockfile.
Tests MUST check observable behavior and meaningful boundaries; avoid tests that mirror syntax.
Review module visibility, dependency direction, security implications, and repository diff.

## Governance

This constitution governs specifications and implementation. Proposed amendments MUST explain
rationale, affected artifacts, compatibility, and migration needs and receive maintainer approval.
Use semantic versions: MAJOR for incompatible governance changes, MINOR for added principles,
PATCH for clarifications. Reviews MUST check compliance and record justified exceptions in plans
and ADRs; exceptions cannot silently weaken financial integrity or security requirements.
The initial version adopts the user's Phase 0 principles; implementation approval remains a
separate review gate. Remove the temporary sync report before committing the constitution.

**Version**: 1.1.0 | **Ratified**: 2026-11-10 | **Last Amended**: 2026-11-10
