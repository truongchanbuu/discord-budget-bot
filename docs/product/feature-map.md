# Feature roadmap

Phase 0, pure Financial Core and owner-access policy are implemented. Business workflows remain planned
and unavailable. Each future feature requires one authoritative Spec Kit specification.

| Feature | Intended ownership | State |
| --- | --- | --- |
| Financial Core | Exact whole-VND values, virtual previews, supplied-context replay/conflict validation | Implemented; no posting or persistence |
| Identity | Owner binding, authorization, future user isolation | [Pure single-owner policy implemented](../../specs/003-owner-access/spec.md); live integration deferred |
| Discord entrypoint | Opt-in local session, trusted owner configuration and private readiness command | [Specified](../../specs/004-discord-owner-entry/spec.md); not implemented |
| Funds | Virtual fund definitions/balances and eventual CRUD | Not implemented |
| Income | Income allocation and salary policy | Not implemented |
| Payments | Awareness before handoff and explicit completion meanings | Not implemented |
| Ledger | Traceable entries, audit history, atomic idempotency | Not implemented |
| Reports | Financial calculations and eventual report images | Not implemented |
| Reminders | Notification policy and salary/reminder scheduling logic | Not implemented |

## Recommended sequence

1. Preserve the implemented Financial Core contract: exact whole-VND, no virtual overdraft,
   traceable candidate origin, replay/conflict rules and honest evidence meanings. It does not
   establish durable idempotency, actual accounting entries, or verified bank balances.
2. Preserve pure owner policy; specify trusted binding loading/authenticated interface or persistence
   boundaries for the first real use case. No provider authentication or resource lookup exists yet.
3. Add small feature slices with their own Discord presentation and relevant domain tests.
4. Specify payment awareness and handoff only after foundational financial invariants are proven.

Future interface work includes slash commands, DMs, message patterns, buttons, selects, modals,
image/attachment handling, and multiple bots/Web/mobile adapters. QR parsing/scanning, OCR, AI
agents/integrations, payment handoff, bank integration, financial reports, salary scheduling,
production financial migrations, and production deployment are outside Phase 0. None is a
Phase 0 implementation task; future specs must justify dependencies and privileges.

The MVP must not directly execute real-money transfers or collect bank passwords, OTPs, card
PINs, CVVs, or bank-session credentials. Awareness precedes handoff. Handoff is not confirmation;
manual confirmation is not bank verification. Roadmap entries never override those constraints.
See the [feature index](../ai/FEATURE_INDEX.md) for authoritative current work.
