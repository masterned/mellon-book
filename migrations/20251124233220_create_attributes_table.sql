CREATE TABLE attributes (
    attribute_id BLOB PRIMARY KEY
        CHECK (length(attribute_id) = 16),
    name         TEXT NOT NULL
        CHECK (name <> '')
) STRICT, WITHOUT ROWID;
