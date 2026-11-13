# Implementation Plan: Trusted Discord Owner Entrypoint

**Branch**: `004-discord-owner-entry` (feature selector; actual Git branch `001-project-foundation`)
**Date**: 2026-11-13 | **Spec**: [spec.md](spec.md)
**Input**: specs/004-discord-owner-entry/spec.md
**Status**: Design generated under the maintainer's $speckit-plan request. Scope accepted for planning;
Subsequent $speckit-tasks request accepts plan review for task generation; implementation is not authorized.

## Summary

Connect the existing owner policy to one explicitly enabled local Discord session and one private
/fundguard readiness command. Offline remains default. Validate trusted configuration before I/O,
use actual invoking identity, named guild-command upsert, minimal event access, safe errors/logging,
and externally bounded startup/recovery/response/shutdown. No financial/resource operation.

Preserve synchronous run() as an explicitly offline library API. Add async start() for the executable,
with one current-thread runtime and a small outer session supervisor. Domain/application stay unchanged;
command presentation belongs to identity, client lifecycle to platform, wiring/configuration to root.

## Technical Context

**Language/Version**: Existing Rust 1.99.1 stable, Edition 2024; preserve pinned toolchain.
**Primary Dependencies**: Add Poise 0.7.1 (default-features=false) and Serenity 0.12.5
(default-features=false, rustls_backend). Keep Tokio 1.54.1 and enable macros/rt/signal/sync/time;
dev-only Tokio test-util for paused-clock verification. Existing tracing/thiserror versions retained.
**Dependency Evidence**: Research's isolated exact-release metadata/framework probe compiled without
warnings on this compiler. Review/commit actual application Cargo.lock during implementation;
repository manifest/lockfile are unchanged by planning. No direct SQLx/Serde/HTTP/auth/secret crate.
**Storage**: None. Immutable startup configuration/binding, transient connection/closing state only.
**Testing**: Existing Cargo suites plus private boundary/unit tests and discord_entry binary tests;
paused Tokio time, synthetic real SDK fixtures/builders and narrow captured I/O futures. No real credentials.
**Target Platform**: Existing local POSIX development and CI; local Ctrl-C shutdown, no deployment.
**Project Type**: One Cargo package and one executable; normal build includes Discord adapter;
connected behavior requires explicit runtime mode, not a new Cargo feature/build matrix.
**Performance Goals**: Immediate constant-text private response, 2s send budget within the 3s
provider response window; 10s maximum shutdown. No financial throughput or benchmark target.
**Constraints**: 30s total startup; named registration at most 10s within startup; observed recovery
at most 30s; no app retry. SDK rate-limit retries remain within absolute operation deadlines.
**Scale/Scope**: One app, one configured guild, one owner binding, one client/shard and one command;
no privileged intents, prefix/DM/content handlers, roles, persistence, hot reload or money movement.
All research unknowns are resolved in [research.md](research.md); finite limits are explicit below.

## Constitution Check

Pre-research and post-design gates: **PASS**; no exception or constitution amendment required.

| Principle | Pre-research requirement | Post-design assessment |
| --- | --- | --- |
| I. Readable Rust | Cohesive values/functions and errors | Private concrete config/data, safe categorical errors, no generic service stack; async only for real I/O |
| II. Feature boundaries | SDK outside business rules | Identity presentation → existing policy; platform owns lifecycle; root composes; Financial Core unchanged, no reverse edge |
| III. Financial integrity | No false payment/bank semantics | Static readiness explicitly offers no financial operations; no resource scope, entries, balances or mutations |
| IV. Minimal dependencies | Real interface justifies SDK | Defaults disabled, checked versions/features, zero gateway intents, no DB/scheduler/AI SDK |
| V. Verified delivery | Spec and evidence separated | FR/AC mapping below; future tasks own execution evidence; proposed ADR and handoff maintain state |
| Security | Least privilege, safe inputs/output | Explicit opt-in, complete trusted binding, private responses, log target allowlist and custom framework errors |
| Workflow | Planning explicitly requested | No code/tasks/live calls; spec scope accepted for design, plan review precedes task generation |

Phase 0 itself stays offline; this is a new opt-in post-Phase-0 interface. Default executable behavior
and the offline run API remain compatible. Credential-free tests and no-production-deployment rule hold.

## Project Structure

### Documentation (this feature)

```text
specs/004-discord-owner-entry/
├── spec.md
├── checklists/requirements.md
├── plan.md
├── research.md
├── data-model.md
├── contracts/
│   ├── runtime.md
│   └── command.md
└── quickstart.md
```

