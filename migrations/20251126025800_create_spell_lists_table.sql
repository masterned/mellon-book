CREATE TABLE spell_lists (
    spell_list_id BLOB PRIMARY KEY
        CHECK (length(spell_list_id) = 16),
    name          TEXT NOT NULL
        CHECK (name <> '')
) STRICT, WITHOUT ROWID;
