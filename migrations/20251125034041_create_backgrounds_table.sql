CREATE TABLE backgrounds (
    background_id BLOB PRIMARY KEY
        CHECK (length(background_id) = 16),
    name          TEXT NOT NULL
        CHECK (name <> '')
) STRICT, WITHOUT ROWID;
