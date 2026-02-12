PRAGMA foreign_keys = ON;

-- recreate the table w/ correct `NOT NULL` constraint
CREATE TABLE new_character_levels (
    character_level_id BLOB    PRIMARY KEY
        CHECK (length(character_level_id) = 16),
    character_id       BLOB    NOT NULL
        REFERENCES characters(character_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(character_id) = 16),
    level              INTEGER NOT NULL,
    UNIQUE (character_id, level)
) STRICT, WITHOUT ROWID;

-- copy data from original table to new table
INSERT INTO new_character_levels
    (character_level_id, character_id, level)
SELECT character_level_id, character_id, level
FROM character_levels
-- account for database violating foreign key constraints
WHERE character_id IS NOT NULL
AND character_id IN (
    SELECT character_id FROM characters
)
;

PRAGMA foreign_keys = OFF;

-- delete old table
DROP TABLE character_levels;

-- rename new table
ALTER TABLE new_character_levels RENAME TO character_levels;

PRAGMA foreign_keys = ON;
