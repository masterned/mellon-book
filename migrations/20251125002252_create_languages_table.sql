CREATE TABLE languages (
    language_id BLOB PRIMARY KEY
        CHECK (length(language_id) = 16),
    name        TEXT NOT NULL
        CHECK (name <> '')
) STRICT, WITHOUT ROWID;
