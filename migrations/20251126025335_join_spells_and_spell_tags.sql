CREATE TABLE spells_spell_tags (
    spell_id BLOB NOT NULL
        REFERENCES spells(spell_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(spell_id) = 16),
    spell_tag_id BLOB NOT NULL
        REFERENCES spell_tags(spell_tag_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(spell_tag_id) = 16),
    PRIMARY KEY (spell_id, spell_tag_id)
) STRICT, WITHOUT ROWID;
