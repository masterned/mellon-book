CREATE TABLE skills (
    skill_id     BLOB PRIMARY KEY
        CHECK (length(skill_id) = 16),
    name         TEXT NOT NULL
        CHECK (name <> ''),
    attribute_id BLOB NOT NULL
        REFERENCES attributes(attribute_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(attribute_id) = 16)
) STRICT, WITHOUT ROWID;
