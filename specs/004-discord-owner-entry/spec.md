# Feature Specification: Trusted Discord Owner Entrypoint

**Feature Branch**: `004-discord-owner-entry` (feature identifier; actual Git branch remains `001-project-foundation`)

**Created**: 2026-11-12

**Status**: Draft for maintainer review. Specification only; no plan, tasks or implementation.

**Input**: User description: "$speckit-specify next faeture pls".
The completed owner-access feature recommends trusted owner configuration and a minimal authenticated
interface next. This increment connects that policy to one explicitly enabled local Discord bot
and one private readiness command, without adding financial operations.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Starting Only the Intended Runtime (Priority: P1)

As the operator, I want offline startup by default and explicit validated configuration for a local
Discord session, so credentials alone cannot accidentally connect the application or enable access.

**Why this priority**: External access must be deliberate, and the trusted binding must exist before
receiving private commands. Existing credential-free development must keep working.

**Independent Test**: Evaluate startup choices with synthetic configuration and an observed external
boundary; invalid/default cases must make zero Discord calls and need no real account.

**Acceptance Scenarios**:

1. **AC-01 — Safe default**: **Given** no runtime selection or explicit offline mode, **When** the
   executable starts, **Then** it performs the existing offline initialization and exits successfully
   with valid logging configuration, even if Discord settings are present, absent or malformed.
2. **AC-02 — Complete live configuration**: **Given** explicit Discord mode with a bot credential,
   one target server and a valid complete owner binding, **When** startup configuration is evaluated,
   **Then** it enables one Discord session using only those trusted settings.
3. **AC-03 — Incomplete configuration**: **Given** explicit Discord mode with any missing/invalid
   required value, **When** startup is attempted, **Then** it fails before any Discord connection or
   registration, with a safe setting/category error and no partial/default owner binding.
4. **AC-04 — Invalid mode**: **Given** an unsupported runtime selection, **When** startup is attempted,
   **Then** it fails without external calls or silently falling back to another mode.

---

### User Story 2 - Checking Readiness as the Configured Owner (Priority: P1)

As the owner, I want one slash command to confirm that FundGuard recognizes me, with a reply visible
only to me and no financial data or account identifiers exposed.

**Why this priority**: This is the first real interface use of the existing owner policy; its value
is a verified access boundary before introducing private financial use cases.

**Independent Test**: Supply synthetic provider-interaction data and a captured response boundary;
verify identity extraction, server restriction, policy outcomes and private responses without Discord.

**Acceptance Scenarios**:

1. **AC-05 — Owner readiness**: **Given** an authenticated command interaction from the bound account
   in the configured server and an enabled binding, **When** /fundguard is invoked, **Then** the owner
   receives a private readiness reply stating that no financial operations are available.
2. **AC-06 — Other account**: **Given** the same server and binding, **When** another authenticated
   account invokes /fundguard, **Then** it receives only a private generic denial, without owner,
   binding, configuration, financial or readiness details.
3. **AC-07 — Spoofed ownership**: **Given** an unknown sender whose name, mention or submitted content
   resembles the configured owner, **When** the command is processed, **Then** authorization uses
   the provider-authenticated invoking account and denies access regardless of that content.
4. **AC-08 — Disabled or out-of-context access**: **Given** a disabled binding or an interaction
   outside the configured server, including a direct message or missing server context, **When**
   /fundguard is processed, **Then** it yields the same generic denial where a private reply is
   possible; no server membership, administrator role or previous success grants owner access.
5. **AC-09 — Private response failure**: **Given** any permitted or denied interaction, **When** the
   private response cannot be delivered, **Then** the application records a safe failure category
   and never falls back to a public message, unsolicited direct message or identity disclosure.

---

### User Story 3 - Operating the Small Integration Safely (Priority: P2)

As the operator, I want narrowly scoped command registration, safe lifecycle handling and repeatable
credential-free checks, so a local bot session can be reviewed without production infrastructure.

**Why this priority**: Provider failures and registration/shutdown behavior must not weaken privacy
or turn a minimal entrypoint into uncontrolled external behavior.

**Independent Test**: Exercise registration intent, failures and shutdown through controlled local
boundaries; inspect emitted diagnostics and command scope with synthetic credentials and identities.

**Acceptance Scenarios**:

1. **AC-10 — Scoped registration**: **Given** complete Discord-mode configuration, **When** commands
   are registered or a session reconnects, **Then** only /fundguard is created/updated in the one
   configured server, without global registration, duplicate commands or deletion of unrelated ones.
