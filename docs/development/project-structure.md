# Project structure

## Implemented files

| Path | Responsibility |
| --- | --- |
| src/main.rs | Tokio runtime, safe structured failure reporting, process exit |
| src/lib.rs | Private startup/platform modules, explicit feature facades and run/start root APIs |
| src/runtime.rs | Runtime selection/composition and safe ApplicationError |
| src/config/runtime.rs / discord.rs | Supplied-value mode parsing and redacted live configuration |
| src/features/identity/presentation.rs | Real slash command, actor mapping, private replies |
| src/platform/discord/ | Client/setup wiring, registration, diagnostics and bounded lifecycle |
| tests/discord_entry.rs | Credential-free executable selection/failure checks |
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
real credential fixtures or speculative modules are needed. Identity now also has a crate-private
presentation module for the real SDK command; private boundary tests live under presentation/.
See the [owner-access contract](../../specs/003-owner-access/contracts/owner-access.md).

## Intended future ownership — no placeholder directories

Future feature workflows include funds, income, payments, ledger, reports and reminders; Financial
Core and pure identity policy are implemented. When behavior
exists, keep it under src/features/<feature> with domain, application, infrastructure, and
presentation layers only as justified by that feature. Domain models and rules stay together;
provider-independent use cases coordinate them; adapters implement actual dependencies.

src/platform/discord now owns client lifecycle. Future database and scheduling modules will own
their respective integration lifecycle.
Create src/shared only for cohesive concepts actually used by multiple features. Do not create
empty directories, mod files, unused traits, or global repositories/services/handlers/models trees.

The [architecture guide](../architecture/overview.md) defines dependency direction. The
[feature map](../product/feature-map.md) describes intended ownership. Feature requirements and
implementation progress belong to [Spec Kit](../../specs/003-owner-access/tasks.md).

## Personal journal (005)

src/features/ledger/domain contains redacted models, finite parser, historical posting validation and
monthly arithmetic. application defines owner-authorized Journal and its genuine persistence port.
infrastructure implements static parameterized SQL/mapping/draft/commit/query operations. presentation
owns actual DM SDK events, private component/modal flow and Vietnamese views. Sibling tests keep each
responsibility/file bounded. Ledger does not create provider clients or access another feature's internals.

src/platform/database.rs owns bounded pool/embedded migration startup; migrations/0001_personal_journal.sql
owns schema constraints and immutable-entry trigger. runtime/client wire the optional pool/service and
shared owner binding. scripts/test-postgres.sh creates/stops only its own disposable private cluster.
No future funds/reports directories or speculative shared abstractions were created.

## Private finance spaces (006)

src/features/finance_spaces owns domain refs/privacy/operation records, application workflows/ports,
SQL and Discord adapters, and presentation messages/components/reference validation. Related tests
live beside each responsibility, split by fault/ownership/lifecycle/limits rather than empty layers.
Ledger calls the Spaces application before its existing journal store; neither domain/application
imports framework types. The existing fg_owners table is the explicitly shared identity anchor;
Spaces never reads/posts financial entries and Ledger adapters never query private space tables.
platform/discord/space_setup.rs composes the shared Http adapter and owns the cancellable bounded
reconciliation worker. scripts/verify-postgres-suites.sh supplies the same nonempty-suite guard to
local disposable DB testing and CI. No new Cargo package/dependency or independent bot is introduced.

Automatic setup amendment: financial storage enables private guild spaces by default. Startup
creates/reuses one private category with a durable backend-only0004 guild/category reservation.
The two location IDs are independent optional overrides. Missing onboarding allowlist permits
starts/recovery in accessible ordinary guild text channels; financial input remains owner-space-only.
Unknown category creation/save or a deleted/hidden binding fails closed; use a verified category
override, never name adoption/blind retry. Shared SDK Http, least privileges and no live-test claims.
