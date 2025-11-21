CREATE TABLE spell_schools (
    spell_school_id BLOB PRIMARY KEY
        CHECK (length(spell_school_id) = 16),
    name            TEXT NOT NULL
        CHECK (name <> '')
) STRICT, WITHOUT ROWID;