[tasks.md](tasks.md) was generated in the subsequent task phase. The runtime contract owns
settings/budgets/root behavior; command
contract owns transport mapping, privacy/registration and test seams; model owns fields/invariants.

### Source Code (planned, not created)

```text
src/
├── main.rs                            # await selected entrypoint, safe fallback/error event
├── lib.rs                             # retain run/StartupError, export start/ApplicationError
├── bootstrap.rs                       # offline initialization, shared logging allowlist
├── runtime.rs                         # selected-runtime composition, safe root error mapping
├── config.rs                          # existing logging parser, declare live config modules
├── config/
│   ├── runtime.rs                     # explicit mode/lookup selection; offline ignores bot fields
│   └── discord.rs                     # complete live validation and redacted credential/config
├── features/identity/
│   ├── mod.rs                         # existing facade plus crate-visible presentation boundary
│   └── presentation.rs                # real slash command, mapping, reply and adapter data/error
└── platform/
    ├── mod.rs                         # only real discord module
    └── discord/
        ├── mod.rs                     # narrow crate-private session entrypoint
        ├── client.rs                  # SDK client/framework/setup wiring
        ├── lifecycle.rs               # supervisor, connection sampling and deadline/close handling
        ├── registration.rs            # bounded named upsert
        └── diagnostics.rs             # safe session categories/framework error handling
tests/
└── discord_entry.rs                   # real executable default/invalid modes under env_clear
```

Private unit tests live with their implementation; split into sibling tests files only when actual
responsibility/200–250-line guidance requires it. Reuse existing tests/support process harness.
Do not expose internals or create a fake production mode just to test them. All new modules contain
real behavior; no new domain, generic provider hierarchy, public config API or financial feature.

## Design and Dependency Direction

Config runtime selection reads only FUNDGUARD_MODE first; offline ignores every Discord-only field.
Discord mode validates credential, target guild, owner account/ref and binding state before any SDK
operation. All settings/errors/Debug rules are in the runtime contract. Existing logging validation
and initialization then run before connection. Library run() never connects, even if mode is discord;
main invokes the new async start() which selects actual executable behavior.

Identity presentation owns CommandData (binding/guild/accepting), ReadinessDecision and a safe
ReadinessError for Poise. It maps real command invoking user/guild to existing resolve_owner; no
fake resource owner, duplicate policy or platform import. Platform receives the feature command and
owns builder/setup/supervision. Credentials exist only in config/composition/SDK initialization,
not command data or domain/application. Poise error callbacks are installed by platform and return
safe categories without printing or replying again. All SDK/request logging targets are suppressed.

Use explicit ephemerality in both command definition and every real reply builder. Fixed text and
empty allowed mentions; one private-send future, no auto-defer/retry/public fallback. All policy
failures have the same denial. A possibly accepted send that times out is not retried. SDK fixture
and serialized real-builder tests must exercise production mapping rather than a parallel fake path.

Named upsert occurs only once in setup, after authenticated Ready initializes application identity.
Do not use bulk/global helpers or defaults that discover application owners. On setup failure,
signal the outer supervisor explicitly before returning Err: Client::start alone will not propagate
this case. Command-ready reporting requires successful connection/registration, not local init.

An accepting flag is lifecycle-only. Supervisor checks setup outcome, tracked client result, Ctrl-C,
absolute startup/recovery deadlines and current shard status. Observe runners/stage every 1s after
first ready; do not depend on reordered stage callbacks. On close stop new work, bound SDK shutdown
and cancellation/join by one total deadline, then return clean-interrupt success or safe failure.

## Budgets, Compatibility and Reporting

| Boundary | Limit / outcome |
| --- | --- |
| Runtime selection/configuration | No network; missing/invalid explicit live fields fail closed |
| Client construction through setup | 30s total absolute deadline; one logical startup attempt |
| Named registration | One upsert; 10s or remaining startup budget, whichever is smaller |
| Private response | One send; 2s, immediate dispatch; no retry/second reply |
| Observed recovery | 30s from sampled loss; Connected resets, intermediate stages do not |
| Shutdown | At most 8s shutdown_all; total close/abort/join at most 10s |

These bound logical SDK operations, including internal rate-limit waits/retries. Do not claim a
single HTTP transmission. Keep SDK rate limiting enabled. A terminal SDK error, setup signal error,
closed setup channel, unexpected client completion, exhausted budget or failed cleanup returns a
payload-free ApplicationError. Its event_name selects safe init/session failure reporting; main
preserves the existing scoped ERROR fallback. Local initialized and discord_ready are distinct events.

