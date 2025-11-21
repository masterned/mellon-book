CREATE TABLE ancestries_character_levels (
    ancestry_id BLOB
        REFERENCES ancestries(ancestry_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(ancestry_id) = 16),
    character_level_id BLOB
        REFERENCES character_levels(character_level_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(character_level_id) = 16),
    PRIMARY KEY (ancestry_id, character_level_id)
) STRICT, WITHOUT ROWID;
