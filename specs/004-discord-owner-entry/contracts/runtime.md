# Runtime and Configuration Contract

Implemented behavior; [tasks.md](../tasks.md) records verification and limits. The specification owns scope.

## Executable and compatibility

The executable awaits a new root async start() → Result<(), ApplicationError> on its existing
current-thread Tokio runtime. The legacy run() → Result<(), StartupError> stays an explicitly offline
library entrypoint; even with Discord environment settings it never starts a client. No nested runtime.

start validates runtime settings, then calls existing offline initialization for logging/local setup,
then either returns (offline) or starts the bounded Discord session. Logging validation happens before
any provider call. application_initialized means local initialization only, never command readiness.

ApplicationError owns only safe categories/static validated setting identifiers or the already safe
StartupError; no raw provider/source data. Display/Debug/source chains are safe. Its event_name()
returns a fixed event name: application_initialization_failed for configuration/local setup failures,
discord_session_failed for Discord lifecycle failures. Main keeps the scoped ERROR fallback and stdout
empty. Preserve existing invalid-log-setting event/message behavior and subscriber collision tests.
Only start and ApplicationError join the existing root public exports; runtime/config/platform remain
private. These types must not expose SDK errors or make credentials public for testing.

## Settings and validation

| Setting | Accepted value / default |
| --- | --- |
| FUNDGUARD_LOG_LEVEL | Existing exact trace/debug/info/warn/error; absent → info; unchanged validation |
| FUNDGUARD_MODE | Absent or exact offline → offline; exact discord → connected mode; everything else, including empty/non-text, fails |
| FUNDGUARD_DISCORD_TOKEN | Required only in discord mode; nonempty UTF-8 with no surrounding Unicode whitespace; no invented provider-token regex |
| FUNDGUARD_DISCORD_GUILD_ID | Required only in discord mode; canonical positive ASCII decimal, 1–20 digits, no leading zero, at most u64::MAX |
| FUNDGUARD_DISCORD_OWNER_ID | Required only in discord mode; existing DiscordAccountId::parse grammar, no normalization |
| FUNDGUARD_OWNER_REF | Required only in discord mode; existing OwnerRef::new semantics, preserving nonblank bytes/case |
| FUNDGUARD_OWNER_STATE | Required only in discord mode; exact enabled or disabled; no inferred default |

Read settings using var_os at the process boundary; pure parsing takes supplied values for tests.
Offline mode must not read or validate Discord-only fields. No .env loading, command-line token,
credential fallback to DISCORD_TOKEN, first-sender registration or file/config editor. Every required
live setting must be valid before SDK initialization; a disabled binding is valid and denies commands.
Server identifiers are not account identifiers: use a private nonzero numeric configuration value,
converted to GuildId only in the adapter, not a misnamed DiscordAccountId reuse.

Errors may name the invalid/missing setting, never its supplied value or provider message. Treat all
configuration aggregates and the owned token wrapper as redacted Debug; no raw Display/serialization.
Only the narrow adapter may borrow the credential for SDK construction. No zeroization guarantee.
.env.example documents offline defaults and commented blank/synthetic live settings only.

## Session and readiness

One client and one shard, GatewayIntents::empty(). Poise defaults disabled; configure
FrameworkBuilder::initialize_owners(false), skip_checks_for_owners=false, custom categorical on_error
and no auto-defer. The builder overwrites the corresponding options field; test the built framework.
No application-owner lookup. Ready-derived application identity supports the HTTP registration call.

The setup closure registers the named command once, sends a payload-free setup result, and returns
its immutable command context. On registration/setup error it sends Failed before returning Err;
on_error never prints the raw FrameworkError or delegates to builtins. A closed setup channel before
success is failure. The supervisor must not infer setup failure from Client::start alone.

Report discord_ready only after gateway connection and successful command registration/setup.
Do not emit it on bare Ready reception, configuration validation or local initialization. No commands
perform use-case work while closing. CommandData shares an accepting flag only for lifecycle gating;
it is not business authorization state or a remembered grant.

## Budgets and terminal outcomes

| Operation | Application policy |
| --- | --- |
| Startup | One logical client-build/start/setup sequence, 30s absolute total deadline beginning before SDK builder awaits |
| Registration | One named upsert, 10s maximum and never beyond remaining startup time |
| Reply | One private send, 2s maximum from handler's immediate dispatch; no app retry or fallback |
| Recovery | Observe shard status every 1s after first readiness; 30s from first observed non-Connected/absent runner to recovery or terminal failure |
| Shutdown | Stop accepting immediately; at most 8s for shutdown_all, at most 10s overall including cancellation/join |

Keep SDK rate limiting enabled. One logical operation can include SDK-managed transport/rate-limit
retries; the stated deadline bounds that future, not the count of wire transmissions. No independent
application reconnect/retry loop or registration on successive Ready/Resumed events.

The supervisor races setup result, task completion, interruption and deadlines. Poll current
shard_manager.runners stage, not potentially reordered event callbacks. Connected clears an active
recovery timer; intermediate stages do not. This measures observed disconnection, not physical outage.
If client returns/exits unexpectedly, setup fails, recovery expires or a task panics/cancels, fail safely.
An unexpected Ok client completion without operator shutdown is also terminal, not successful readiness.

Interrupt sets accepting=false and requests shutdown. On clean close before the total deadline return
success; on cleanup timeout abort/drop remaining tracked work and return safe failure. Do not extend
the total timeout by awaiting a cancelled task without a remaining deadline. Runtime/process teardown
must not leave another reconnect task/session serving commands. Existing in-flight private sends
are bounded; no persistent jobs or financial state need draining.

## Diagnostics and testability

Allow tracing only when target equals fundguard or begins fundguard::, at the configured level;
all SDK/transport targets are silent at every level. Same target allowlist applies to fallback.
Static categories cover configuration, setup/registration, connection/recovery, denial, response and
shutdown; no IDs, token, raw interaction, SDK error Display/Debug or config dumps are fields.

Private lifecycle tests use synthetic setup/status/interrupt signals and task results, paused Tokio
time and no real SDK client. Production sampling maps current SDK status to a small internal state;
tests feed that same supervisor. Test setup Err + pending client explicitly, not just returned client
Err. Closing status channels/missing runner must not accidentally report readiness. No test connector
is selected through runtime environment or exposed publicly solely for integration tests.
