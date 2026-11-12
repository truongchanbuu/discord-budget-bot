# Security baseline and preliminary threat model

This is a preliminary design baseline, not a production security certification. Phase 0 runs
locally without external connections or business data. The [constitution](../../.specify/memory/constitution.md)
and [active feature index](../ai/FEATURE_INDEX.md) govern requirements.

## Assets, actors, and boundaries

Current assets: source integrity, Cargo.lock, dependency provenance, local process configuration,
structured diagnostic integrity, and CI permissions. Future assets: Discord tokens, owner bindings,
financial records, attachments, idempotency records, and audit trails. Do not create sample personal
financial records for foundation tests.

Actors include the owner, future other users/untrusted Discord senders, malicious attachment or
provider-response authors, dependency maintainers/compromised packages, and contributors who can
change CI. Current entry points are process environment and repository/CI inputs. Future Discord
interactions, messages, attachments, scheduled execution, and provider responses are untrusted.
Trust boundaries include environment-to-parser, repository-to-CI runner, Discord-to-presentation,
identity-to-authorized use case, application-to-provider, and persistence-to-financial state.

## Threats and initial controls

| Threat | Boundary / asset | Initial mitigation | Current state and future owner / residual risk |
| --- | --- | --- | --- |
| Token/secret exposure | Environment/repository/logs | No hardcoded secrets; ignore real .env; safe example only; no raw input retained in errors | Implemented for supported config. Discord integration must secure token provisioning/rotation; no token support exists yet |
| Sensitive financial logging | Provider/application diagnostics | Structured events with safe categories; review Display and Debug; never dump config, requests, attachments, or amounts | Startup redaction tested. Every future feature must review its own diagnostic fields |
| Malicious messages/images | Discord-to-presentation | Validate type, length, size, parsing limits and authorization before use cases; least privilege | No Discord/attachment processing yet. Future presentation/infrastructure must address denial of service and untrusted content |
| Wrong-owner access | Identity-to-application | Explicit owner binding and fail-closed authorization; do not trust all senders in personal-use mode | Pure owner policy implemented; no authenticated adapter or authoritative resource lookup. Live integrations must enforce every caller obligation below |
| Forged or replayed completion | External/manual signal-to-ledger | Distinguish handoff/manual confirmation/bank verification; atomically persist idempotency and audit origin | No payments/ledger yet. Financial Core/payment specs own concurrency and verification semantics |
| Dependency compromise | Registry-to-build | Minimal dependencies/features, reviewed source/release metadata, committed lockfile, controlled upgrades | Lockfile/build reviewed. This is not a vulnerability audit; future upgrades require fresh review |
| CI compromise | Contribution-to-runner/token | Read-only contents permission, full-SHA official checkout, no persisted credentials, no privileged PR events or deployment secrets | Workflow configured; hosted execution unverified until a real run. Untrusted code still executes in CI |
| Malformed environment / diagnostic failure | Environment-to-startup | Typed validation; no raw values in errors; scoped ERROR fallback if global logging fails | Unit/integration/collision tests cover current paths; startup failure exits nonzero |

## Financial Core controls and trust limits

Financial Core is a pure library with no logging, environment access, clock, provider, registry,
journal or persistence. It rejects unsupported money formats/currencies, fractions, negative values,
zero changes, overflows, blank references and virtual overdraft. Errors retain only categories;
Debug/pretty/nested formatting redacts financial values and references. Deliberate getters are
not permission to log payloads. Synthetic tests and external privacy compile probes verify this
boundary without real accounts or records.

ValidatedChange is constructible only through successful validation, with immutable origin/intent
and before/proposed snapshots. It proves internal coherence only. OwnerRef is not authentication;
callers vouch for supplied prior context. Two calls with None can both be eligible. Replay returns
the supplied prior result before fresh capacity checks; it is not a concurrent exactly-once commit.
Handoff-only input cannot yield fresh completion; manual confirmation remains explicitly virtual.
No bank-proof variant, mutable result setter, or unchecked restoration API exists.

Future input adapters must impose justified size/rate limits and bind/authorize owners. Persistence
must preserve the full 0–18,446,744,073,709,551,615 VND range or revisit its schema explicitly, validate
restoration, and atomically commit effect/origin/key/audit data. Do not silently narrow to a signed
column or represent validation candidates as journal receipts. These obligations remain future
work. See [ADR 0003](../architecture/decisions/0003-financial-core-validation.md).

## Owner-access policy and caller trust

Identity now validates canonical account identifiers and evaluates an explicitly supplied immutable
single-owner binding. Missing/unknown actor, missing/disabled binding, or absent/foreign resource
scope produces the same Denied category. Subsequent evaluations use current inputs; prior success
never overrides a disabled/absent binding. Values/bindings/results redact supported diagnostics and
errors contain no source payload. No logging, provider, environment, storage or mutable cache exists.

This is policy, not live authentication or proof of resource ownership. Future integrations must:

1. Obtain actors from authenticated provider events, never names, mentions or user-submitted IDs.
2. Load bindings from trusted operator configuration, not the requesting actor.
3. Retrieve resource ownership authoritatively, independently of a client-supplied owner claim.
4. Check current access before every private use case, including retries/replays; never reuse a
   returned owner reference as a permission credential.
5. Map denial uniformly without disclosing configured/foreign identity or dumping request data.
6. Apply the same boundary to future commands, messages, components and additional interfaces.

Public TrustedActor construction asserts the caller's trust; it cannot verify provider provenance.
The policy takes supplied owner scope without looking up a resource. Binding loading, real
integration, in-flight revocation and transactional enforcement remain deferred. No claim of
constant-time denial, durable permission or complete production security is made. See the
[contract](../../specs/003-owner-access/contracts/owner-access.md) and
[ADR 0004](../architecture/decisions/0004-owner-access-policy.md).

## Secrets and integration guidelines

Never commit real tokens, banking credentials, personal records, or secret .env files. .env.example
contains only the supported non-secret log setting and is not automatically loaded. Do not pass
real secrets in commands or use them in tests. Future token provisioning must use deployment
secret facilities or a justified secret store with restricted access. If exposed, revoke/rotate
at the provider and follow a scoped incident cleanup; Git ignore is not access control.

Use minimum justified gateway intents. Message-content/attachment access must be explicitly
justified by an owning feature. External integrations must validate responses, define timeouts,
retry/idempotency boundaries, authenticate appropriately, and authorize the bound user before
business actions. Never trust provider payloads simply because transport succeeded. No provider
SDK, banking integration, or generalized security framework is introduced in Phase 0.

## Financial constraints for future work

Never collect or store banking passwords, OTPs, card PINs, CVVs, or banking-session credentials.
The MVP must not directly execute real-money transfers. Use no floating-point financial amounts.
Virtual fund balances are not verified bank balances. Every mutation requires a traceable origin,
auditable history, and atomic idempotency so duplicate completion creates no duplicate entry.
Awareness must occur before handoff. Handoff is not payment confirmation or successful transfer;
manual confirmation is not bank verification. Minimize and protect sensitive financial data.

Trusted binding loading, retention/deletion, encryption at rest, in-flight revocation, recovery, multi-user
isolation, transaction concurrency, provider verification, and deployment hardening remain open
for their owning future specifications. There is no financial persistence or bank connection now.
Passing foundation tests proves neither financial correctness nor complete production security.
