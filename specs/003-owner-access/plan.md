# Implementation Plan: Owner Identity and Access Boundaries

**Branch**: `003-owner-access` (feature selector; actual Git branch `001-project-foundation`)
**Date**: 2026-11-12 | **Spec**: [spec.md](spec.md)
**Input**: specs/003-owner-access/spec.md
**Status**: Implemented and verified locally on 2026-11-12 under the maintainer's
$speckit-implement instruction. This plan records the adopted design; [tasks.md](tasks.md) owns evidence.

## Summary

Add a pure single-owner policy under features::identity. Validate canonical Discord account input,
retain explicit actor/binding roles, resolve only the configured enabled owner, and authorize only
matching authoritative resource-owner scope. Return a borrowed existing OwnerRef or generic denial.
No persistent grant, provider authentication, resource lookup, binding loader or financial action.

Reuse Financial Core's existing OwnerRef through the identity application boundary. Keep identity
domain independent, preserve the existing Financial Core API, and introduce no new dependency.
Research resolves numeric grammar, ownership, cross-feature direction, trust limits and diagnostics.

## Technical Context

**Language/Version**: Rust 1.99.1 stable, Edition 2024; preserve rust-toolchain.toml.
**Primary Dependencies**: std and existing thiserror 2.0.21 for this feature. Existing startup uses
Tokio 1.54.1, tracing 0.1.44 and tracing-subscriber 0.3.23; no manifest/lockfile/feature changes.
**Storage**: None; immutable supplied inputs, no binding registry or accounts table.
**Testing**: Cargo test public-contract integration cases; existing foundation/Financial Core tests;
rustfmt, Clippy, cargo check; targeted external compiler privacy probes outside the repository.
**Target Platform**: Existing local Rust environment and GitHub Actions runner; no deployment change.
**Project Type**: One Cargo package; public library feature with existing offline executable.
**Performance Goals**: Account parsing accepts at most 20 bytes; policy evaluates one binding and
exact owner equality. No throughput/latency SLA, timing-equivalence claim, benchmark or optimization
framework is justified. Owner comparison uses existing strings; no extra owner length cap.
**Constraints**: No network, storage, environment reads, clock, global state or mutation in policy;
no secrets/identity payloads in diagnostics; preserve user edits and meaningful private APIs.
**Scale/Scope**: One Discord account binding to one internal owner; explicit optional inputs deny
when absent. No roles, account linking, multi-user administration, sessions, bots or HTTP API.
All research unknowns are resolved; future integrations remain explicitly excluded.

## Constitution Check

Gate evaluated before research and re-evaluated after design: **PASS**, no exception requested.

| Principle | Before research | Post-design evidence |
| --- | --- | --- |
| I. Human-readable Rust | Use cohesive concrete values and functions | Named BindingState, Result errors, borrowed owner; no traits, generic engine, arbitrary splitting or async validation |
| II. Feature-first boundaries | Identity owns access policy | Domain uses std/thiserror; application → Financial Core public facade only; no reverse edge/private imports |
| III. Financial integrity | Preserve Financial Core | No financial state, amounts, evidence, replay semantics or durable-commit claims changed |
| IV. Minimal integrations | No live provider needed | Manifest/lockfile unchanged; no SDK, storage, runtime or provider abstraction added |
| V. Verified delivery | Specification is authoritative | AC/FR coverage below; tasks own verified 37-test/four-gate evidence, handoff/index updated |
| Security | Deny by default and protect identifiers | No content-as-identity, generic Denied, checked constructors, redacted diagnostics, caller trust obligations |
| Governance | $speckit-plan explicitly requested | Spec/plan/tasks reviewed; implementation explicitly authorized; ADR accepted |

## Project Structure

### Documentation (this feature)

```text
specs/003-owner-access/
├── spec.md
├── checklists/requirements.md
├── plan.md
├── research.md
├── data-model.md
├── contracts/owner-access.md
└── quickstart.md
```

[tasks.md](tasks.md) was generated in the separately requested $speckit-tasks step.
[research.md](research.md) records decisions/alternatives and primary sources; the model owns
fields/ownership; the contract owns signatures and exact validation/denial semantics.

### Source Code (implemented)

```text
src/features/
├── mod.rs                         # add only the live identity export
├── financial_core/                # existing, unchanged
└── identity/
    ├── mod.rs                     # narrow public facade; private modules
    ├── domain.rs                  # validated actor/account vocabulary and categorical errors
    └── application.rs             # binding and pure owner/scope policy
tests/
├── identity.rs                    # public-contract module wiring
└── identity/
    ├── accounts.rs                # grammar/bounds and owner compatibility
    ├── resolution.rs              # current enabled binding and deny defaults
    ├── scope.rs                   # authoritative owner match/revocation
    ├── privacy.rs                 # diagnostics and input-channel independence
    └── support.rs                 # minimal synthetic constructor fixtures
```

**Structure Decision**: Keep a single package. The small domain vocabulary fits one cohesive file;
only split if actual code responsibility/size warrants it. New files stay within AGENTS.md's
200–250-line guidance. Module files wire implemented functionality only. No infrastructure,
presentation, platform/discord or shared directory is needed. src/lib.rs already exports features;
only src/features/mod.rs needs new wiring. Existing main/bootstrap/config behavior stays unchanged.

## Design and Dependency Direction

