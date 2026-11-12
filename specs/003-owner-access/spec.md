# Feature Specification: Owner Identity and Access Boundaries

**Feature Branch**: `003-owner-access` (feature identifier; Git branch remains `001-project-foundation`)

**Created**: 2026-11-12

**Status**: Implemented and verified locally on 2026-11-12 under the maintainer's
$speckit-implement instruction. Completion evidence is in [tasks.md](tasks.md).

**Input**: User description: "$speckit-specify pls continue with next feature".
The roadmap recommends identity/authorization before financial workflows. This specification selects
single-owner identity binding and owner-scoped access decisions as the next bounded increment.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Recognizing the Configured Owner (Priority: P1)

As the owner, I want my explicitly configured Discord account to resolve to my internal owner
identity, so future interactions cannot infer ownership from whoever sends a message first.

**Why this priority**: The existing Financial Core accepts owner references without authenticating
users. Explicit binding establishes the access policy required before exposing private use cases.

**Independent Test**: Supply a synthetic trusted actor and configured binding; verify resolution
without Discord access, stored accounts, financial data, or resource lookup.

**Acceptance Scenarios**:

1. **AC-01 — Known owner**: **Given** one valid enabled binding between a Discord account and an
   internal owner, **When** a trusted actor with that exact account is resolved, **Then** the result
   contains only the bound internal owner identity and succeeds without changing the binding.
2. **AC-02 — Unknown actor**: **Given** the same binding, **When** another account is resolved,
   **Then** access is denied and no bound owner or account identifier is disclosed.
3. **AC-03 — No binding**: **Given** no binding, **When** any actor is resolved, **Then** access is
   denied; the actor is never automatically registered as the owner.
4. **AC-04 — Invalid identity**: **Given** missing or malformed account/owner input, **When** it is
   validated, **Then** it cannot produce a valid binding or successful access decision.

---

### User Story 2 - Restricting Access to the Owner's Scope (Priority: P1)

As the owner, I want future private operations to accept only my authorized owner scope, so
someone cannot obtain access by submitting a different owner's reference in a request.

**Why this priority**: An identity match alone cannot establish ownership of the data being used.

**Independent Test**: Supply synthetic trusted actor, binding, and authoritative resource-owner
scope; verify allow/deny decisions without actual funds, resources, or financial mutations.

**Acceptance Scenarios**:

1. **AC-05 — Matching scope**: **Given** a matching trusted actor and enabled binding, **When** the
   authoritative resource-owner scope equals the bound owner, **Then** access is allowed for that
   owner only; the supplied scope and binding remain unchanged.
2. **AC-06 — Foreign scope**: **Given** a matching trusted actor, **When** the resource-owner scope
   belongs to another owner, **Then** access is denied without returning that owner's details.
3. **AC-07 — Forged owner claim**: **Given** an unknown actor who supplies the configured owner's
   reference, **When** access is evaluated, **Then** access is denied regardless of the owner claim.
4. **AC-08 — Revoked access**: **Given** a previously successful decision, **When** a new decision
   uses a disabled or absent binding, **Then** access is denied; prior success grants no continuing
   access and no hidden remembered authorization overrides the current binding.

---

### User Story 3 - Keeping Trust and Privacy Explicit (Priority: P2)

As a future interface maintainer, I want a reusable access policy with explicit trust limits and
safe failures, so commands, messages, and interactions can share rules without exposing identities.

**Why this priority**: Reuse must preserve the distinction between provider-authenticated actors,
application authorization, and untrusted message content.

**Independent Test**: Exercise the same policy with synthetic trusted inputs representing different
interaction types; inspect failures and diagnostics without connecting any interface.

**Acceptance Scenarios**:

1. **AC-09 — Input-channel independence**: **Given** the same trusted account, binding and scope,
   **When** the decision is requested for a message, command or component interaction, **Then** the
   owner resolution and access outcome are identical; channel/server membership conveys no grant.
