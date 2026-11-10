# Phase 0 Data Model

This describes initialization data, not a database schema. There are no persisted records,
financial entities, user entities, migrations, or financial state transitions in Phase 0.

## Runtime Configuration

| Attribute | Representation | Validation / default | Lifetime |
| --- | --- | --- | --- |
| Log level | Typed tracing level | Absent → info; exactly trace/debug/info/warn/error | Startup process only |

Configuration is immutable after construction. The parser receives an optional OS string to
support absent and non-Unicode input without modifying process-global environment. Invalid input
produces a typed error naming `FUNDGUARD_LOG_LEVEL`; the error does not retain the supplied value.
There is no Discord token, owner ID, database URL, or financial setting in this phase.

## Startup Errors

Distinguish invalid/non-Unicode log-level configuration from inability to install diagnostics.
Error Display and Debug must be safe: retain categories and setting names, never raw environment
values or unreviewed external error strings. `thiserror` defines explicit types; the composition
root propagates errors instead of panicking. Exact private type names can follow implementation
conventions without changing the external contract.

## Lifecycle

Process started → configuration validated → diagnostics installed → initialization event emitted
(subject to level filtering) → successful exit. Validation or diagnostic setup failure → safe
structured ERROR tracing event through a scoped fallback subscriber → unsuccessful exit.
The stable failure event field is application_initialization_failed; errors retain no raw values. There is no business mutation at any transition.

## Feature Delivery Record

The feature specification owns requirements; the technical plan owns design; tasks.md owns the
implementation checklist; the handoff links to actual status and evidence. These are repository
artifacts, not runtime records or a new persistence subsystem. See [plan.md](plan.md) for boundaries
and [runtime contract](contracts/runtime.md) for observable behavior.
