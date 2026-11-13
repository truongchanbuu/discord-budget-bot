# Session Handoff

Updated: 2026-11-13

## Current phase and active feature

004 Trusted Discord Owner Entrypoint has completed task generation under the maintainer's
$speckit-tasks request, which accepts plan review for this step. [Specification](../../specs/004-discord-owner-entry/spec.md),
[plan](../../specs/004-discord-owner-entry/plan.md) and design artifacts remain authoritative.
[Tasks](../../specs/004-discord-owner-entry/tasks.md) has 38 unchecked items across Setup, Foundational,
US1 runtime selection, US2 private command, US3 safe integration, and Polish. Detailed dependency and
acceptance coverage live there. No feature-004 implementation exists; constitution 1.1.0 is unchanged.

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

Read-only analysis found A1/C1/I1 (two medium, one low; no critical or constitution conflicts).
The maintainer authorized planning remediation: T008 now distinguishes padded owner references,
T016/T027 explicitly cover duplicate acknowledgements and unsupported dispatch, and T033/T034
consistently follow T032. Focused re-review resolves all three findings; no remaining planning blocker.
Next: explicit $speckit-implement authorization. Do not start code automatically.
US1 is an independently testable configuration/dispatch checkpoint; live executable wiring waits until
US2 privacy and US3 lifecycle/error safeguards exist. All 38 tasks remain unchecked.

Task generation and remediation validated format/IDs/dependencies, FR/AC/SC coverage, artifact links, file sizes and
whitespace. No before/after task hooks are configured. No application checks were rerun for this
documentation-only step; no new runtime/CI/live passing result is claimed. The task-generation
session began with a clean tree and changes documentation only.

Manual live smoke is opt-in and uses locally supplied credentials only. Report it separately from
simulated coverage; no live integration assurance follows from the earlier dependency probe.
No financial operations, resource lookup, persistence, privileged intents or deployment are included.
Use [FEATURE_INDEX.md](FEATURE_INDEX.md) when the ignored selector is absent. Active directory is
specs/004-discord-owner-entry; actual Git branch remains 001-project-foundation.
