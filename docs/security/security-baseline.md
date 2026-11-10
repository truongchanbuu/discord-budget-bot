# Security baseline and preliminary threat model

This is a preliminary design baseline, not a production security certification. Phase 0 runs
locally without external connections or business data. The [constitution](../../.specify/memory/constitution.md)
and [active specification](../../specs/001-project-foundation/spec.md) govern requirements.

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
| Wrong-owner access | Identity-to-application | Explicit owner binding and fail-closed authorization; do not trust all senders in personal-use mode | No auth system implemented. Identity feature must specify isolation, revocation, and permitted actions |
| Forged or replayed completion | External/manual signal-to-ledger | Distinguish handoff/manual confirmation/bank verification; atomically persist idempotency and audit origin | No payments/ledger yet. Financial Core/payment specs own concurrency and verification semantics |
| Dependency compromise | Registry-to-build | Minimal dependencies/features, reviewed source/release metadata, committed lockfile, controlled upgrades | Lockfile/build reviewed. This is not a vulnerability audit; future upgrades require fresh review |
| CI compromise | Contribution-to-runner/token | Read-only contents permission, full-SHA official checkout, no persisted credentials, no privileged PR events or deployment secrets | Workflow configured; hosted execution unverified until a real run. Untrusted code still executes in CI |
| Malformed environment / diagnostic failure | Environment-to-startup | Typed validation; no raw values in errors; scoped ERROR fallback if global logging fails | Unit/integration/collision tests cover current paths; startup failure exits nonzero |

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

Identity binding, retention/deletion, encryption at rest, access revocation, recovery, multi-user
isolation, transaction concurrency, provider verification, and deployment hardening remain open
for their owning future specifications. There is no financial persistence or bank connection now.
Passing foundation tests proves neither financial correctness nor complete production security.