2. **AC-11 — Terminal startup failure**: **Given** rejected credentials, denied registration or a
   terminal connection/setup failure, **When** startup cannot complete, **Then** the executable exits
   unsuccessfully with a sanitized category and no token, account/server/owner identifier, raw
   interaction, provider body or false readiness-success event.
3. **AC-12 — Session interruption**: **Given** a running session, **When** connectivity is interrupted,
   **Then** command success is not fabricated, retries do not duplicate commands or bypass access
   checks, and a terminal session failure is reported safely with an unsuccessful process result.
4. **AC-13 — Local shutdown**: **Given** a running local session, **When** the operator interrupts it,
   **Then** the application stops accepting work and closes its session within ten seconds under
   controlled test conditions, without leaking credentials or claiming financial work was performed.
5. **AC-14 — Repeatable verification**: **Given** a checkout without Discord credentials or live
   services, **When** automated verification runs, **Then** configuration, mapping, policy, private
   response selection, failure and lifecycle cases pass without opening real provider connections,
   while existing offline, Financial Core and owner-access checks remain intact.

### Edge Cases

- Ambient Discord credentials must not switch offline mode into a connected session. Invalid
  logging configuration retains its existing safe startup failure behavior in either mode.
- In Discord mode, empty, whitespace-only, padded or non-text credential input is invalid. Local
  validation must not invent a credential format or claim that nonblank text authenticates a bot.
- Missing binding fields never default to the first sender. Account format and exact owner-reference
  semantics reuse the completed identity/Financial Core contracts; no implicit normalization.
- A disabled binding is valid configuration: the session may start, but every private command denies.
- Binding changes take effect only after restarting the local process; hot reload and in-flight
  revocation are excluded. A new process must not reuse a remembered authorization result.
- Readiness has no financial resource to look up: do not fabricate a resource owner or treat a
  request-supplied owner as authoritative scope to demonstrate authorization.
- Missing/malformed invoking-account context fails closed. Unsupported command names do not dispatch
  owner use cases. Duplicate delivery must not create state, public replies or extra registrations.
- A registration or response error must not be formatted directly into logs; provider errors can
  retain tokens, identifiers and raw response content even when the application used synthetic data.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Preserve offline runtime as the default. Only explicit Discord-mode selection may
  initialize a client or register commands. Offline mode ignores Discord-only settings and preserves
  existing initialization, logging validation and process-exit behavior.
- **FR-002**: Require explicit Discord-mode configuration for one bot credential, one target server,
  one Discord owner-account identifier, one internal owner reference and enabled/disabled binding
  state. Validate the complete configuration before any provider call; unsupported modes fail safely.
- **FR-003**: Load the binding only from operator-controlled local configuration. Reuse existing
  account/owner/state validation and equality semantics; reject incomplete or malformed settings
  without partial binding, first-contact registration or implicit owner substitution.
- **FR-004**: Protect bot credentials and identity-bearing configuration from normal/pretty/nested
  diagnostics, errors and logs. Never commit real values, echo raw settings, forward raw provider
  errors or log interaction payloads. Safe errors may name the setting/category only.
- **FR-005**: Support exactly one no-argument slash command, /fundguard, in the configured server.
  Its sole behavior is private owner readiness or generic denial; there is no financial/resource
  mutation, public status command, owner-setting command or data query.
- **FR-006**: Extract actor identity from the authenticated invoking account in the provider's command
  event, never names, mentions, bot/application ownership, roles, server membership or supplied owner
  fields. Missing/invalid actor or server context MUST deny; the target server must match exactly.
- **FR-007**: Invoke the existing owner-resolution policy for every valid readiness request using
  the startup-loaded binding. A disabled binding denies uniformly. Do not add a new authorization
  engine, grant cache, resource lookup or reverse dependency from Financial Core to identity.
- **FR-008**: Authorized replies MUST state readiness and that no financial operations are available,
  without exposing identifiers, credentials or configuration. All policy/context rejections use the
  same generic denial meaning without revealing which check failed.
- **FR-009**: Every command reply or acknowledgement MUST be private to the invoking user. Initiate
  the response within Discord's three-second initial-response window under controlled test
  conditions; external delivery delays remain provider-dependent. Failure to deliver privately
  MUST NOT trigger public or unsolicited-message fallback or expose a raw error.
- **FR-010**: Register/update only this named command in the configured server; preserve unrelated
  commands, never register globally, and avoid duplicate command creation on reconnect/restart.
  A terminal registration failure prevents reporting bot readiness; no server-management capability
  or broad administrative privilege is justified by this command.
- **FR-011**: Use the minimum permissions and event access required for the slash command. Do not
  request administrator privileges, message-content access, member/presence access or process
  ordinary messages, attachments or direct messages. Explain required access in local setup docs.
