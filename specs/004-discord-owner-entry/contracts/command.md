# Private Owner Readiness Command Contract

Planned Discord adapter surface; no financial/resource operation or provider verification is delivered
by writing this contract. Existing identity and Financial Core public contracts remain unchanged.

## Command definition and registration

- Name: fundguard; type: slash/chat input; description: Check FundGuard readiness.
- Zero parameters, subcommands, prefix/context-menu handlers, buttons or registration commands.
- Register only in FUNDGUARD_DISCORD_GUILD_ID; no global command or user-install/DM entrypoint.
- Command ephemerality is enabled and each reply explicitly sets ephemeral(true).
- No owners_only, guild-only precheck bypass or application-owner lookup; handler itself denies
  missing/wrong server and resolves the operator-bound actor uniformly.

Poise Command::create_as_slash_command produces one CreateCommand. Configured GuildId::create_command
performs the named upsert in setup. Do not call register_globally, register_in_guild, set_commands or
bulk replacement. Preserve other names/types, including unrelated commands of the same application.
No command deletion on shutdown. Restart repeats a safe upsert; reconnect does not register again.

Guild installation is operator-controlled, using bot and applications.commands scopes with no bot
permission bits (permissions=0). The installing human needs permission to install an app; that is not
a reason to grant the bot Administrator. No privileged intents or message/member/presence subscriptions.
Document server/channel application-command access separately: install/visibility never grants
FundGuard ownership. Use a dedicated developer application/server; do not clear existing global
commands automatically if an operator previously configured them elsewhere.

## Authentic input mapping and authorization

Only a genuine application-command interaction routed by the authenticated SDK session reaches the
production command. Extract invoking account from the interaction user (Poise ctx.author().id), guild
from interaction guild_id (ctx.guild_id()). Do not use ready/application owner, member roles, display
name, mention, command content or a submitted OwnerRef as identity evidence. No resource owner exists.

A private mapper used by the production handler validates missing/invalid account and missing/foreign
guild as Denied. On a matching server, call identity::resolve_owner with the startup-loaded binding
for each invocation. Enabled exact account yields Ready, all other cases Denied. The mapper is
presentation code translating external input, not a duplicate business policy or new application layer.

No financial validation, fake scope equal to a caller's owner, storage lookup, remembered grant or
multi-owner table. A new process loads fresh operator configuration; changes require restart.
Future resource operations must separately obtain authoritative resource scope and authorize it.

## Fixed response and failure policy

| Result | Exact content | Visibility |
| --- | --- | --- |
| Ready | FundGuard is ready. No financial operations are available. | Ephemeral |
| Denied | Access denied. | Ephemeral |

Only constant text, no IDs/configuration/version/token/financial data, embeds, attachments or pings.
Set allowed mentions to empty as a defense even though text has no mentions. Send immediately, with
one 2s timeout budget and no automatic defer. All valid responses initiate within the provider's 3s
window in controlled tests; transport/server delivery timing is not guaranteed by local tests.

A failed/timed-out/ambiguous send is one safe response-failure category. Do not retry, switch to public,
send a DM, create a followup, or emit a second error reply. Duplicate event delivery has no state or
registration effect; a duplicate-ack provider error is sanitized without another send attempt.
Unsupported command names never dispatch owner policy. On framework errors, custom on_error emits
only safe static categories, never delegates/prints raw errors or sends default error text. Valid
command policy denials are handled by the command's private reply path, not framework owner checks.

## Adapter data and test seams

Feature-owned presentation CommandData contains immutable binding, target guild and a shared accepting
flag. Its Debug is redacted. Platform creates the framework with the command supplied by presentation;
presentation depends on existing application APIs and SDK types, never platform client/lifecycle.
A small presentation-owned ReadinessError::Unavailable is sufficient for Poise's error parameter;
setup signals its detailed safe lifecycle category separately rather than retaining raw SDK errors.
The command produces this error only after a private send failure; on_error cannot send again.

Use actual SDK CommandInteraction fixtures through serenity::json::from_value to exercise the same
production field mapping. Include a forged display name/content and a mismatched real invoking ID.
Test captured CreateReply by converting to_slash_initial_response(CreateInteractionResponseMessage::new())
and serializing through serenity::json::to_value: verify the ephemeral flag, constant text and empty
mentions. Do not reach into private Poise builder fields or assert on Debug of SDK payloads.

Use one narrow private FnOnce/future seam around the real send operation to count invocations,
capture the real builder and test timeout/error behavior. Registration may similarly inject one
upsert future accepting the real guild/command builder; verify scope/name/zero args, call count,
timeout and error propagation. Production wiring must use the actual SDK named method, not a
parallel fake dispatcher. No provider service hierarchy, production fake mode or new mock dependency.

## Acceptance verification map

| AC | Planned evidence |
| --- | --- |
| AC-01–04 | Pure environment parser plus real child executable offline/invalid cases; fake connector called only after complete explicit live configuration |
| AC-05–08 | Real SDK input mapping, existing owner resolution, disabled/foreign/missing cases, spoofed fields; captured constant private reply |
| AC-09 | One send on error/timeout; captured private payload, no alternative send/DM/followup path |
| AC-10 | Actual slash builder and named-upsert wiring, preserve unrelated commands, setup once across reconnect signals |
| AC-11 | Sanitized provider-error markers, setup failure signal with pending client, config/build/registration failures never ready |
| AC-12 | Observed status/recovery clocks, SDK/client terminal outcomes, no re-registration or bypass |
| AC-13 | Accepting gate, interrupt/cleanup deadlines, client/task cancellation and process result |
| AC-14 | All deterministic cases offline; four Cargo gates and retained prior suites; manual live smoke reported separately |

Tests establish adapter logic/wiring under controlled boundaries, not that Discord authenticated a
real account or delivered a private message. Only an actual opt-in live smoke can support those claims.
