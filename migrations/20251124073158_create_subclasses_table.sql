CREATE TABLE subclasses (
    subclass_id BLOB PRIMARY KEY
        CHECK (length(subclass_id) = 16),
    name        TEXT NOT NULL
        CHECK (name <> '')
) STRICT, WITHOUT ROWID;
