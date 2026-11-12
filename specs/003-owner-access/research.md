# Owner Access Research and Decisions

Date: 2026-11-12. Research record for the now-implemented design. [tasks.md](tasks.md) owns
verification evidence; research itself made no dependency or lockfile change.

## 1. Existing owner identity and feature direction

**Decision**: Reuse Financial Core's existing OwnerRef exclusively in identity application code
through its public feature facade. Identity domain owns DiscordAccountId, TrustedActor,
BindingState and AccessError; application owns OwnerBinding and access functions.

**Rationale**: Source inspection confirms nonblank Unicode-whitespace validation, exact byte/case
equality and redacted Debug already exist. The constitution requires cross-feature calls through
explicit application boundaries. Application-level reuse preserves this contract without coupling
identity domain to financial internals. Financial Core has no reverse identity dependency.

**Alternatives considered**: Duplicate owner types repeat validation/conversions; domain-level
imports bypass the application boundary; moving OwnerRef to shared changes a delivered feature
without a current need. Untyped strings weaken role/value safety. None is necessary.

## 2. Canonical Discord account input

**Decision**: DiscordAccountId privately wraps std::num::NonZeroU64. Validate the specified lexical
form before checked u64 parsing: 1–20 ASCII digits, first digit 1–9, value at most u64::MAX.
Map all malformed/overflow cases to payload-free InvalidAccountId.

**Rationale**: The grammar is an explicit application requirement, not provider-account discovery.
Rust integer parsing alone is insufficient for a strict lexical contract: its documented grammar
permits a plus sign. Lexical validation rejects signs, zeros/padding, Unicode digits and mentions.
The 20-digit bound follows the numeric maximum and is not an arbitrary restriction.
NonZeroU64 keeps zero unrepresentable without unsafe code or another crate.

**Alternatives considered**: Raw strings retain unnecessary input and permit inconsistent forms;
plain u64 admits zero internally; regex/UUID/SDK wrappers add dependencies without current benefit.

Primary references checked on 2026-11-12:
[standard NonZero documentation](https://doc.rust-lang.org/std/num/struct.NonZero.html),
[integer parsing rules](https://doc.rust-lang.org/std/primitive.u64.html#method.from_str_radix).
These sources describe representation/parsing only, not authentication or Discord account validity.

## 3. Dependencies and compatibility

**Decision**: Keep Cargo.toml, Cargo.lock and rust-toolchain.toml unchanged. Use Rust 1.99.1,
Edition 2024, standard-library values and the existing thiserror 2.0.21 for AccessError.
No new async work or feature flags. Existing Tokio 1.54.1, tracing 0.1.44 and tracing-subscriber
0.3.23 continue serving startup only.

**Rationale**: Manifest/lockfile inspection confirms these pinned resolutions; the current standard
library documentation is 1.99.1 and provides the required nonzero and checked parsing facilities.
The [versioned thiserror documentation](https://docs.rs/thiserror/2.0.21/thiserror/) supports ordinary
categorical errors and static Display messages. Nothing here requires a provider or framework upgrade.
Compatibility was subsequently verified during feature 003 implementation; actual results belong
to tasks.md. Research references are not a vulnerability audit or a substitute for that evidence.

**Alternatives considered**: Poise/Serenity, SQLx, Serde, auth/token libraries and a multi-crate
workspace all exceed this pure policy increment. Selecting latest versions or enabling optional
providers without functionality would increase maintenance without satisfying another requirement.

## 4. Policy results, borrowing and revocation

**Decision**: Two synchronous functions resolve the configured owner and authorize explicit owner
scope. Both take optional actor/binding; scoped access also takes optional authoritative owner.
Return Result<&OwnerRef, AccessError>, borrowing the binding. Use one Denied error for every policy
rejection. No AuthorizedOwner token, boolean mode, mutable policy object, cache or prior-grant input.

**Rationale**: OwnerRef is already validated and redacted. A reference exposes exactly the owner
needed by future use cases without allocation, duplicate result records or misleading credentials.
Each invocation evaluates current supplied binding; retaining a reference does not grant future
access. No claim is made about in-flight revocation or synchronizing external actions.

**Alternatives considered**: An owned grant/session introduces lifecycle questions outside scope;
a bool loses the resolved owner; separate detailed denial reasons disclose binding/scope state.

## 5. Trusted input and diagnostics

**Decision**: TrustedActor::new accepts a validated account as a caller assertion. Its name and
contract explicitly disclaim proof of provider authentication. It is separate from raw content;
there is no authenticated=true flag, channel/role bypass or live provider verifier.
Custom Debug redacts account, actor and binding payloads; OwnerRef already redacts results.
Static AccessError Display/Debug carries no raw input or nested provider source.

**Rationale**: Pure local construction cannot cryptographically authenticate a provider. Creating a
fake sealed proof mechanism would add complexity without a real adapter. Future adapters must
supply authenticated actor identity, trusted configuration and independently fetched resource scope.

**Alternatives considered**: Inferring identity from display names/mentions, accepting a client owner
claim, or treating local construction as authentication would weaken the specified trust boundary.

## 6. Verification and unresolved decisions

**Decision**: Public-contract tests cover values, resolution, scoped checks, privacy and unchanged
inputs with synthetic fixtures. Review the integration guidance for AC-10; combine tests and source/
dependency review for AC-12. No fabricated live-interface end-to-end claim is permitted.

**Rationale**: Those two scenarios include obligations on integrations not built by this increment.
A channel loop can demonstrate identical supplied inputs produce identical policy decisions; it
must not introduce a channel-dispatch abstraction solely for tests.

All planning unknowns are resolved. Future binding loading, provider authentication, authoritative
lookup, in-flight revocation and owner transfer are explicit exclusions rather than hidden defaults.