2. **AC-10 — No trust from content**: **Given** a username, display name, mention or owner field in
   untrusted content, **When** future integration guidance is reviewed, **Then** it requires the
   authenticated actor from the provider and forbids using that content as identity evidence.
3. **AC-11 — Safe diagnostics**: **Given** synthetic account and owner markers, **When** supported
   diagnostic formatting and errors are inspected, **Then** they reveal no raw account/owner
   identifiers, binding contents, credentials, financial payloads or hidden error-source payloads.
4. **AC-12 — Honest policy limits**: **Given** only a locally evaluated access decision, **When**
   its meaning is inspected, **Then** it makes no claim of provider authentication, resource lookup,
   durable permission, financial commit or bank verification, and produces no external side effect.

### Edge Cases

- Empty or Unicode-whitespace-only owner references cannot define an owner; other owner bytes
  retain the exact equality semantics already established by Financial Core.
- Account identifiers with signs, whitespace, leading zeros, zero, non-ASCII digits, punctuation,
  or a value beyond the accepted range are invalid; names and mentions are not accepted aliases.
- A configured account differs from an actor by one digit: deny, with no fuzzy matching.
- Disabled binding plus matching account/scope still denies; a foreign scope never becomes allowed
  because the actor is the configured owner.
- An old successful result is supplied after revocation: it cannot replace a new policy evaluation.
- Same actor with a changed binding owner resolves only to the current supplied owner; no stored
  previous mapping is consulted. Operational account transfer and financial-data migration are excluded.
- A locally constructed actor can imitate a real account: pure policy cannot verify its provenance.
  Future adapters must obtain the actor from an authenticated provider boundary.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Support one explicitly supplied owner binding relating a Discord account identifier
  to one internal owner reference, with an enabled/disabled state. No first-contact registration,
  fallback owner, multi-owner collection, or automatic inference from a message is permitted.
- **FR-002**: Accepted Discord account text MUST be a positive ASCII decimal integer from 1 through
  18,446,744,073,709,551,615, without leading zeros, signs or surrounding whitespace. Reject invalid
  input without normalization, truncation or retaining raw input in errors. This is the application's
  accepted identifier format; it is not a claim that the account exists or is authenticated.
- **FR-003**: Internal owner references MUST follow the existing Financial Core owner semantics:
  reject empty/Unicode-whitespace-only input; compare remaining bytes exactly without trimming,
  case folding or reinterpretation. Do not redefine or weaken Financial Core's existing contract.
- **FR-004**: Resolve a supplied trusted actor only when an enabled binding contains its exact
  account identifier. Return the configured internal owner on success, never a request's claimed
  owner. Missing actor, absent/disabled binding and nonmatching account MUST deny by default.
- **FR-005**: A scoped-access decision MUST require successful owner resolution and equality between
  the resolved owner and the explicitly supplied authoritative resource-owner scope. Missing scope
  cannot authorize resource access. An actor's requested scope does not establish resource ownership.
- **FR-006**: Access denial MUST reveal neither the configured identity nor another owner's data.
  Unknown actors, disabled/absent bindings and foreign scopes MUST have the same externally visible
  denial meaning. Malformed values may have safe field-category validation errors without payloads.
- **FR-007**: Each decision MUST use the binding supplied for that evaluation. No hidden cache,
  sticky session, remembered grant or previous success may override absence/disablement. A successful
  result is a decision about these inputs, not a reusable credential or authorization for later work.
- **FR-008**: Separate trusted actor input, configured binding, resolved owner, and authoritative
  resource-owner scope from untrusted content. Document that future adapters authenticate the actor
  and retrieve resource ownership independently of client claims before invoking private use cases.
- **FR-009**: Policy MUST behave identically across supported future interaction types and remain
  deterministic for supplied inputs, without network, environment, storage, clock or global state.
  Inputs MUST remain unchanged. Success MUST perform no financial or other external mutation.
