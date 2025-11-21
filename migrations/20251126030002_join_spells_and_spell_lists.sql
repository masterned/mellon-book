CREATE TABLE spells_spell_lists (
    spell_id BLOB NOT NULL
        REFERENCES spells(spell_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(spell_id) = 16),
    spell_list_id BLOB NOT NULL
        REFERENCES spell_lists(spell_list_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(spell_list_id) = 16),
    PRIMARY KEY (spell_id, spell_list_id)
) STRICT, WITHOUT ROWID;
