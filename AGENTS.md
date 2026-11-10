# FundGuard repository instructions

## Start every session

1. Read `.specify/memory/constitution.md`.
2. Read `docs/product/overview.md`.
3. Read `docs/ai/SESSION_HANDOFF.md`.
4. Find the active feature in `docs/ai/FEATURE_INDEX.md`; read its spec.md, plan.md, tasks.md,
   relevant research/contracts, and current checklist. Use tracked index/handoff discovery if the
   locally ignored `.specify/feature.json` selector is absent; do not invent a new feature.
5. Inspect Git status and related existing code before changes. Preserve unrelated work.
6. Read `docs/architecture/overview.md`, `docs/development/coding-standards.md`, relevant ADRs,
   and security guidance; follow feature boundaries and current authorization.

## During work

Implement only agreed scope in dependency order. Use installed Spec Kit workflows and one
specification per feature. Keep generated skills/scripts/templates/workflows unchanged unless a
concrete tooling defect requires correction. Place temporary tool output outside the repository.
Do not introduce unused traits, generic frameworks, empty future modules, or business features to
make a folder tree look complete. Domain/application must remain independent of Discord/SQLx.
No floats for financial amounts, prohibited banking credentials, direct MVP transfers, or secret
logging. Respect minimal privileges and explicit identity/authorization boundaries.

## Complete every session

7. Run relevant verification; for Rust changes use cargo fmt --all -- --check,
   cargo clippy --all-targets -- -D warnings, cargo test, and cargo check. Review the diff,
   dependencies, visibility, and security implications. Never claim a check passed without evidence.
8. Update the active tasks.md only after verifying each completed task; keep partial/blocked work
   unchecked and record concise evidence. Do not duplicate its detailed checklist elsewhere.
9. Update `docs/ai/SESSION_HANDOFF.md` with phase, active spec/tasks, completed and pending work,
   blockers, decisions, verification results/limits, and next action.
10. Record important architecture changes in `docs/architecture/decisions/` and reconcile affected
    artifacts. Do not rely on chat history as the only project state.
11. Report completed, partial, blocked, and unverified work distinctly. CI configuration is not a
    hosted passing run. Do not deploy without an explicit task instruction.

Phase 0 contains initialization/tooling only. Financial Core invariants must be specified before
Discord payment features. Read the current handoff for actual implementation status.
