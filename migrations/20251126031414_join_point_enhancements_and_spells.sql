CREATE TABLE point_enhancements_spells (
    point_enhancement_id BLOB NOT NULL
        REFERENCES point_enhancements(point_enhancement_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(point_enhancement_id) = 16),
    spell_id BLOB NOT NULL
        REFERENCES spells(spell_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(spell_id) = 16),
    PRIMARY KEY (point_enhancement_id, spell_id)
) STRICT, WITHOUT ROWID;
