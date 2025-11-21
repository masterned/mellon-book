CREATE TABLE spells_spell_effects (
    spell_id      BLOB NOT NULL
        REFERENCES spells(spell_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(spell_id) = 16),
  spell_effect_id BLOB NOT NULL
        REFERENCES spell_effects(spell_effect_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(spell_effect_id) = 16),
  PRIMARY KEY (spell_id, spell_effect_id)
) STRICT, WITHOUT ROWID;
