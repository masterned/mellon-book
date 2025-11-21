CREATE TABLE character_levels (
    character_level_id BLOB PRIMARY KEY
        CHECK (length(character_level_id) = 16),
    character_id       BLOB
        REFERENCES characters(character_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(character_id) = 16),
    level              INTEGER NOT NULL,
    UNIQUE (character_id, level)
) STRICT, WITHOUT ROWID;
