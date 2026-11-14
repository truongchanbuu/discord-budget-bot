CREATE TABLE fg_finance_spaces (
    id BIGSERIAL PRIMARY KEY CHECK (id > 0),
    guild_id TEXT NOT NULL CHECK (guild_id ~ '^[1-9][0-9]{0,19}$' AND guild_id::numeric <= 18446744073709551615),
    member_id TEXT NOT NULL CHECK (member_id ~ '^[1-9][0-9]{0,19}$' AND member_id::numeric <= 18446744073709551615),
    journal_owner TEXT NOT NULL UNIQUE REFERENCES fg_owners(owner),
    legacy_binding BOOLEAN NOT NULL DEFAULT FALSE,
    owner_joined_at TEXT CHECK (owner_joined_at IS NULL OR (owner_joined_at ~ '^[0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])T([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9][.][0-9]{9}Z$' AND owner_joined_at::timestamptz IS NOT NULL)),
    generation TEXT NOT NULL CHECK (generation ~ '^[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}$'),
    channel_id TEXT UNIQUE CHECK (channel_id ~ '^[1-9][0-9]{0,19}$' AND channel_id::numeric <= 18446744073709551615),
    category_id TEXT NOT NULL CHECK (category_id ~ '^[1-9][0-9]{0,19}$' AND category_id::numeric <= 18446744073709551615),
    state TEXT NOT NULL CHECK (state IN ('prepared','creating','uncertain','verifying','ready','access_pending','suspended','retired')),
    revision BIGINT NOT NULL DEFAULT 0 CHECK (revision >= 0),
    current_operation_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    UNIQUE (guild_id,member_id),
    UNIQUE (id,generation),
    CHECK (state <> 'ready' OR (channel_id IS NOT NULL AND owner_joined_at IS NOT NULL))
);
CREATE UNIQUE INDEX fg_space_legacy ON fg_finance_spaces(guild_id) WHERE legacy_binding;
CREATE TABLE fg_space_operations (
    id TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text CHECK (id ~ '^[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}$'),
    space_id BIGINT NOT NULL REFERENCES fg_finance_spaces(id),
    kind TEXT NOT NULL CHECK (kind IN ('create','grant_viewer','revoke_viewer')),
    source TEXT NOT NULL CHECK (length(source) BETWEEN 1 AND 128 AND source ~ '^[!-~]+$'),
    actor TEXT NOT NULL CHECK (actor ~ '^[1-9][0-9]{0,19}$' AND actor::numeric <= 18446744073709551615),
    generation TEXT NOT NULL CHECK (generation ~ '^[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}$'),
    expected_revision BIGINT NOT NULL CHECK (expected_revision >= 0),
    target_member TEXT CHECK (target_member ~ '^[1-9][0-9]{0,19}$' AND target_member::numeric <= 18446744073709551615),
    target_joined_at TEXT CHECK (target_joined_at IS NULL OR (target_joined_at ~ '^[0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])T([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9][.][0-9]{9}Z$' AND target_joined_at::timestamptz IS NOT NULL)),
    desired_access TEXT CHECK (desired_access IN ('viewer','absent')),
    phase TEXT NOT NULL CHECK (phase IN ('prepared','dispatched','uncertain','verifying','confirmed','failed','retired')),
    returned_channel_id TEXT CHECK (returned_channel_id ~ '^[1-9][0-9]{0,19}$' AND returned_channel_id::numeric <= 18446744073709551615),
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    dispatched_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    outcome TEXT CHECK (outcome IN ('confirmed','cancelled','expired','denied','permission','timeout','uncertain','retired')),
    UNIQUE (space_id,source),
    UNIQUE (space_id,id),
    CHECK ((kind='create' AND target_member IS NULL AND target_joined_at IS NULL AND desired_access IS NULL)
        OR (kind='grant_viewer' AND target_member IS NOT NULL AND target_joined_at IS NOT NULL AND desired_access='viewer')
        OR (kind='revoke_viewer' AND target_member IS NOT NULL AND target_joined_at IS NOT NULL AND desired_access='absent')),
    CHECK (target_member IS NULL OR target_member <> actor),
    CHECK (phase <> 'prepared' OR dispatched_at IS NULL),
    CHECK ((phase IN ('confirmed','failed','retired')) = (completed_at IS NOT NULL)),
    CHECK (dispatched_at IS NOT NULL OR phase IN ('prepared','failed','retired'))
);
CREATE UNIQUE INDEX fg_space_pending_operation ON fg_space_operations(space_id)
    WHERE phase NOT IN ('confirmed','failed','retired');
ALTER TABLE fg_finance_spaces ADD CONSTRAINT fg_space_pending_reference
    FOREIGN KEY (id,current_operation_id) REFERENCES fg_space_operations(space_id,id) DEFERRABLE INITIALLY DEFERRED;
