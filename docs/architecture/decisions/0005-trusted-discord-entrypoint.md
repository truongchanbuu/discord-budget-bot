# ADR 0005: Opt-in Discord owner entrypoint with bounded lifecycle

Date: 2026-11-13. Status: Proposed — planning recommendation, not implementation approval.

## Context

The completed identity policy trusts callers to provide authenticated actors and operator bindings.
Feature 004 connects those inputs for one private local readiness command, with no financial data.
Existing offline initialization and business APIs must remain usable without credentials/services.

## Proposed decision

Use defaults-disabled Poise/Serenity with rustls and zero gateway intents. Enable the client only
through explicit runtime selection after full environment validation. Retain synchronous offline
run and add an async selected-runtime root entrypoint; do not add a second binary or nested runtime.
Put command mapping in identity presentation and SDK lifecycle in platform, with no provider types
or environment access entering domain/application. Keep the existing one-way Financial Core reuse.

Use one named guild-command upsert, never Poise's bulk registration helper. Set private reply
visibility explicitly, use fixed text and one bounded send, and replace default framework errors.
Whitelist reviewed application tracing targets at all log levels because SDK errors/instrumentation
can retain sensitive payloads. Do not claim this filtering is a complete production security audit.

Supervise setup results independently of Client::start, since Poise setup failure does not terminate
that future. Bound startup/registration/response/recovery and shared shutdown cleanup deadlines;
keep SDK rate limiting but do not mistake logical operation limits for wire-request counts.

## Consequences and alternatives

This adds two justified direct SDK dependencies and an unavoidable transitive integration graph,
without enabling cache, privileged event access, business persistence or a generic provider engine.
A narrow injected future at real I/O boundaries permits offline tests; it does not authenticate real
users. Optional live smoke and hosted CI require separate evidence.

Bulk registration can delete unrelated commands. Default errors/logging can disclose provider data.
An unsupervised setup callback can hang while awaiting nonexistent user_data. Those simpler-looking
options therefore fail concrete requirements. SDK-only owner checks conflate app ownership with
FundGuard ownership. Global registration, HTTP callbacks, multi-bot abstractions and Cargo feature
matrices broaden scope without present need.

Binding changes take effect on process restart. Current owner policy does not establish authoritative
resource ownership; future financial operations must add that boundary and atomic audit/idempotency.
No deployment, financial command or production credential is approved by this ADR.

See [plan](../../../specs/004-discord-owner-entry/plan.md),
[research](../../../specs/004-discord-owner-entry/research.md),
[runtime contract](../../../specs/004-discord-owner-entry/contracts/runtime.md) and
[command contract](../../../specs/004-discord-owner-entry/contracts/command.md).
