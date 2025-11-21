CREATE TABLE items (
    item_id BLOB PRIMARY KEY
        CHECK (length(item_id) = 16),
    name    TEXT NOT NULL
        CHECK (name <> '')
) STRICT, WITHOUT ROWID;