CREATE TABLE fg_space_viewers (
    space_id BIGINT NOT NULL REFERENCES fg_finance_spaces(id),
    generation TEXT NOT NULL CHECK (generation ~ '^[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}$'),
    member_id TEXT NOT NULL CHECK (member_id ~ '^[1-9][0-9]{0,19}$' AND member_id::numeric <= 18446744073709551615),
    member_joined_at TEXT NOT NULL CHECK (member_joined_at ~ '^[0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])T([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9][.][0-9]{9}Z$' AND member_joined_at::timestamptz IS NOT NULL),
    desired_state TEXT NOT NULL CHECK (desired_state IN ('viewer','revoked')),
    confirmed_state TEXT NOT NULL CHECK (confirmed_state IN ('pending','viewer','absent','uncertain')),
    revision BIGINT NOT NULL DEFAULT 0 CHECK (revision >= 0),
    operation_id TEXT NOT NULL,
    granted_by TEXT NOT NULL CHECK (granted_by ~ '^[1-9][0-9]{0,19}$' AND granted_by::numeric <= 18446744073709551615),
    changed_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (space_id,generation,member_id),
    FOREIGN KEY (space_id,operation_id) REFERENCES fg_space_operations(space_id,id),
    CHECK (member_id <> granted_by)
);
CREATE TABLE fg_space_access_events (
    id BIGSERIAL PRIMARY KEY,
    space_id BIGINT NOT NULL REFERENCES fg_finance_spaces(id),
    operation_id TEXT,
    source TEXT NOT NULL CHECK (length(source) BETWEEN 1 AND 128 AND source ~ '^[!-~]+$'),
    actor TEXT NOT NULL CHECK (actor ~ '^[1-9][0-9]{0,19}$' AND actor::numeric <= 18446744073709551615),
    action TEXT NOT NULL CHECK (action IN ('reserved','verified','granted','revoked','invalidated','retired','recovered','expired','dispatched','uncertain')),
    outcome TEXT NOT NULL CHECK (outcome IN ('confirmed','cancelled','expired','denied','permission','timeout','uncertain','retired','pending')),
    transition_revision BIGINT NOT NULL CHECK (transition_revision >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    UNIQUE (space_id,transition_revision),
    FOREIGN KEY (space_id,operation_id) REFERENCES fg_space_operations(space_id,id)
);
CREATE FUNCTION fg_space_immutable_audit() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'space audit is append only';
END;
$$;
CREATE TRIGGER fg_space_audit_immutable BEFORE UPDATE OR DELETE ON fg_space_access_events
    FOR EACH ROW EXECUTE FUNCTION fg_space_immutable_audit();
CREATE FUNCTION fg_space_immutable_owner() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF (NEW.guild_id,NEW.member_id,NEW.journal_owner,NEW.legacy_binding)
        IS DISTINCT FROM (OLD.guild_id,OLD.member_id,OLD.journal_owner,OLD.legacy_binding) THEN
        RAISE EXCEPTION 'space ownership is immutable';
    END IF;
    RETURN NEW;
END;
$$;
CREATE TRIGGER fg_space_owner_immutable BEFORE UPDATE ON fg_finance_spaces
    FOR EACH ROW EXECUTE FUNCTION fg_space_immutable_owner();
CREATE FUNCTION fg_space_immutable_operation() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF (NEW.space_id,NEW.kind,NEW.source,NEW.actor,NEW.generation,NEW.expected_revision,
        NEW.target_member,NEW.target_joined_at,NEW.desired_access)
        IS DISTINCT FROM (OLD.space_id,OLD.kind,OLD.source,OLD.actor,OLD.generation,OLD.expected_revision,
        OLD.target_member,OLD.target_joined_at,OLD.desired_access) THEN
        RAISE EXCEPTION 'space operation identity is immutable';
    END IF;
    IF OLD.returned_channel_id IS NOT NULL AND NEW.returned_channel_id IS DISTINCT FROM OLD.returned_channel_id THEN
        RAISE EXCEPTION 'space operation candidate is immutable';
    END IF;
    RETURN NEW;
END;
$$;
CREATE TRIGGER fg_space_operation_immutable BEFORE UPDATE ON fg_space_operations
    FOR EACH ROW EXECUTE FUNCTION fg_space_immutable_operation();
ALTER TABLE fg_finance_spaces ENABLE ROW LEVEL SECURITY;
ALTER TABLE fg_space_operations ENABLE ROW LEVEL SECURITY;
ALTER TABLE fg_space_viewers ENABLE ROW LEVEL SECURITY;
ALTER TABLE fg_space_access_events ENABLE ROW LEVEL SECURITY;
REVOKE ALL ON fg_finance_spaces,fg_space_operations,fg_space_viewers,fg_space_access_events FROM PUBLIC;
REVOKE ALL ON SEQUENCE fg_finance_spaces_id_seq,fg_space_access_events_id_seq FROM PUBLIC;
REVOKE ALL ON FUNCTION fg_space_immutable_audit(),fg_space_immutable_owner(),fg_space_immutable_operation() FROM PUBLIC;
DO $$
DECLARE api_role TEXT;
BEGIN
    FOREACH api_role IN ARRAY ARRAY['anon','authenticated','service_role'] LOOP
        IF EXISTS (SELECT FROM pg_roles WHERE rolname=api_role) THEN
            EXECUTE format('REVOKE ALL ON fg_finance_spaces,fg_space_operations,fg_space_viewers,fg_space_access_events FROM %I',api_role);
            EXECUTE format('REVOKE ALL ON SEQUENCE fg_finance_spaces_id_seq,fg_space_access_events_id_seq FROM %I',api_role);
            EXECUTE format('REVOKE ALL ON FUNCTION fg_space_immutable_audit(),fg_space_immutable_owner(),fg_space_immutable_operation() FROM %I',api_role);
        END IF;
    END LOOP;
END;
$$;
