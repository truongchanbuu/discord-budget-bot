# ADR 0002: Offline initialization with minimal dependencies

Date: 2026-11-10. Status: Accepted for Phase 0.

## Context

A safe foundation must compile and test without Discord tokens, PostgreSQL, or business records.
It must exercise configuration, lifecycle, errors, and diagnostics rather than simulate features.

## Decision

Pin Rust 1.99.1, Edition 2024. Use Tokio macros/rt with a current-thread runtime, tracing std,
tracing-subscriber fmt/std, and thiserror. Default features are disabled for the first three;
Cargo.lock records the actual resolution. No serialization, network, database, scheduler, or
Discord dependency is needed yet. Verify feature flags when adding the first real integration.

Read FUNDGUARD_LOG_LEVEL only, default info, with exact validated lowercase levels and no implicit
.env loading. Initialize then exit. Startup errors retain safe categories, not raw values or
subscriber error strings. Use a scoped ERROR subscriber for failure reporting, preserving the
constitution's structured-tracing rule even when global setup fails. Normal INFO initialization
is intentionally filtered by warn/error levels.

## Alternatives and consequences

Installing the entire preferred stack would expand build/security surface with unused code.
An env-filter/config/dotenv framework is unnecessary for one setting. Long-lived service clients
and graceful shutdown wait for an actual lifecycle. No DI trait is needed for pure parsing.

Tests use supplied values or cleared child environments. A shared std-only harness bounds children
to 10 seconds, drains pipes concurrently, and kills/reaps timed-out children. Subscriber collision
is tested in an isolated test process; no parent environment mutation or production test hook.
Test deadlines are safeguards, not a production latency SLA.

See [version research](../../../specs/001-project-foundation/research.md) and the
[runtime contract](../../../specs/001-project-foundation/contracts/runtime.md). This decision
provides no financial workflow, payment verification, or production security guarantee.
