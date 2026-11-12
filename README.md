# FundGuard

FundGuard is a Discord-first personal finance and payment-awareness application. The vision is to
make financial intent visible before payment handoff while keeping virtual budgets distinct from
verified bank information.

**Current status:** Phase 0 foundation is complete and locally verified. The executable validates
logging configuration, initializes diagnostics, and exits. It makes no Discord/database calls.
The library also provides Financial Core values, pure invariant validation and single-owner access
policy. Live authentication and resource authorization integration are not connected. No fund CRUD, ledger,
payment workflow, reports, reminders, persistence, or production deployment exists.

## Install and run

Install rustup using the [official Rust instructions](https://www.rust-lang.org/tools/install),
then run from the repository root in a POSIX shell:

```bash
rustup show
cargo fetch --locked
cargo run --locked
```

The committed toolchain selects Rust 1.99.1, rustfmt, and Clippy; first use can download them.
Cargo.lock fixes dependency resolution. No Discord token, PostgreSQL server, banking access, or
production credentials are needed. The application itself and tests make no service calls.

Optional configuration:

```bash
FUNDGUARD_LOG_LEVEL=debug cargo run --locked
```

Exact values: trace, debug, info, warn, error. Default: info. `.env.example` documents this single
non-secret setting; `.env` is not loaded automatically. Startup events go to stderr, stdout stays
empty, and warn/error suppress the INFO success event. Invalid settings fail with a structured,
redacted ERROR event. See the [runtime contract](specs/001-project-foundation/contracts/runtime.md).

## Verify

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check
```

Unit and executable tests run without real credentials or services. Child processes have bounded
waits, drained output, and isolated environments. CI runs the same checks with locked dependency
resolution. The workflow is configured; hosted CI is unverified until a real run is available.
For success/failure scenarios, see the [validation guide](specs/001-project-foundation/quickstart.md).

## Architecture and development

One Rust package implements a Feature-First Modular Monolith with Clean Architecture boundaries.
Meaningful startup/configuration, Financial Core and identity modules exist. Future business logic stays independent of
Discord/persistence so other bots, Web, and mobile interfaces can reuse it.

Financial Core exposes exact whole-VND values, nonblank typed references, no-overdraft previews,
and owner-scoped new/replay/conflict validation through features::financial_core. It uses only std
and existing thiserror. Results are virtual candidates, not journal receipts or bank proof.
See the [Financial Core contract](specs/002-financial-core/contracts/financial-core.md).

Identity resolves an explicit enabled account-to-owner binding and authorizes matching owner scope
through features::identity. Unknown/missing actors, disabled/missing bindings and foreign/missing
scope deny access. Callers must supply authenticated identity and authoritative ownership; local
policy is not provider authentication or a reusable permission grant. See the
[owner-access contract](specs/003-owner-access/contracts/owner-access.md) and
[validation guide](specs/003-owner-access/quickstart.md).

Current dependencies are Tokio, tracing, tracing-subscriber, and thiserror. Future integrations
select Poise/Serenity, PostgreSQL/SQLx, and Serde only when required; no unused providers are added.
See [architecture](docs/architecture/overview.md), [structure](docs/development/project-structure.md),
[coding standards](docs/development/coding-standards.md), and [security](docs/security/security-baseline.md).

Use the installed Spec Kit skills: constitution → specify → clarify when necessary → plan →
review → tasks → analyze → approved implementation. Preserve generated tooling and keep one
authoritative spec per feature. Review the constitution and scope before editing, run focused
checks, inspect the diff, and update verified task progress and handoff. Start sessions with
[AGENTS.md](AGENTS.md); do not rely only on chat history.

[Active specification](specs/003-owner-access/spec.md) ·
[Implementation checklist](specs/003-owner-access/tasks.md) ·
[Feature index](docs/ai/FEATURE_INDEX.md) · [Session handoff](docs/ai/SESSION_HANDOFF.md)

## Roadmap

Financial Core and pure owner-access policy are implemented in isolation. The next specification
should define trusted binding loading and a first authenticated interface, or persistence and atomic
financial-commit boundaries, before payment features. Live identity integration, funds, income,
ledger, payments, reporting and reminders remain future work. QR/OCR/AI, banking, payment handoff, financial migrations, and deployment remain outside
Phase 0. The MVP must not execute direct real-money transfers or collect bank passwords, OTPs,
PINs, CVVs, or banking-session credentials. See the [product overview](docs/product/overview.md)
and [feature map](docs/product/feature-map.md) for planned ownership and sequencing.
