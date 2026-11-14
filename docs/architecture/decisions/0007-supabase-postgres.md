# ADR 0007: Supabase-managed PostgreSQL for application storage

Date: 2026-11-14. Status: Accepted by maintainer's request to replace self-hosting.

Supabase hosts PostgreSQL; retain SQLx and the existing journal port, schema, owner locks and atomic
transactions. No Supabase SDK/REST rewrite, Auth mapping or service API key is needed. The application
uses FUNDGUARD_FINANCE_DATABASE_URL as before. Domain/application and financial rules do not change.

Use a direct connection, or shared session pooler for IPv4-only networks, on port5432. Session mode
preserves prepared statements and migration advisory locks. Known Supabase transaction6543 endpoints
are rejected before connection. Remote TLS is required; optional verify-full plus the project's CA
adds certificate/hostname verification. Require mode alone encrypts without that identity assurance.

Disable Data API before first initialization in a dedicated development project. Migration0002 enables
default-deny RLS and revokes table/sequence/function access from PUBLIC and existing anon,
authenticated and service_role roles, including SQLx metadata. Only owned application objects change.
Backend uses the trusted migration/table-owner credential; owner bypasses RLS. This is a single-owner
backend boundary, not tenant RLS or a least-privilege production runtime-role design. Never expose DB
credentials to Discord clients. Production migration/runtime separation needs a separate design.

Keep applied migration0001 unchanged; upgrade in place with0002. No automatic dump/restore or deleting
local data: an existing database needs an explicitly reviewed backup/restore operation before cutover.
CI/local transaction tests continue on disposable PostgreSQL; they do not require a Supabase account.
No cloud project creation, paid service, deployment or live data migration is part of this change.

Sources: [Connections](https://supabase.com/docs/guides/database/connecting-to-postgres),
[API security](https://supabase.com/docs/guides/api/securing-your-api).
