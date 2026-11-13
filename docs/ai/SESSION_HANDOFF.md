# Session Handoff

Updated: 2026-11-13

## Current phase and active feature

004 Trusted Discord Owner Entrypoint is implemented and locally verified under the maintainer's
$speckit-implement request. All 38 [tasks](../../specs/004-discord-owner-entry/tasks.md) are checked;
that file owns detailed verification evidence. [Specification](../../specs/004-discord-owner-entry/spec.md),
[plan](../../specs/004-discord-owner-entry/plan.md), research, model, contracts and guide agree with delivery.
Constitution 1.1.0 and quality checklist (16/16) are unchanged. No next feature has been created.

## Implemented behavior and decisions

Default/explicit offline initializes and exits without reading bot fields. Legacy synchronous run()
stays offline; async start() validates complete live configuration before initialization/provider I/O.
Poise 0.7.1/Serenity 0.12.5 use disabled defaults/rustls, empty intents and one client/shard. Tokio has
signal/sync/time and dev-only test-util. Existing core and identity domain/application are unchanged.

Identity presentation maps real SDK actor/guild to existing resolve_owner, fixed private readiness or
generic denial, empty mentions and one 2s send. No public/DM/followup fallback. Owner-reference bytes
remain exact. Disabled binding denies; configuration changes require restart. There is no financial
resource operation, resource lookup, remembered grant or data query.

Platform owns named guild upsert and a concrete supervisor. Setup independently signals success/failure
because Client::start does not propagate Poise setup failure. FrameworkBuilder itself must disable
initialize_owners: the built-options test caught its default overriding the options field. Custom
on_error uses static categories, and both subscribers suppress SDK targets at every level.

Limits: 30s total startup, registration min(10s, remaining startup), 2s reply, observed loss 30s after
1s sampling, SDK close 8s and total close/cancel/join 10s. SDK internal retries are not wire-count
claims. Accepted [ADR 0005](../architecture/decisions/0005-trusted-discord-entrypoint.md) records choices.

## Verification and limits

Passed cargo fmt --all -- --check, cargo clippy --all-targets -- -D warnings, cargo test and cargo check.
69 tests passed: 33 library + 1 binary unit + 4 bootstrap + 6 entrypoint + 15 core + 10 identity.
Also passed locked check/fetch and explicit offline cargo run --locked; stdout empty, safe init event.
Checks used one build job and disabled dev/test debug info for SDK build resource use. No real Discord
client is constructed by automated tests; no credentials/provider sockets/database are needed.

Reviewed final diff, module visibility/direction, minimal resolved features, secret markers, .env ignores,
source size and local documentation links. Existing CI already runs locked gates with read-only
permissions and no credentials; retained after review. Generated tooling is unchanged. The session
began from clean HEAD 422305c; no unrelated work was overwritten.

Full Poise dispatch/error callbacks require SDK runner context. Offline tests use the real command
table/builders and source review rather than a fabricated dispatcher. Real authentication, registration
preservation, private delivery, reconnect and live Ctrl-C cleanup remain unverified; the guide has
individual unverified smoke rows. Hosted CI is unverified. No complete security audit/panic-payload
containment or memory-zeroization claim. Local tests are not provider integration evidence.

## Pending work, blockers and next action

No partial or blocked implementation tasks. Optional live smoke remains opt-in with locally supplied
credentials; never ask for tokens in chat or commit identifying payloads. No deployment was performed.
Next recommended specification: persistence and atomic financial-commit boundaries, including
trusted resource ownership, origin/audit retention, concurrency and durable idempotency. Do not begin
financial commits/payments or add SQLx until separately specified and authorized.

Use [FEATURE_INDEX.md](FEATURE_INDEX.md) if the ignored selector is absent. Delivered active directory
is specs/004-discord-owner-entry; actual Git branch remains 001-project-foundation. Read current source
and tasks at the next session; do not rely solely on this summary or conversation history.
