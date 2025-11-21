CREATE TABLE classes (
    class_id BLOB PRIMARY KEY
        CHECK (length(class_id) = 16),
    name     TEXT NOT NULL
        CHECK (name <> '')
) STRICT, WITHOUT ROWID;
