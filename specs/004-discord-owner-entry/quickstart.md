# Discord Entrypoint Validation Guide

**Status**: Planned guide. Feature 004 is not implemented; no new application passing-test claim.

## Prerequisites

Use existing Rust 1.99.1/Edition 2024 and rustfmt/Clippy. During authorized implementation, add the
reviewed SDK/features and commit the resolved Cargo.lock described in [research](research.md).
Dependency downloads may use the network; automated tests must not contact Discord or a database.
Read [runtime](contracts/runtime.md) and [command](contracts/command.md) contracts for exact inputs.

## Safe baseline and automated checks

Preserve the actual current Git state before edits. This planning session began with a clean tree;
previously committed instruction/source edits must not be reverted. Follow [tasks.md](tasks.md)
only after implementation authorization.
After implementation, from repository root:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --test discord_entry
cargo test
cargo check
FUNDGUARD_MODE=offline cargo run --locked
```

The discord_entry target is planned, not present. It covers real executable offline/invalid inputs;
private config/mapper/registration/supervisor unit tests run under the full suite. The default run
must initialize offline, log safely and exit without credentials/provider calls. Existing CI keeps
locked checks and no credentials. Prior 37-test evidence is historical, not proof of this new code.

Automated scenario groups:

1. Absent/offline mode ignores every malformed/ambient Discord-only input. Explicit live mode
   rejects each missing/non-text/invalid required field before a fake connector can be called.
   Unsupported mode and invalid log level fail safely; preserve existing child env_clear isolation.
2. Valid synthetic live configuration selects exactly one controlled connector; enabled/disabled
   state preserves existing binding semantics. No real SDK client may be constructed by this test.
3. Construct actual SDK interaction fixtures with forged display names and distinct invoking IDs;
   invoke the production mapper and existing policy for owner/nonowner, disabled, wrong/missing
   server and missing actor cases. Serialize the real reply builder; assert private flags/text.
4. Capture named upsert scope/builder and finite attempts; preserve unrelated commands and register
   once across reconnect signals. Review production SDK method selection, not just fake call counts.
5. With paused Tokio time, test startup builder/setup deadlines, setup Err while client remains
   pending, registration timeout, response error/timeout with no fallback, observed-loss recovery,
   unexpected task completion and bounded interrupt cleanup. Do not wait real 30s in ordinary tests.
6. At every configured log level, inject synthetic sensitive SDK/transport target events and provider
   errors. Verify both primary/fallback subscribers suppress them and application categories remain
   useful. Test nested credential/config/error formatting without logging real identities or tokens.

This is controlled-boundary verification, not a real Discord end-to-end/authentication certificate.

## Optional live smoke after implementation

Use a dedicated developer-owned Discord application and server. Installation permissions/scopes
are in the command contract: bot plus applications.commands, permissions=0, privileged intents off.
Do not configure a public interactions endpoint; this design receives interactions through Gateway.
Use an existing voluntary second test account for denial checks; no production accounts are required.

1. Review source/lockfile and automated evidence first. Do not start live mode during ordinary CI.
2. Provision the bot token through a secure local prompt/secret facility into the process environment.
   Do not paste it into chat, shell command text/history, documentation or a committed file.
3. Set the target-server, owner-account, owner-reference and explicit binding state locally using
   the exact setting names in the runtime contract. No .env file is automatically loaded.
4. Start only this command, once the reviewed configuration is present:

```bash
FUNDGUARD_MODE=discord cargo run --locked
```

5. Confirm local initialization is distinct from discord_ready after successful registration. Check
   that unrelated commands remain and /fundguard has no parameters or global registration.
6. Invoke as the owner: private fixed readiness, no financial operations or identifiers. Invoke as
   the other account: only private generic denial. Verify no public channel/DM fallback.
7. Interrupt with Ctrl-C: process stops within ten seconds; do not call a forced timeout graceful.
   Restart with disabled binding and verify former owner denial, then stop and clear local secrets.

Record pass/fail/unverified separately for registration, owner reply, nonowner denial, privacy and
shutdown. Do not retain tokens, account/server/owner IDs, raw interaction payloads or identifying
screenshots as repository evidence. A separate temporary unrelated command used for preservation
checks must be operator-owned and cleaned up narrowly; never bulk-delete command lists.

If credentials/access are unavailable, mark live smoke unverified and still complete honest automated
work. Live failure is a real known issue to investigate, not something that simulated tests override.
Hosted CI status also needs an actual run. No deployment or financial/banking operation is authorized.
