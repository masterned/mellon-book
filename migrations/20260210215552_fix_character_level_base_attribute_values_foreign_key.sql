PRAGMA foreign_keys = ON;

-- recreate the table w/ correct FKs
CREATE TABLE new_character_level_base_attribute_values (
    character_level_id BLOB    NOT NULL
        REFERENCES character_levels(character_level_id)
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

-- copy data from original table to new table
INSERT INTO new_character_level_base_attribute_values
    (character_level_id, attribute_id, value)
SELECT character_level_id, attribute_id, value
FROM character_level_base_attribute_values;

-- drop dependent view if exists
DROP VIEW IF EXISTS character_level_attributes;

PRAGMA foreign_keys = OFF;

-- delete old table
DROP TABLE character_level_base_attribute_values;

-- rename new table
ALTER TABLE new_character_level_base_attribute_values RENAME TO character_level_base_attribute_values;

PRAGMA foreign_keys = ON;

