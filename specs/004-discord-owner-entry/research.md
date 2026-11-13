# Discord Entrypoint Research and Decisions

Date: 2026-11-13. Planning only; no product code or repository dependency changes.

## 1. Maintained dependencies and compatibility

**Decision**: Propose Poise 0.7.1 and Serenity 0.12.5 with defaults disabled, Serenity rustls_backend
only explicitly added. Keep existing Rust 1.99.1/Edition 2024 and dependency versions; extend Tokio
1.54.1 with signal/sync/time alongside macros/rt. Add Tokio test-util to dev dependencies for paused
clock tests. No new direct HTTP, Serde, configuration, secret-management or database crate.

Proposed production additions/settings during implementation:

```toml
poise = { version = "0.7.1", default-features = false }
serenity = { version = "0.12.5", default-features = false, features = ["rustls_backend"] }
tokio = { version = "1.54.1", default-features = false, features = ["macros", "rt", "signal", "sync", "time"] }
```

**Rationale**: Poise requires Serenity ^0.12.5 and already enables its builder/client/gateway/model/
utils/collector/framework support. Direct Serenity enables the required TLS backend without SDK
cache, chrono, standard framework or Poise panic-handling defaults. Networking is enabled transitively;
no rt-multi-thread or Tokio full is needed for the one-client current-thread runtime.
[Poise manifest](https://docs.rs/crate/poise/0.7.1/source/Cargo.toml),
[Serenity manifest](https://docs.rs/crate/serenity/0.12.5/source/Cargo.toml).

**Evidence**: An isolated /tmp dependency/framework-builder probe resolved 213 dependency packages
and passed cargo check -j1 on the pinned compiler in 1m39s, without warnings or provider calls.
Its exact-release probe exercised initialize_owners=false and skip_checks_for_owners=false.
This verifies dependency/build compatibility only, not FundGuard integration or a vulnerability audit.
Implementation must commit its own reviewed Cargo.lock; transitive SDK dependencies are justified
by this real integration, not permission to use their frameworks in business code.

**Alternatives considered**: Serenity alone discards the preferred command framework without a
current benefit; SDK defaults enable unnecessary capabilities; optional Cargo flags add a second
build mode before need. The runtime is opt-in, but the ordinary build includes the adapter.

## 2. Minimal event access and authenticated identity

**Decision**: GatewayIntents::empty(), one shard via Client::start, no member/cache/message handlers.
Extract invoking user and guild from the actual application-command interaction. Disable Poise
initialize_owners and skip_checks_for_owners; never use owners_only as FundGuard authorization.

**Rationale**: InteractionCreate does not require gateway intents. Ready supplies application-ID
information used by Serenity HTTP; no extra application-ID configuration or owner-discovery lookup
is needed. The existing owner policy is authoritative; installation/application owners are different.
[SDK event definitions](https://docs.rs/crate/serenity/0.12.5/source/src/model/event.rs),
[framework options](https://docs.rs/crate/poise/0.7.1/source/src/structs/framework_options.rs).

**Alternatives considered**: non_privileged intents are broader than needed; role/admin/server
membership checks do not bind the configured account; public client claims cannot authenticate actors.

## 3. Registration without collateral changes

**Decision**: Build one slash definition using Command::create_as_slash_command, then call configured
GuildId::create_command once in successful setup. Treat missing definition as a safe setup error.
No global registration, list-and-replace, cleanup command or registration on every reconnect.

**Rationale**: Discord's create operation upserts the same name/type/scope. Poise register_in_guild
uses bulk set_commands and would remove unrelated commands. Setup is consumed once per framework;
process restart can repeat named upsert without duplicate creation.
[Discord command semantics](https://docs.discord.com/developers/interactions/application-commands#updating-and-deleting-a-command),
[GuildId methods](https://docs.rs/crate/serenity/0.12.5/source/src/model/guild/guild_id.rs),
[Poise registration helpers](https://docs.rs/crate/poise/0.7.1/source/src/builtins/mod.rs).

**Alternatives considered**: Bulk replacement violates FR-010; a registration/admin command increases
privilege and surface area; global installation/commands exceed the one-server scope.

## 4. Private responses and framework errors

**Decision**: Slash-only /fundguard, explicit command ephemerality and CreateReply::ephemeral(true).
No auto-defer, prefix handler, public defer, follow-up fallback or default error handler. The handler
maps fixed readiness/denial text and invokes one bounded private-send operation. on_error logs only
safe framework categories and never delegates to builtins or retries a possibly completed reply.

**Rationale**: Poise default errors can print raw data and send error text to users. CreateReply fields
are private: test the actual serialized to_slash_initial_response builder via serenity::json::to_value,
not invented field getters. SDK from_value also allows synthetic real CommandInteraction fixtures
without direct serde_json dependency. These tests must execute the production mapper/reply builder.
[Reply builder](https://docs.rs/crate/poise/0.7.1/source/src/reply/builder.rs),
[default errors](https://docs.rs/crate/poise/0.7.1/source/src/builtins/mod.rs),
[SDK JSON helpers](https://docs.rs/crate/serenity/0.12.5/source/src/json.rs).

**Alternatives considered**: Command ephemerality alone is too implicit; default ctx.say/error output
is unsuitable; a mock parallel dispatcher could pass while production mapping remains wrong.

## 5. Setup failure and finite lifecycle bounds

**Decision**: An outer supervisor owns client execution, setup-result signaling, interruption,
startup/recovery timers and shutdown. Setup sends Ready/Failed through a one-shot channel, independent
of Poise user_data. Registration precedes readiness; setup errors also signal failure before returning.

**Rationale**: Poise setup Err invokes on_error but does not stop Client::start; user_data may remain
unset and event dispatch wait indefinitely. ClientBuilder also performs gateway discovery, so only
timing its subsequent start call would leave an unbounded path.
[Framework source](https://docs.rs/crate/poise/0.7.1/source/src/framework/mod.rs),
[ClientBuilder source](https://docs.rs/crate/serenity/0.12.5/source/src/client/mod.rs).

One logical builder/startup attempt: total 30s; one named upsert: 10s within that budget; one reply:
2s within the provider's 3s initial-response window. No application retry. SDK 429 handling may
retry internally; preserve its rate limiter and cancel the operation at the absolute deadline.
This is not a one-wire-request guarantee.
[SDK rate limiting](https://docs.rs/crate/serenity/0.12.5/source/src/http/ratelimiting.rs),
[Discord response window](https://docs.discord.com/developers/interactions/receiving-and-responding#followup-messages).

After first readiness, sample the single shard manager's current runners/stage once per second.
Absent/non-Connected status starts a 30s recovery deadline; actual Connected recovery clears it,
intermediate stages do not. Use an observed-loss bound, not a physical-outage detection guarantee.
Stage callbacks can reorder and Poise's event callback does not receive every bridge-stage update.
[Runner info](https://docs.rs/crate/serenity/0.12.5/source/src/gateway/bridge/mod.rs),
[stage dispatch](https://docs.rs/crate/serenity/0.12.5/source/src/gateway/bridge/shard_runner.rs).

On interrupt/failure set accepting=false, bound shutdown_all to 8s and the entire close/cancel/join
path to 10s. Never await cleanup without the remaining deadline. A clean interrupt succeeds; timeout
or provider/task failure exits unsuccessfully. No outer restart loop or persistent state machine.
[SDK shard manager](https://docs.rs/crate/serenity/0.12.5/source/src/gateway/bridge/shard_manager.rs).

## 6. Logging and configuration protection

**Decision**: Only reviewed targets fundguard and fundguard::… may reach tracing output, at the
configured level. Apply this allowlist to global and fallback subscribers, even at trace/debug.
Keep custom redacted configuration/credential Debug, payload-free errors and no raw provider sources.

**Rationale**: SDK request instrumentation and warnings can carry routes, interactions or provider
errors. Sanitizing application errors alone is insufficient; tracing filtering alone also cannot
stop Poise default direct stderr output, hence both controls are required. Existing subscriber std/fmt
features already provide registry/filter_fn; no environment-filter/regex crate is needed.
[SDK HTTP instrumentation](https://docs.rs/crate/serenity/0.12.5/source/src/http/client.rs),
[subscriber filter](https://docs.rs/tracing-subscriber/0.3.23/tracing_subscriber/filter/fn.filter_fn.html).

**Alternatives considered**: Substring replacement/redaction after formatting misses other secrets;
allowing SDK warnings or ambient RUST_LOG undermines a reproducible privacy boundary. No secure
memory-erasure or full dependency-security claim is made.

## 7. Composition and test seams

**Decision**: Preserve synchronous fundguard::run() as explicitly offline compatibility. Add async
start() for executable runtime selection, without a nested runtime. Keep existing business APIs
unchanged and put the real command in identity presentation, lifecycle in platform, wiring in root.
Use small private FnOnce/future seams for actual send/upsert boundaries and concrete supervisor
channels/tasks for fake readiness/disconnect/shutdown. No provider registry or generic service stack.

**Rationale**: These are real I/O/testing boundaries; pure policy remains synchronous. Paused-time
unit tests and captured SDK builders verify deadlines/privacy without sockets. Binary tests cover
default/invalid config only; no test-only environment switch may activate a fake production connector.

**Alternatives considered**: Making run async breaks the existing API; environment mutation in parent
tests is unsafe; a second executable or HTTP server adds lifecycle and security work outside scope.

All planning unknowns are resolved. Limits and SDK-specific caveats are explicit contract inputs;
manual live smoke and hosted CI remain unverified until separately executed.

## Implementation findings (2026-11-13)

The real-framework regression test caught FrameworkBuilder overriding options.initialize_owners.
Explicitly call FrameworkBuilder::initialize_owners(false), then assert the final built options.
Serenity Context cannot be safely constructed without runner context under these SDK features.
Offline coverage therefore uses real command-table assertions plus published dispatch/error-path
review; it does not invent a parallel dispatcher or construct a live client. See ADR 0005.
