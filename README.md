# FundGuard

FundGuard is a Discord-first personal finance and payment-awareness application. It aims to make
financial intent visible before payment handoff, while distinguishing virtual budgets from bank evidence.

**Current status:** Offline initialization, pure Financial Core invariants, owner-access policy and
an opt-in Discord readiness adapter are implemented. The one /fundguard command uses the configured
owner binding and replies privately. Local tests use controlled boundaries. Live authentication,
scoped registration and shutdown passed, and the maintainer confirmed a private owner readiness reply.
Other-account/disabled denial and network recovery remain unverified. No fund CRUD, ledger, persistence, payment workflow,
reports, reminders or production deployment exists.

## Install and run

Install rustup using the [official Rust instructions](https://www.rust-lang.org/tools/install).
From the repository root in a POSIX shell:

```bash
rustup show
cargo fetch --locked
FUNDGUARD_MODE=offline cargo run --locked
```

The committed toolchain selects Rust 1.99.1 with rustfmt and Clippy. Cargo.lock fixes dependency
resolution. Downloads need registry access; offline execution and automated tests need no credentials
or running services. The first SDK build is substantially larger than the original offline foundation.

Absent FUNDGUARD_MODE defaults to offline. Exact offline/discord values are supported; invalid modes
fail. Discord settings are ignored offline. FUNDGUARD_LOG_LEVEL accepts trace/debug/info/warn/error
and defaults to info. Diagnostics use stderr, stdout stays empty, and warn/error suppress INFO events.
.env.example is a reference only: .env files are not loaded automatically.

## Optional Discord setup

Use a dedicated developer application/server, bot plus applications.commands installation scopes,
permissions=0 and no privileged gateway intents. Provide all required settings from trusted operator
environment configuration; provision the token through a secure local prompt/secret facility, never
chat, committed files or shell command history. Disabled binding is valid and denies all commands.

The [runtime contract](specs/004-discord-owner-entry/contracts/runtime.md) owns exact setting names,
validation and deadlines. The [validation guide](specs/004-discord-owner-entry/quickstart.md) explains
opt-in startup, owner/second-account checks and cleanup. Only after that local configuration is ready:

```bash
FUNDGUARD_MODE=discord cargo run --locked
```

/fundguard has no arguments. It sends private readiness or generic denial in the configured server;
it exposes no financial data. Registration upserts only this command and preserves unrelated commands.
Configuration changes require restart. Ctrl-C stops the session with bounded cleanup; terminal
failures exit unsuccessfully. Local initialization is distinct from discord_ready.

## Verify

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check
```

Unit tests use real SDK fixtures/builders and controlled I/O futures; executable tests isolate child
environments. They never create a live SDK client or require Discord/PostgreSQL. CI runs locked checks
with minimal permissions and no credentials. Hosted CI and remaining manual denial/recovery checks need separate evidence.
On constrained machines, CARGO_BUILD_JOBS=1 and CARGO_PROFILE_DEV_DEBUG=0/CARGO_PROFILE_TEST_DEBUG=0
reduce build resource use without changing runtime behavior.

## Architecture and development

One Rust package implements a Feature-First Modular Monolith. Domain/application rules remain
independent of Discord and persistence. Identity presentation maps invoking SDK identity into the
existing policy; platform owns client lifecycle and the root selects/configures execution.
Synchronous fundguard::run() stays offline; async fundguard::start() selects the executable runtime.

Financial Core provides exact whole-VND values and pure virtual invariant validation. Candidates
are not journal receipts or bank proof. Identity provides owner resolution and scoped authorization;
future financial use cases still need authoritative resource ownership and fresh access checks.
Dependencies are Tokio, Poise/Serenity, tracing, tracing-subscriber and thiserror. SQLx/PostgreSQL,
serialization and other providers wait for a feature requiring them.

Read [architecture](docs/architecture/overview.md), [structure](docs/development/project-structure.md),
[coding standards](docs/development/coding-standards.md), [security](docs/security/security-baseline.md)
and [AGENTS.md](AGENTS.md). Use Spec Kit: constitution → specify → clarify when needed → plan →
review → tasks → analyze → approved implementation. Keep one authoritative specification per feature,
preserve generated tooling, verify progress and update tasks/handoff rather than relying on chat.

[Active specification](specs/004-discord-owner-entry/spec.md) ·
[Implementation checklist](specs/004-discord-owner-entry/tasks.md) ·
[Feature index](docs/ai/FEATURE_INDEX.md) · [Session handoff](docs/ai/SESSION_HANDOFF.md)

## Roadmap

Next define persistence and atomic financial-commit boundaries, including authoritative owner scope,
audit origins, concurrency and durable idempotency, before payment features. Funds, income, ledger,
payments, reports, reminders and additional interfaces remain future work. QR/OCR/AI, banking and
deployment need separate specifications. The MVP must not directly transfer money or collect bank
passwords, OTPs, PINs, CVVs or banking-session credentials. See the
[product overview](docs/product/overview.md) and [feature map](docs/product/feature-map.md).
