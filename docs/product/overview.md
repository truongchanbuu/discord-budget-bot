# FundGuard product overview

FundGuard is a Discord-first personal financial management and payment-awareness application.
It aims to make financial intent and consequences visible before a person hands off a payment,
while keeping virtual budgets distinct from verified bank information.

The initial product is personal-use and single-owner. Multi-user support and additional Discord,
Web, or mobile interfaces may be introduced later through explicit identity/authorization and
isolation requirements. A single-owner assumption never makes arbitrary Discord senders trusted.

Phase 0 implements local configuration, structured diagnostics, initialization, and verification.
Financial Core now adds pure whole-VND values and domain invariant validation through a library
facade; it does not apply or store financial changes. Identity adds pure configured-owner resolution
and owner-scope checks. The opt-in Discord adapter loads trusted environment bindings and maps SDK
actors to a single private readiness command. Live authentication/registration/shutdown passed smoke
testing, and the maintainer confirmed a private owner readiness reply. Other-account/disabled denial
and network recovery remain unverified. Authoritative resource lookup, persistence, fund management, financial
ledger, payment workflows, reports and reminders remain future work. No banking credentials are collected and no real-money transfers
are executed. Production deployment is not part of this phase.

Financial correctness, auditability, security, readable Rust, and maintainability take priority
over speculative performance optimization. See the [feature roadmap](feature-map.md),
[architecture](../architecture/overview.md), and [security baseline](../security/security-baseline.md).
Feature-specific requirements remain authoritative in the [active Spec Kit specification](../../specs/004-discord-owner-entry/spec.md).