- **FR-010**: Binding, actor, owner-bearing results and errors MUST have safe supported diagnostic
  formatting, including nested/pretty forms and error chains. Do not log raw identifiers, bindings,
  credentials or financial content. Deliberate identity access for use cases is not logging consent.
- **FR-011**: Preserve feature ownership and provider-independent business boundaries. Expose only
  meaningful access decisions and validated concepts. Avoid speculative roles, permission engines,
  generic authentication frameworks, empty layers, or moving existing financial rules into identity.
- **FR-012**: Document future enforcement obligations: authenticate external actors; obtain bindings
  from trusted operator configuration and owner scope from authoritative data; evaluate access for
  every private use case, including replays; re-check current access before acting. A policy result
  alone proves none of those integration steps has occurred.
- **FR-013**: Verify all acceptance scenarios with synthetic identities and no real credentials or
  external accounts. Preserve existing offline bootstrap behavior, Financial Core semantics and
  repository verification gates. Record requirement review separately from implementation evidence.

### Key Entities

- **Discord account identifier**: Canonical accepted external account reference; not a name, token,
  proof of authentication, internal owner, or financial account.
- **Trusted actor**: Account identity asserted by a trusted calling boundary for one evaluation;
  local construction cannot prove that a provider authenticated it.
- **Owner binding**: Operator-controlled association between one account and one internal owner,
  including whether access is currently enabled; not an editable request parameter.
- **Internal owner reference**: Existing opaque Financial Core owner scope, independent of Discord.
- **Resource-owner scope**: Explicit authoritative owner of the resource being accessed; no resource
  storage or lookup is introduced here.
- **Access decision**: Resolved owner or denial for supplied inputs; no session, lasting grant,
  financial mutation, or evidence of external authentication.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All known-owner scenarios resolve exactly the configured internal owner; 100% of
  unknown/missing/disabled identity scenarios deny, with zero automatic registrations.
- **SC-002**: Every scoped-access case allows only the combination of matching actor, enabled
  binding and matching authoritative owner; every missing/foreign-scope case denies.
- **SC-003**: All boundary identifier cases have deterministic accept/reject outcomes with zero
  implicit normalization, owner substitution or changes to supplied inputs.
- **SC-004**: All supported diagnostic/error cases expose zero synthetic sensitive markers; denial
  outcomes reveal no distinction that discloses configured or foreign identity details.
- **SC-005**: All 12 acceptance scenarios are verified before implementation completion, using
  zero live services or real accounts, while existing foundation and Financial Core checks pass.
- **SC-006**: Review finds zero live authentication flows, stored accounts, financial mutations,
  lasting grants, banking integrations or deployment changes in this increment. Integration guidance
  explicitly identifies all caller trust obligations in FR-012.

## Assumptions

- This next feature is a single-owner identity/access-policy increment selected from the tracked
  roadmap. Maintainer review confirms this scope before planning; Financial Core remains complete.
- Bindings, trusted actors and authoritative scope are supplied explicitly. Loading operator
  configuration, creating a live authenticated adapter and resource lookup belong to later features.
- Removing/disabling the binding affects subsequent evaluations. In-flight revocation, hot reload,
  credential recovery, owner transfer and financial-data migration require separate specifications.
- No user signup, account-linking command, OAuth/session/token handling, roles/admin delegation,
  multi-user administration, database, HTTP interface, live Discord bot or scheduled actor exists here.
- Funds, ledger posting, income, payment awareness/handoff, reports, reminders, QR/OCR/AI, banking
  integration, real-money transfers and deployment are excluded. No prohibited banking credentials
  are collected. Existing code, dependencies, constitution and unrelated user edits stay unchanged
  during this specification step.
- Concrete contracts, code placement and any necessary architecture decision belong to planning.
  This document specifies observable policy and trust limits, not a production authentication system.