- **FR-012**: Own exactly one client lifecycle in the composition/integration boundary. Distinguish
  local initialization, connected/command-ready state, request denial and terminal failures through
  safe structured categories. Report terminal startup/session failure with an unsuccessful exit;
  handle operator interruption with bounded shutdown as specified by AC-13.
- **FR-013**: Define finite startup/registration/response attempt limits and a terminal failure path
  during planning. Reconnection may use supported provider behavior, but may not create commands
  repeatedly without bound, bypass current policy or replay responses as new financial work.
- **FR-014**: Keep provider interactions and credentials outside domain/application rules; commands
  translate input into existing application policy. Integration lifecycle/configuration belong to
  their established boundaries. Add only dependencies needed by this real integration after
  compatibility review; preserve the single-package architecture and existing financial semantics.
- **FR-015**: Automated verification and CI MUST require no real credentials or services. Cover all
  fourteen acceptance scenarios with tests/reviews and controlled external boundaries, retaining
  the existing four verification gates and previously implemented behavior. Do not claim simulated
  interactions establish real Discord authentication or delivery.
- **FR-016**: Document a separate opt-in manual smoke procedure for a developer-controlled Discord
  application/server: valid owner success, second-account denial, private replies and shutdown.
  Record it as unverified unless actually executed; never require or request real tokens in chat,
  commit them, or treat configured CI as evidence of a hosted/live passing run.

### Key Entities

- **Runtime choice**: Explicit offline or Discord session selection; absent means offline.
- **Trusted connection configuration**: Operator-supplied credential and target-server selection,
  plus a validated immutable single-owner binding; never user-controlled command input.
- **Authenticated command actor**: Invoking account taken from the provider interaction boundary;
  a local synthetic event does not prove actual provider authentication.
- **Readiness request/result**: No-argument owner access check yielding minimal readiness or denial;
  no financial resource, balance, transaction or lasting permission is produced.
- **Private interaction response**: Reply visible only to its invoker; delivery failure cannot relax
  privacy or authorization requirements.
- **Session lifecycle outcome**: Initialization/command readiness, sanitized failure or operator
  shutdown; no database, financial or banking state accompanies these events.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every default/offline configuration case makes zero Discord calls; every incomplete
  Discord configuration fails before the first provider call with no sensitive diagnostic payload.
- **SC-002**: All valid configured-owner requests in the target server select private readiness;
  all unknown, disabled, missing or foreign-context cases select only generic private denial.
- **SC-003**: All response paths preserve privacy and initiate acknowledgement within three seconds
  in controlled tests; zero failure paths fall back to public/unsolicited messages.
- **SC-004**: Registration/reconnect checks find exactly the one intended command in the configured
  server scope, zero global registration, zero duplicate creation and zero unrelated-command deletion.
- **SC-005**: All fourteen acceptance scenarios have automated or documented review evidence before
  completion; required automated checks use zero real credentials/accounts/services, and optional
  live smoke verification is reported separately from simulated coverage.
- **SC-006**: Every tested terminal failure exits unsuccessfully without sensitive marker disclosure;
  controlled operator-shutdown tests complete within ten seconds. No financial/persistence or
  production-deployment capability is introduced.

## Assumptions

- The handoff's recommended first authenticated interface is selected for maintainer scope review.
  Initial live use is opt-in local development with one operator-controlled application/server and
  one owner binding. This is a post-Phase-0 feature, not permission to broaden Phase 0 retrospectively.
- Configuration is supplied through the existing process environment approach; no automatic .env
  loading, configuration editor, persistent identity store, signup or account-linking flow is added.
  Concrete setting names, dependency versions, deadlines and adapter contracts belong to planning.
- Readiness is not a financial-resource operation. Existing owner-scoped financial use cases still
  require authoritative resource lookup and current authorization when they are specified later.
- Live smoke tests require voluntarily provisioned local credentials and server access. Their absence
  does not block honest automated-only completion; it does block claiming live integration verified.
- DMs, ordinary message patterns, buttons/selects/modals, attachments, reminders, report images,
  multiple bots, Web/mobile interfaces, financial CRUD/ledger/payments, QR/OCR/AI, banking and
  production deployment are excluded. No prohibited banking credentials or real-money transfers.
- Provider documentation confirms [guild-scoped commands](https://github.com/discord/discord-api-docs/blob/main/developers/interactions/application-commands.mdx)
  and the [initial response window](https://github.com/discord/discord-api-docs/blob/main/developers/interactions/receiving-and-responding.mdx).
  Concrete integration choices and current compatibility must be verified during planning.
