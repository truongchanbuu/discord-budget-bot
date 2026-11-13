# Rust coding standards

Use Clean Code as guidance adapted to idiomatic Rust. Readability and clear responsibilities
matter more than arbitrary function lengths. Keep a cohesive function together instead of
splitting it into trivial wrappers. Prefer explicit control flow and standard-library facilities.

## Naming, responsibilities, and control flow

Use intention-revealing snake_case functions/modules, PascalCase structs/enums, and descriptive
variants. Apply Single Responsibility at function, module, and feature level. Separate validation,
workflow, transformation, and persistence when those phases are meaningful. Use guard clauses to
avoid deep nesting; no nested ternaries or unclear boolean flags. Prefer named enums for modes.

Acceptable standalone example:

```rust
fn validate_label(label: &str) -> Result<(), &'static str> {
    if label.trim().is_empty() {
        return Err("label is required");
    }
    Ok(())
}
```

Unacceptable style (compiles, but obscures responsibility and uses an unclear boolean):

```rust
fn process_data(s: String, flag: bool) -> bool {
    if flag {
        if !s.is_empty() {
            return true;
        }
    }
    false
}
```

Prefer commands that make mutation explicit and queries that return values without hidden side
effects. Do not mechanically create command/query classes or CQRS infrastructure.

## Ownership and mutable state

Borrow read-only input, own data when retaining it, and move values when ownership transfers.
Clone only when independent ownership is needed; do not clone to avoid understanding lifetimes.
Keep state immutable by default and mutation local.

Good:

```rust
fn label_length(label: &str) -> usize {
    label.len()
}
```

Bad: unnecessary allocation and ownership requirement:

```rust
fn label_length(label: String) -> usize {
    label.clone().len()
}
```

## Errors and safe diagnostics

Use Result and ? for recoverable failures, with thiserror for explicit domain/application/startup
error types. Preserve meaningful categories without retaining secrets or raw external diagnostics.
Use Option only for absence, not to erase useful errors. No unwrap/expect in runtime-critical code
unless a documented invariant justifies it; test fixture/assertion failures may use them.

Good:

```rust
fn parse_count(input: &str) -> Result<u32, &'static str> {
    input.parse().map_err(|_| "count must be an unsigned integer")
}
```

Bad: panic and unreviewed raw-input handling:

```rust
fn parse_count(input: &str) -> u32 {
    input.parse().unwrap()
}
```

All production events use structured tracing, including startup failures through the scoped
fallback. Stable event names and safe categorical fields support diagnosis. Never log tokens,
bank data, input attachments, environment dumps, or sensitive financial amounts. Check Display
and Debug too. Avoid blanket instrument/debug logging of request/configuration objects.

Good diagnostic pattern: tracing::error!(event = "application_initialization_failed", error = %error),
where error is a reviewed safe type. Bad: println!/eprintln! production logs or a tracing event
that includes a raw token, config object, or external provider response.

## Modules, visibility, and abstractions

Private by default. Expose only the use cases/types needed by callers; prefer pub(crate) for
internal access. Re-export a narrow public surface from lib.rs. Feature presentation calls
application use cases; infrastructure implements actual ports, never the reverse. No circular
imports or direct calls into another feature's adapters.

Good: mod config with a private parser and one reviewed startup API. Bad: pub mod config plus
public mutable settings solely to make tests reach internals. Keep unit tests with private code.

Pass actual dependencies through constructors/parameters when a real boundary needs injection.
Introduce traits for concrete substitution, provider boundaries, or meaningful testing problems;
prefer concrete types otherwise. Generics must improve a demonstrated use case, not build a
universal repository/service layer. Avoid wrapper methods that only rename a call.

## Async and lifecycle

Only use async for real asynchronous work or the runtime entrypoint. Do not mark pure validation
async. Avoid blocking I/O on runtime workers; choose the provider's async API or a justified
blocking boundary when needed. Tokio runtime/client/pool/scheduler lifecycle belongs to composition
and platform; business rules do not create runtimes. Do not enable Tokio full speculatively.

## Future domain values and transactions

Financial Core implements pure validated whole-VND values; the ledger now implements owner-serialized
PostgreSQL journal transactions under its explicit application port.
Use validated value objects for
actual invariants, with explicit currency/units; never floating-point financial amounts. Avoid
boolean flags that confuse virtual funds with bank verification. Financial Core defines
range, overflow, currency scale and reference semantics in its authoritative contract; future
extensions must resolve rounding and identity requirements before implementation.

A future mutation must atomically persist its financial entry, traceable origin, idempotency state,
and audit information within the owning use case's transaction boundary. A retry/duplicate must
not create another entry. Keep external calls outside transactions unless the owning spec
explicitly resolves locking and failure semantics. Infrastructure uses SQLx only when specified;
application workflows own transaction policy and expose no SQLx types to domain logic.

## Tests, documentation, and formatting

Use cargo fmt, Clippy, cargo test, and cargo check as documented in [README](../../README.md).
Unit tests cover pure rules beside their modules; integration tests exercise observable public
behavior. Test defaults, invalid inputs, error categories/redaction, and meaningful boundaries;
avoid asserting private formatting or mirroring syntax. Foundation tests require no real secrets,
Discord, or PostgreSQL. Isolate global subscriber state in children; never mutate parent process
environment. Child deadlines and output draining prevent hung tests.

Comments explain why, constraints, or non-obvious decisions; do not narrate obvious statements.
Rustdoc describes public contracts. Good comment: why fallback logging must work after global
installation failure. Bad comment: "return success" immediately before Ok(()).
Record architectural tradeoffs in ADRs, feature requirements in Spec Kit, and actual progress in
its tasks and [handoff](../ai/SESSION_HANDOFF.md). Do not create competing specifications.
