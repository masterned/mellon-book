CREATE TABLE character_levels_classes (
    character_level_id BLOB NOT NULL
        REFERENCES character_levels(character_level_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(character_level_id) = 16),
    class_id           BLOB NOT NULL
        REFERENCES classes(class_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(class_id) = 16),
    PRIMARY KEY (character_level_id, class_id)
) STRICT, WITHOUT ROWID;
