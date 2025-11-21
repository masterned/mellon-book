CREATE TABLE ancestry_traits_character_levels (
    ancestry_trait_id  BLOB NOT NULL
        REFERENCES ancestry_traits(ancestry_trait_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(ancestry_trait_id) = 16),
    character_level_id BLOB NOT NULL
        REFERENCES character_levels(character_level_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(character_level_id) = 16),
    PRIMARY KEY (ancestry_trait_id, character_level_id)
) STRICT, WITHOUT ROWID;
