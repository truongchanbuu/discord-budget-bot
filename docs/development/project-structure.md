# Project structure

## Implemented files

| Path | Responsibility |
| --- | --- |
| src/main.rs | Tokio runtime, safe structured failure reporting, process exit |
| src/lib.rs | Private startup modules and explicit feature facades |
| src/bootstrap.rs | Configuration and diagnostics initialization; isolated collision test |
| src/config.rs | Pure log-level parsing, environment read, colocated unit tests |
| tests/bootstrap.rs | Executable contract tests under controlled child environments |
| tests/support/mod.rs | Shared bounded child-process harness with drained output pipes |
| Cargo.toml / Cargo.lock | One package, minimal declared dependencies and resolved versions |
| rust-toolchain.toml / rustfmt.toml | Pinned compiler/components and stable formatting |
| .github/workflows/ci.yml | Credential-free checks; no deployment |

The test harness is included only under cfg(test) in bootstrap, and as an integration-test module.
It creates no runtime abstraction or additional public API. Test expect/unwrap calls represent
failed test setup/assertions, never runtime-critical error handling.

## Financial Core (implemented)

src/features/financial_core/mod.rs is the public facade; domain and application paths are private.
Domain files money.rs, balance.rs, references.rs, intent.rs, and error.rs each own one cohesive
responsibility. application.rs contains the pure replay/fresh-validation workflow and private
coherent candidate fields. Module files wire only real implemented modules, not future skeletons.
The facade exports the planned value/input/result/error vocabulary and validate_intent.

tests/financial_core.rs wires money, balance, replay, and evidence public-contract modules under
tests/financial_core/. support.rs builds synthetic valid fixtures through the public API. These
fixtures create no store/provider mock. Reference unit tests live beside their domain implementation.
No application service object, infrastructure or presentation layer is needed for pure validation.
See the [Financial Core contract](../../specs/002-financial-core/contracts/financial-core.md).

## Identity (implemented)

src/features/identity/mod.rs exports only account/actor values, BindingState, OwnerBinding,
AccessError and the two access functions. domain.rs owns validated identity vocabulary;
application.rs owns immutable binding and policy, with the only cross-feature edge to Financial
Core's public OwnerRef. The domain/application modules and binding fields remain private.

Tests under tests/identity/ cover accounts, resolution, scoped access and privacy. support.rs uses
synthetic public constructors; tests/identity.rs wires real test modules. No provider mocks,
credential fixtures, new infrastructure/presentation layers or speculative modules are needed.
See the [owner-access contract](../../specs/003-owner-access/contracts/owner-access.md).

## Intended future ownership — no placeholder directories

Future feature workflows include funds, income, payments, ledger, reports and reminders; Financial
Core and pure identity policy are implemented. When behavior
exists, keep it under src/features/<feature> with domain, application, infrastructure, and
presentation layers only as justified by that feature. Domain models and rules stay together;
provider-independent use cases coordinate them; adapters implement actual dependencies.

Future src/platform/discord, database, and scheduling modules own shared integration lifecycle.
Create src/shared only for cohesive concepts actually used by multiple features. Do not create
empty directories, mod files, unused traits, or global repositories/services/handlers/models trees.

The [architecture guide](../architecture/overview.md) defines dependency direction. The
[feature map](../product/feature-map.md) describes intended ownership. Feature requirements and
implementation progress belong to [Spec Kit](../../specs/003-owner-access/tasks.md).
