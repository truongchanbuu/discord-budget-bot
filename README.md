# FundGuard

FundGuard is a Discord-first personal finance and payment-awareness application. It aims to make
financial intent visible before payment handoff, while distinguishing virtual budgets from bank evidence.

**Current status:** Offline initialization, pure Financial Core, owner policy, guild readiness and
an optional PostgreSQL-backed private finance journal are implemented. Finance DMs support reviewed
income/expenses, categories, fund initialization/transfers and monthly totals. Prior readiness live
smoke passed; the new financial DM flow and hosted CI remain unverified live. No bank integration,
real transfers, AI, portfolio valuation, recurring salary or production deployment.

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

## Private finance setup and use

Finance is optional. Without FUNDGUARD_FINANCE_DATABASE_URL, the existing readiness-only bot works.
Use a dedicated **Supabase development project** for managed PostgreSQL; no self-hosted database is
required to run the bot. In Supabase, disable Data API before first startup, then copy a Direct or
Session pooler connection string from **Connect** (port5432). Use Session mode on IPv4-only networks;
Transaction pooler6543 is unsupported. Add `sslmode=require`; stronger certificate verification is
available. See the [Supabase setup guide](specs/005-conversational-finance/quickstart.md#supabase-setup).

Set FUNDGUARD_FINANCE_DATABASE_URL to that project's PostgreSQL connection URL in your trusted local
environment. Keep credentials in your ignored local configuration; never paste a real URL/token into
chat or shell history. The application does not load .env.local automatically. In Bash/Zsh, load your
own trusted file and start:

```bash
set -a
source .env.local
set +a
FUNDGUARD_MODE=discord cargo run --locked
```

Finance enables only the DIRECT_MESSAGES intent; leave privileged Message Content off. Open the bot's
profile and send a DM (server privacy settings must allow DMs). Server financial messages are ignored.
Only the configured owner can access the journal. Try:

```text
trợ giúp
quỹ
số dư đầu kỳ 200k
tôi vừa donate 100k cho vtuber A
nhận lương 15tr
tiết kiệm 3tr
đầu tư 2tr
tạo quỹ Du lịch
tổng kết 2026-11
```

Each financial message creates a preview; only **Lưu** posts it. **Sửa** opens amount/category/fund-route/
date/note fields; modal submission creates a revised preview, then Lưu. **Hủy** changes no money.
Missing/multiple/invalid amounts need correction. Use integer VND, k/nghìn/ngàn/tr/triệu, no fractions
or conversion. Optional `ngày YYYY-MM-DD` supports historical dates; future received amounts are rejected.
For another source/destination, edit route to `chi tiêu -> tiết kiệm` or another owned fund. Explicit
natural “từ … sang …” routes require review/correction; this is finite parsing, not unrestricted AI.

Defaults: Chi tiêu, Tiết kiệm, Đầu tư, all zero. Initialize funds or record received income before
spending; no overdraft, including backdated history. Savings/investment transfers preserve total
principal and are not consumption. Opening is initial/carryover, not salary. Monthly summaries include
opening/initialization, received income, consumption, reserve inflows, total/spending closing and
category totals. Percentages use integer floor, unavailable with zero income. Long views show at most
12 funds/10 categories and explicitly disclose omitted rows; aggregate totals still include all.
All values are virtual/self-reported, never bank verification. No committed edits/deletion in this slice.

Drafts survive restart and expire after15min. Duplicate confirmation returns the same immutable receipt.
A failed Discord receipt does not undo/repeat a committed financial effect; retry the same draft.
Detailed limits/setup/validation: [feature guide](specs/005-conversational-finance/quickstart.md).

## Verify

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check
```

Ordinary tests need no credentials/provider connections. Persistence tests are explicitly ignored
there and must also run against disposable PostgreSQL:

```bash
scripts/test-postgres.sh
```

Requires PostgreSQL server tools plus createdb on PATH. If server tools are elsewhere, set
FUNDGUARD_TEST_PG_BIN; the script creates/stops its own private socket-only cluster with synthetic
fixtures, overwriting only its test URL. It never uses your application URL or existing database.
Local PostgreSQL is needed only for this isolated DB test command, not ordinary bot use.
CI has separate normal gates and a synthetic PostgreSQL integration job; hosted results unverified.
CARGO_BUILD_JOBS=1 and CARGO_PROFILE_DEV_DEBUG=0/CARGO_PROFILE_TEST_DEBUG=0 reduce SDK build resources.

## Architecture and development

One Rust package, feature-first modular monolith. Financial Core and identity stay provider-independent.
The ledger feature owns this first personal-journal slice: pure parse/posting/report rules, one real
persistence port, PostgreSQL adapter, and concrete DM presentation. FinanceEvents sits alongside Poise;
identity has no ledger dependency. Platform owns client/pool lifecycle; root composes optional finance.

Owner-row serialization, immutable entries, unique source, revisions and whole/range checks protect
atomic idempotency and historical nonnegative balances. No float money or mutable balance cache.
Read [ADR0006](docs/architecture/decisions/0006-conversational-journal.md),
[architecture](docs/architecture/overview.md), [coding standards](docs/development/coding-standards.md),
[security](docs/security/security-baseline.md) and [AGENTS.md](AGENTS.md).
Use Spec Kit: specify → plan → tasks → analyze → authorized implementation, with clarification/review
as needed. Requirements, detailed progress and current state live in spec/tasks/handoff respectively.

[Active specification — planned private channels](specs/006-private-finance-channels/spec.md) ·
[Delivered journal tasks](specs/005-conversational-finance/tasks.md) · [Feature index](docs/ai/FEATURE_INDEX.md) ·
[Handoff](docs/ai/SESSION_HANDOFF.md)

## Roadmap

Future specifications may add immutable correction/reversal, account/category administration,
recurring income, richer reporting and other interfaces. Payment awareness/handoff needs separate
invariants before any integration. No banking passwords, OTPs, PINs, CVVs or session credentials;
no direct MVP real-money transfers. See the [feature map](docs/product/feature-map.md).