Identity domain contains DiscordAccountId (private NonZeroU64), TrustedActor, BindingState and
AccessError. Validated account input is not proof a Discord account exists or was authenticated.
TrustedActor::new explicitly records a caller assertion; it must not become a fake proof token.

Identity application owns private-field OwnerBinding because it combines domain values with the
existing Financial Core OwnerRef. It imports OwnerRef only through features::financial_core.
Financial Core imports no identity code. Future reverse dependence requires explicit ownership
review; do not silently introduce a cycle or move OwnerRef into shared during this feature.

OwnerBinding::new takes validated account/owner plus Enabled/Disabled, owns the mapping, and has
no setters/public raw fields. No owner validation is duplicated. Account and actor may Copy; keep
OwnerBinding non-Clone unless concrete use later justifies it. Domain-to-application access to an
actor's account uses a narrow crate-visible accessor, not a public raw-number accessor.

Two synchronous application functions have no provider parameters. resolve_owner takes optional
actor and binding; authorize_scope additionally takes optional authoritative OwnerRef. Both return
Result<&OwnerRef, AccessError> tied to the binding. Resolution denies absent actor/binding, disabled
state or account mismatch. Scoped authorization resolves first, then denies missing/foreign scope.
Every rejection is identical Denied. Account construction uses InvalidAccountId; existing owner
construction retains CoreError. There is no result grant wrapper or previous-success parameter.

All sensitive Debug forms redact values; no raw Display or parser/source error is exposed. No
logging is emitted by policy. Deliberate owner access is needed by future use cases but must not
be logged. Borrowing prevents unnecessary copies, not authorization bypass by malicious local
code. Future callers still authenticate actors, load trusted bindings, fetch authoritative scope,
and re-evaluate current authorization before every private use case, including replays.

## Verification and Requirement Coverage

Tests must be authored before new behavior, then verified at each story checkpoint. Reuse valid
synthetic constructors in support.rs, not a repository/provider mock. Keep test expectations based
on public contract and meaningful input/state boundaries. No business-domain mutation tests.

| Requirement | Planned verification |
| --- | --- |
| FR-001, FR-004 | resolution: exact enabled mapping, absent actor/binding, unknown actor, no fallback; AC-01–03 |
| FR-002, FR-003 | accounts: full lexical/range matrix plus unchanged OwnerRef blank/exact-byte behavior; AC-04 |
| FR-005 | scope: matching/foreign/missing owner and forged owner claim; AC-05–07 |
| FR-006 | resolution/scope: all rejection reasons equal Denied, safe error chains; AC-02/03/06/07/11 |
| FR-007 | scope: success then disabled/absent/changed binding; no previous-success API; AC-08 |
| FR-008, FR-012 | contract caller-obligation review and API inspection; no names/content/channel bypass; AC-09/10/12 |
| FR-009 | repeated supplied inputs, unchanged binding/actor/scope, source/import review; AC-01/05/09/12 |
| FR-010 | privacy: numeric/string markers absent from Debug/pretty/nested Result and every error/source; AC-11 |
| FR-011 | facade/privacy compiler probes, dependency/visibility review, no reverse edge; AC-12 |
| FR-013 | all 12 AC verified, four Cargo gates and complete existing suite retained |

SC-001 maps to resolution; SC-002 to scope; SC-003 to account/owner boundary tests; SC-004 to
privacy/denial tests; SC-005 to full verification; SC-006 to source/API/scope and guidance review.
AC-10 is document review because no live adapter is built. AC-12 combines pure behavioral tests
and source/API review. Do not claim all scenarios are live-interface end-to-end tests.

Account cases: 1, MAX, MAX+1, 0, 00, 01, empty, plus/minus, whitespace, Unicode digits, decimals,
exponents, separators, mentions and overlong text. Owner cases: blank Unicode whitespace, exact
case/padding preservation and mismatch. Scope matrix crosses absent/matching/nonmatching actor,
absent/enabled/disabled binding, and absent/matching/foreign owner. Denied exposes no identity.
After success, change/remove/disable binding and verify a fresh evaluation uses only current input.

Use temporary external rustc probes to confirm raw account construction, private module imports and
binding field modification are rejected. Avoid a permanent compile-test dependency/harness.
Run cargo fmt --all -- --check, cargo clippy --all-targets -- -D warnings, cargo test and cargo check;
also run cargo test --test identity after target creation. Existing CI discovers that test target.
All four gates and the identity target passed locally: 37 tests total, including 10 new identity
tests. Lockfile/toolchain are unchanged. Five external compiler probes verified private boundaries;
see tasks.md for exact evidence. Hosted CI remains unverified.

## Review and Delivery Sequence

After task generation and read-only analysis, the maintainer authorized implementation. Account/
actor values, safe errors, binding/resolution, scoped authorization and privacy tests were delivered
in dependency order. Both minor analysis wording issues were clarified without behavior changes.

README, product/architecture/structure/security docs, index, devlog and handoff now describe actual
policy availability and future caller obligations. [ADR 0004](../../docs/architecture/decisions/0004-owner-access-policy.md)
is accepted. tasks.md owns the detailed verification records. No live authentication, configuration
loader, storage or financial workflow was added; existing user edits were preserved.

## Complexity Tracking

No constitution violation or complexity exception. Existing OwnerRef reuse is one declared inward
application-level dependency, not a shared-kernel refactor. The design adds only meaningful values,
a concrete binding and two policy functions; no traits, service object, generic engine, speculative
layers or new dependencies.
