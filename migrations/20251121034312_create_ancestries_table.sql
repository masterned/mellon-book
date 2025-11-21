CREATE TABLE ancestries (
    ancestry_id BLOB PRIMARY KEY
        CHECK (length(ancestry_id) = 16),
    name        TEXT NOT NULL
        CHECK (name <> '')
) STRICT, WITHOUT ROWID;
