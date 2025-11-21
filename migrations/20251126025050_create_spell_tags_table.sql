CREATE TABLE spell_tags (
    spell_tag_id BLOB PRIMARY KEY
        CHECK (length(spell_tag_id) = 16),
    name         TEXT NOT NULL
        CHECK (name <> '')
) STRICT, WITHOUT ROWID;
