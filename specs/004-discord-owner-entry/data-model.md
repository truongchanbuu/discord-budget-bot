# Discord Entrypoint Data Model

Implemented transient model. No database schema, identity registry, financial resource or persisted permission exists.
[Runtime contract](contracts/runtime.md) owns setting/lifecycle rules; [command contract](contracts/command.md)
owns external mapping, response and registration semantics.

## Configuration and root results

| Concept / planned value | Fields / ownership | Constraints |
| --- | --- | --- |
| RuntimeConfig | Offline or Discord(DiscordConfig), owned at root | Absent mode → Offline; exact offline/discord only; no provider access during parsing |
| DiscordConfig | SecretToken, target-server nonzero value, owned OwnerBinding | All fields required in live mode; no partial default; immutable startup snapshot; aggregate Debug redacted |
| SecretToken | Private owned String | Nonempty UTF-8, no surrounding Unicode whitespace; no format regex, raw Display, serialization or public getter; custom redacted Debug |
| Target server | Private configuration NonZeroU64, converted to GuildId in adapter | Canonical decimal 1–u64::MAX, no padding/sign/leading zero; not an account ID type |
| OwnerBinding | Existing identity application value | Existing DiscordAccountId, OwnerRef and explicit Enabled/Disabled; exact equality/validation unchanged |
| ApplicationError | Public wrapper, private safe category/setting or existing safe StartupError | No raw SDK/provider payload or unsafe source chain; safe Display/Debug; event_name returns a fixed static reporting event |

Config parsing reads supplied Option<OsString> values through a narrow lookup closure in tests;
production lookup uses var_os. Non-text live fields fail; offline mode must never consult them.
Existing logging Config remains responsible for its exact level grammar. start validates runtime
configuration before offline initialization and SDK creation. Configuration errors retain a field
category only; all fields are validated before the first external call.

No new owner representation/validator, credential registry or config service. Only the composition/
adapter boundary can expose the secret to SDK construction; SDK-required ownership/copying is a real
boundary, not permission to clone whole configuration or dump it in diagnostics. No memory-erasure claim.

## Command presentation values

| Concept / planned value | Fields / meaning | Constraints |
| --- | --- | --- |
| CommandData | Owned immutable OwnerBinding, target GuildId, shared accepting flag | Feature presentation owns Poise data; token absent; Debug redacted; flag gates lifecycle only |
| ReadinessDecision | Ready or Denied | Private presentation output from current server/actor mapping and existing resolve_owner; no owner payload/grant |
| ReadinessError | Unavailable | Safe presentation-owned Poise error category, no source payload; setup failure details signaled separately |
| Command input | SDK CommandInteraction from authenticated client | Read interaction invoking user/guild only; no invented client owner claim or financial resource |
| Reply | Actual Poise CreateReply | Fixed content, ephemeral=true, empty allowed mentions; no embeds/attachments or public fallback |
| Registration definition | Actual Serenity CreateCommand converted from Poise command | fundguard, slash/chat input, zero arguments; only named target-guild upsert |

Platform depends on presentation's narrow command/data/error surface and existing application APIs.
Presentation depends only on its application policy and transport framework, never platform startup.
Identity domain/application remain SDK-independent; Financial Core does not gain an identity edge.
No fresh domain/application use-case object is needed just to return static readiness.

## Lifecycle-only state

- Setup outcome: one-shot Ready or safe Failed(category); sender closure before readiness is failure.
  Setup errors must signal failure even if Poise subsequently waits on unavailable user_data.
- Observed connection: Connected or Unavailable mapped from the current single-shard manager snapshot.
  One-second sampling and a monotonic observed-loss deadline; no persisted timestamps/state machine.
- Accepting flag: shared AtomicBool/Arc used only to stop new command work on close/failure. It never
  replaces owner policy, stores a permission or allows an unknown actor.
- Client execution: one tracked task/future with safe terminal result; unexpected completion,
  cancellation or panic maps to failure without formatting JoinError/SDK source payloads.
- Shutdown trigger: real Ctrl-C in production, controlled signal in tests. Clean bounded closure
  returns success; timeout/failure returns a sanitized ApplicationError.

Startup includes client construction, connection and one registration. First readiness clears the
startup watchdog. Later observed connection loss starts a recovery deadline; Connected clears it,
intermediate stages do not. Failures and operator interruption set accepting=false before cleanup.
One shared absolute shutdown deadline covers SDK close, cancellation and joining; no trailing
unbounded await. Details and durations are authoritative in contracts/runtime.md.

No owner/config mutation, hot reload, in-flight permission revocation, command deduplication store,
ledger posting, transaction or bank evidence is introduced. Repeated readiness has no financial effect.
