CREATE TABLE fg_finance_categories (
    guild_id TEXT PRIMARY KEY CHECK (guild_id ~ '^[1-9][0-9]{0,19}$' AND guild_id::numeric <= 18446744073709551615),
    category_id TEXT UNIQUE CHECK (category_id ~ '^[1-9][0-9]{0,19}$' AND category_id::numeric <= 18446744073709551615),
    reserved_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
ALTER TABLE fg_finance_categories ENABLE ROW LEVEL SECURITY;
REVOKE ALL ON fg_finance_categories FROM PUBLIC;
DO $$
DECLARE api_role TEXT;
BEGIN
    FOREACH api_role IN ARRAY ARRAY['anon','authenticated','service_role'] LOOP
        IF EXISTS (SELECT FROM pg_roles WHERE rolname = api_role) THEN
            EXECUTE format('REVOKE ALL ON fg_finance_categories FROM %I', api_role);
        END IF;
    END LOOP;
END $$;
