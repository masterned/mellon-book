CREATE TABLE character_level_base_attribute_values (
    character_level_id BLOB    NOT NULL
        REFERENCES character_levels(character_level)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(character_level_id) = 16),
    attribute_id       BLOB    NOT NULL
        REFERENCES attributes(attribute_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(attribute_id) = 16),
    value              INTEGER NOT NULL,
    PRIMARY KEY (character_level_id, attribute_id)
) STRICT, WITHOUT ROWID;
