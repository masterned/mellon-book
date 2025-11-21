CREATE TABLE character_levels_subclasses (
    character_level_id BLOB NOT NULL
        REFERENCES character_levels(character_level_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(character_level_id) = 16),
    subclass_id        BLOB NOT NULL
        REFERENCES subclasses(subclass_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(subclass_id) = 16),
    PRIMARY KEY (character_level_id, subclass_id)
) STRICT, WITHOUT ROWID;
