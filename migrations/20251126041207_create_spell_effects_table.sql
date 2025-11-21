CREATE TABLE spell_effects (
    spell_effect_id BLOB PRIMARY KEY
        CHECK (length(spell_effect_id) = 16),
    name            TEXT NOT NULL
        CHECK (name <> ''),
    description     TEXT NOT NULL
        CHECK (description <> '')
) STRICT, WITHOUT ROWID;
