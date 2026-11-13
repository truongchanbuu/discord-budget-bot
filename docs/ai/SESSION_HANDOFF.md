# Session Handoff

Updated: 2026-11-13

## Current phase and active feature

004 Trusted Discord Owner Entrypoint has completed technical planning under the maintainer's
$speckit-plan request. [Specification](../../specs/004-discord-owner-entry/spec.md) scope is accepted
for planning; [plan](../../specs/004-discord-owner-entry/plan.md), research, data model, runtime/
command contracts and quickstart are ready for review. No tasks.md or feature-004 implementation
exists. Constitution 1.1.0 is unchanged; no implementation approval is inferred.

## Planned design and decisions

Keep offline default and legacy run() offline-only; introduce async start() for selected runtime.
Complete operator configuration precedes provider access. Identity presentation maps genuine command
actor/guild to existing owner policy, and platform supervises one client with zero gateway intents.
Named guild-command upsert preserves unrelated commands; all replies explicitly private. Replace
Poise default errors and allow only reviewed application tracing targets, even at trace/debug.

Proposed Poise 0.7.1/Serenity 0.12.5 disable defaults with rustls enabled. Existing Tokio gains
signal/sync/time, plus dev-only test-util. A setup result signal is required: SDK Client::start alone
does not propagate framework setup failure. Startup 30s, registration 10s within startup, reply 2s,
observed recovery 30s, SDK shutdown 8s and total close/cancel/join 10s. SDK internal retry counts are
not application/wire-attempt guarantees. Details are authoritative in the runtime/command contracts.
[ADR 0005](../architecture/decisions/0005-trusted-discord-entrypoint.md) remains proposed.

## Verification and limits

Pre/post-design constitution checks pass without exceptions. All 16 requirements, fourteen AC and
six outcomes have planned tests/reviews. Specification-quality checklist remains 16/16 and was not
modified. Planned private unit tests will exercise real mapper/builders and bounded operations;
binary tests will cover offline/invalid input. Automated tests must use no real sockets or credentials;
no production fake mode is planned.

Read-only research checked published source and primary Discord documentation. An isolated /tmp
probe resolved the exact SDK/current dependency combination and compiled a minimal framework builder
on Rust 1.99.1, Edition 2024 without warnings. This is dependency compatibility evidence, not a
FundGuard feature test, vulnerability audit or live authentication/delivery claim.
Validated artifact links, prerequisites, coverage, file sizes, whitespace and unchanged protected
source/tooling/configuration. No before/after planning hooks are configured.

Repository Cargo checks were not rerun because planning changes documentation only. The repository
manifest/lockfile/toolchain and all implementation source/tests remain unchanged. Prior 37-test
results belong to completed [owner-access tasks](../../specs/003-owner-access/tasks.md). This session
began with a clean working tree; existing committed instruction/source changes are preserved.
Hosted CI and live Discord smoke remain unverified; no token/account/server was used.

## Pending work, blockers and next action

No unresolved planning blocker. Review the plan/contracts and proposed ADR, then request
$speckit-tasks. The review-plan gate precedes task generation. Later analyze all artifacts and
explicitly authorize implementation; do not start code or generate tasks automatically.

Manual live smoke is opt-in and uses locally supplied credentials only. Report it separately from
simulated coverage; no live integration assurance follows from the dependency probe. No financial
operations, resource lookup, persistence, privileged intents, message handling or deployment.
Use [FEATURE_INDEX.md](FEATURE_INDEX.md) when the ignored selector is absent. Active directory is
specs/004-discord-owner-entry; actual Git branch remains 001-project-foundation.
