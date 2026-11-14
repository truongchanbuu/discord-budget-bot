ALTER TABLE public.fg_owners ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.fg_funds ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.fg_drafts ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.fg_entries ENABLE ROW LEVEL SECURITY;
ALTER TABLE public._sqlx_migrations ENABLE ROW LEVEL SECURITY;

REVOKE ALL ON TABLE public.fg_owners, public.fg_funds, public.fg_drafts,
    public.fg_entries, public._sqlx_migrations FROM PUBLIC;
REVOKE ALL ON SEQUENCE public.fg_drafts_id_seq, public.fg_entries_id_seq FROM PUBLIC;
REVOKE ALL ON FUNCTION public.fg_immutable_entry() FROM PUBLIC;

DO $$
BEGIN
    IF EXISTS (SELECT FROM pg_roles WHERE rolname = 'anon') THEN
        REVOKE ALL ON TABLE public.fg_owners, public.fg_funds, public.fg_drafts,
            public.fg_entries, public._sqlx_migrations FROM anon;
        REVOKE ALL ON SEQUENCE public.fg_drafts_id_seq, public.fg_entries_id_seq FROM anon;
        REVOKE ALL ON FUNCTION public.fg_immutable_entry() FROM anon;
    END IF;
    IF EXISTS (SELECT FROM pg_roles WHERE rolname = 'authenticated') THEN
        REVOKE ALL ON TABLE public.fg_owners, public.fg_funds, public.fg_drafts,
            public.fg_entries, public._sqlx_migrations FROM authenticated;
        REVOKE ALL ON SEQUENCE public.fg_drafts_id_seq, public.fg_entries_id_seq FROM authenticated;
        REVOKE ALL ON FUNCTION public.fg_immutable_entry() FROM authenticated;
    END IF;
    IF EXISTS (SELECT FROM pg_roles WHERE rolname = 'service_role') THEN
        REVOKE ALL ON TABLE public.fg_owners, public.fg_funds, public.fg_drafts,
            public.fg_entries, public._sqlx_migrations FROM service_role;
        REVOKE ALL ON SEQUENCE public.fg_drafts_id_seq, public.fg_entries_id_seq FROM service_role;
        REVOKE ALL ON FUNCTION public.fg_immutable_entry() FROM service_role;
    END IF;
END;
$$;
