CREATE TABLE masteries (
    mastery_id BLOB PRIMARY KEY
        CHECK (length(mastery_id) = 16),
    name       TEXT NOT NULL
        CHECK (name <> ''),
    bonus      INTEGER NOT NULL
) STRICT, WITHOUT ROWID;