Global and fallback logging allow only fundguard and fundguard::… targets at selected level. No
ambient RUST_LOG override, dependency warnings, raw SDK errors, interaction/config dumps or real ID
fields. Retain current ANSI/stdout guarantees. Capture SDK-target synthetic events at every level
in tests; merely testing custom error Display does not cover this boundary.

## Verification and Requirement Coverage

| Requirement | Planned tests/review and acceptance coverage |
| --- | --- |
| FR-001 | Binary/default/explicit offline, ambient malformed bot values, zero connector calls; AC-01 |
| FR-002 | Config matrix: exact modes, each missing/invalid/non-text field, valid disabled binding; AC-02–04 |
| FR-003 | Existing value constructors, exact owner bytes, no fallback or first-contact enrollment; AC-03/07/08 |
| FR-004 | Nested config/token/error formats, all-level primary/fallback target-filter tests and raw provider markers; AC-03/09/11 |
| FR-005 | Actual command definition name/type/zero args, constant readiness/denial text; AC-05/06/10 |
| FR-006 | Actual SDK user/guild fixture mapping; spoofed names/content, missing/foreign context; AC-05–08 |
| FR-007 | Existing policy used per request; disabled/unknown actor, repeated calls without grants; AC-05–08 |
| FR-008 | Fixed content without IDs, generic denial for all context/policy cases; AC-05–09 |
| FR-009 | Serialize real response builder, private flags, one timed send/no fallback, paused clock; AC-09 |
| FR-010 | Named upsert only, correct scope, unrelated commands preserved, setup-once across reconnect; AC-10/11 |
| FR-011 | Intents empty, scopes/permissions review, no prefix/DM/member/content path; AC-08/10/14 |
| FR-012 | Setup result plus pending client, safe lifecycle events/terminal exit, bounded interrupt; AC-11–13 |
| FR-013 | Paused deadlines, builder/setup/recovery errors, registration/response call counts and no outer retry; AC-09–13 |
| FR-014 | Import/dependency/visibility/full-diff review; core/policy source unchanged; AC-14 |
| FR-015 | Four Cargo gates, retained 37-test baseline plus new unit/binary cases; no real connections; AC-14 |
| FR-016 | Opt-in manual smoke guide and explicit pass/fail/unverified result slots; live evidence never inferred |

SC-001 maps to configuration; SC-002 to actor/server/owner policy; SC-003 to real private builders/
deadline tests; SC-004 to named registration; SC-005 to full verification and explicit evidence
limits; SC-006 to terminal/shutdown/scope tests. All 14 AC are accounted for, with source/guidance
review where live facts cannot be established offline. No manual smoke has been run by planning.

Use one private FnOnce/future seam per actual send/upsert boundary, and concrete task/channel inputs
for the real supervisor's controlled tests. No sockets in automated tests, no parent env mutation,
no endpoint configured via environment for fakes. Tokio test-util permits paused time. Unit tests
must include setup failure while client stays pending and all shutdown awaits bounded by the same
deadline. Binary tests validate default/invalid config; valid-live selection is tested via a captured
internal connector rather than contacting Discord. Actual SDK method wiring must also be reviewed.

## Review and Delivery Sequence

Plan review is accepted for task generation by the maintainer’s subsequent $speckit-tasks request.
Run cross-artifact analysis on the generated tasks before authorized implementation.
Implement validated configuration and compatible root dispatch; real feature command/mapping/private
responses; scoped registration/client wiring; bounded supervisor/diagnostics; full tests/documentation.
The future task list must sequence test-first cases and all operation limits without speculative layers.

On delivery update README, .env.example, architecture/structure/security docs, feature map/index,
devlog and handoff. Keep offline API, live adapter implementation, simulated checks and real smoke/CI
status distinct. Proposed [ADR 0005](../../docs/architecture/decisions/0005-trusted-discord-entrypoint.md)
records consequential adapter choices; adoption requires reviewed implementation authorization.

## Complexity Tracking

No constitution exceptions. Two SDK dependencies and Tokio features serve the first real interface.
Explicit supervisor signaling, logging filtering and named registration address verified SDK behavior;
they are not a generalized workflow engine. No persistence, HTTP server, multi-bot deployment,
financial use-case object, secret store or new trait hierarchy is introduced.
