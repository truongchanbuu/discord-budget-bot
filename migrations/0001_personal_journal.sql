CREATE TABLE fg_owners (
    owner TEXT PRIMARY KEY CHECK (length(btrim(owner)) > 0 AND octet_length(owner) <= 512)
);
CREATE TABLE fg_funds (
    owner TEXT NOT NULL REFERENCES fg_owners(owner),
    name TEXT NOT NULL CHECK (length(name) BETWEEN 1 AND 80),
    role TEXT NOT NULL CHECK (role IN ('spending','saving','investment')),
    PRIMARY KEY (owner,name)
);
CREATE TABLE fg_drafts (
    id BIGSERIAL PRIMARY KEY,
    owner TEXT NOT NULL REFERENCES fg_owners(owner),
    origin TEXT NOT NULL CHECK (length(origin) BETWEEN 1 AND 128),
    actor TEXT NOT NULL CHECK (length(actor) BETWEEN 1 AND 32),
    channel TEXT NOT NULL CHECK (length(channel) BETWEEN 1 AND 32),
    original_note TEXT NOT NULL CHECK (length(original_note) <= 1000),
    kind TEXT NOT NULL CHECK (kind IN ('income','expense','transfer','opening')),
    amount NUMERIC CHECK (amount > 0 AND amount <= 18446744073709551615 AND amount = trunc(amount)),
    category TEXT NOT NULL CHECK (length(category) BETWEEN 1 AND 80),
    fund TEXT NOT NULL CHECK (length(fund) BETWEEN 1 AND 80),
    destination TEXT CHECK (length(destination) BETWEEN 1 AND 80),
    entry_date DATE NOT NULL,
    note TEXT NOT NULL CHECK (length(note) <= 1000),
    revision INTEGER NOT NULL DEFAULT 0 CHECK (revision >= 0),
    expires TIMESTAMPTZ NOT NULL,
    created TIMESTAMPTZ NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending','posted','cancelled')),
    entry_id BIGINT,
    UNIQUE (owner,origin),
    UNIQUE (owner,id),
    CHECK ((status = 'posted') = (entry_id IS NOT NULL))
);
CREATE TABLE fg_entries (
    id BIGSERIAL PRIMARY KEY,
    owner TEXT NOT NULL,
    draft_id BIGINT NOT NULL,
    origin TEXT NOT NULL,
    actor TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('income','expense','transfer','opening')),
    amount NUMERIC NOT NULL CHECK (amount > 0 AND amount <= 18446744073709551615 AND amount = trunc(amount)),
    category TEXT NOT NULL CHECK (length(category) BETWEEN 1 AND 80),
    fund TEXT NOT NULL,
    destination TEXT,
    entry_date DATE NOT NULL,
    note TEXT NOT NULL CHECK (length(note) <= 1000),
    created TIMESTAMPTZ NOT NULL,
    confirmed TIMESTAMPTZ NOT NULL,
    confirmed_actor TEXT NOT NULL CHECK (length(confirmed_actor) BETWEEN 1 AND 32),
    UNIQUE (owner,origin),
    UNIQUE (draft_id),
    UNIQUE (owner,id),
    FOREIGN KEY (owner,draft_id) REFERENCES fg_drafts(owner,id),
    FOREIGN KEY (owner,fund) REFERENCES fg_funds(owner,name),
    FOREIGN KEY (owner,destination) REFERENCES fg_funds(owner,name),
    CHECK ((kind = 'transfer') = (destination IS NOT NULL)),
    CHECK (destination IS NULL OR destination <> fund)
);
ALTER TABLE fg_drafts ADD FOREIGN KEY (owner,entry_id) REFERENCES fg_entries(owner,id);
CREATE INDEX fg_entries_history ON fg_entries(owner,entry_date,id);
CREATE FUNCTION fg_immutable_entry() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'journal entries are immutable';
END;
$$;
CREATE TRIGGER fg_entry_immutable BEFORE UPDATE OR DELETE ON fg_entries
FOR EACH ROW EXECUTE FUNCTION fg_immutable_entry();
