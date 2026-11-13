# Feature roadmap

Phase 0, pure Financial Core, owner-access policy and opt-in Discord readiness are implemented. The first private manual-journal slice is implemented; other workflows remain planned. Each future feature requires one authoritative Spec Kit specification.

| Feature | Intended ownership | State |
| --- | --- | --- |
| Financial Core | Exact whole-VND values, virtual previews, supplied-context replay/conflict validation | Implemented; no posting or persistence |
| Identity | Owner binding, authorization, future user isolation | [Pure single-owner policy implemented](../../specs/003-owner-access/spec.md); trusted environment binding and readiness adapter implemented; live private owner reply confirmed; other-account/disabled denial pending |
| Discord entrypoint | Opt-in local session, trusted owner configuration and private readiness command | [Implemented](../../specs/004-discord-owner-entry/spec.md); local tests and live auth/registration/shutdown verified; owner reply confirmed; denial/recovery checks pending |
| Funds | Virtual register, opening and transfers | Minimal register in ledger implemented; rename/delete/full settings deferred |
| Income | Received salary/freelance/bonus and allocations | Manual receipt/transfers implemented; recurring/expected-income automation deferred |
| Payments | Awareness before handoff and explicit completion meanings | Not implemented |
| Ledger | Traceable entries, audit history, atomic idempotency | [Conversational journal implemented](../../specs/005-conversational-finance/spec.md); no committed edits/deletion |
| Reports | Monthly cash flow/category/fund totals | Monthly text journal view implemented; images/export deferred |
| Reminders | Notification policy and salary/reminder scheduling logic | Not implemented |

## Recommended sequence

Preserve exact core/owner contracts and the first journal's durable idempotency, immutable audit,
historical nonnegative balances and private confirmations. Verify the financial DM flow with synthetic
local data before real personal use. Specify immutable correction/reversal and richer administration
before expanding financial workflows. Payments/awareness/handoff remain separate future specifications.

Future interface work beyond implemented readiness and finance DMs includes broader message patterns,
additional buttons/selects/modals,
image/attachment handling, and multiple bots/Web/mobile adapters. QR parsing/scanning, OCR, AI
agents/integrations, payment handoff, bank integration, financial reports, salary scheduling,
production financial migrations, and production deployment are outside Phase 0. None is a
Phase 0 implementation task; future specs must justify dependencies and privileges.

The MVP must not directly execute real-money transfers or collect bank passwords, OTPs, card
PINs, CVVs, or bank-session credentials. Awareness precedes handoff. Handoff is not confirmation;
manual confirmation is not bank verification. Roadmap entries never override those constraints.
See the [feature index](../ai/FEATURE_INDEX.md) for authoritative current work.
