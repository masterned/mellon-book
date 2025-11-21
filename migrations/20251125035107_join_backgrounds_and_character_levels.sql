CREATE TABLE backgrounds_character_levels (
    background_id      BLOB NOT NULL
        REFERENCES backgrounds(background_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(background_id) = 16),
    character_level_id BLOB NOT NULL
        REFERENCES character_levels(character_level_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(character_level_id) = 16),
    PRIMARY KEY (background_id, character_level_id)
) STRICT, WITHOUT ROWID;
