# Foundation Runtime Contract

## Invocation and Effects

`cargo run --locked` runs `fundguard` after implementation. No application arguments are required;
CLI argument parsing is not introduced in Phase 0. No network listeners, outbound connections,
Discord clients, database pools, files containing business data, or scheduled jobs are created.
The process initializes and exits. Cargo itself may download crates before the executable starts.

## Environment

| Setting | Required | Default | Accepted values |
| --- | --- | --- | --- |
| FUNDGUARD_LOG_LEVEL | No | info | trace, debug, info, warn, error (exact lowercase strings) |

Empty strings, whitespace, unknown values, uppercase values, and non-Unicode values are invalid.
No `.env` file is loaded implicitly. Other environment variables are not application settings;
Discord tokens and database URLs must never activate integrations or appear in output.

## Success

Exit status is 0. At default level, stderr contains a structured tracing INFO event with stable
field `event="application_initialized"` (formatter spacing/quoting may vary) indicating local
initialization only. Levels trace/debug/info permit this event; warn/error suppress it while
preserving successful exit. No application output is written to stdout. Disable ANSI output and
avoid relying on timestamps or complete formatted lines in tests.

## Failure

Invalid configuration produces a nonzero exit and a stderr diagnostic identifying
`FUNDGUARD_LOG_LEVEL` and accepted values without including the supplied value. Non-Unicode input
may use the same safe validation diagnostic. Subscriber setup failure also exits nonzero with a
safe structured ERROR event with stable field `event="application_initialization_failed"`.
Exact nonzero exit code is not part of the contract. No initialization success event appears
when startup fails. If normal tracing setup has not succeeded, emit the failure through a minimal scoped ERROR
tracing subscriber on stderr, independent of invalid configuration or existing global subscribers.
No plain-text production diagnostic exception is permitted.

Neither Display nor Debug for startup/configuration errors may expose raw settings, credentials,
or arbitrary underlying diagnostic strings. No stack trace, environment dump, or token is emitted.

## Validation Boundaries

Use pure parser tests for all log-level cases and subprocess tests for actual exit/output behavior.
Do not mutate the parent process environment. Synthetic marker values test redaction; they are
not real credentials. Subscriber collision can be verified with scoped/unit setup or isolated
process tests, never competing global initialization across parallel tests. Bound every child
with a 10-second test deadline, including collision-test children; kill/reap on timeout and capture
output safely so full pipes cannot defeat the deadline. No production latency SLA is implied.
Invalid-configuration subprocess cases must assert the ERROR event and stable failure field in
addition to exit status, safe setting/allowed-values text, empty stdout, and marker redaction.
The subscriber-collision unit test verifies the typed startup failure; it need not create a new
production argument, configuration setting, or injection interface to force that condition.

The application exports only its startup entrypoint and error type; module details and configuration
remain private. No REST, Discord command schema, banking contract, or database API is exposed yet.
