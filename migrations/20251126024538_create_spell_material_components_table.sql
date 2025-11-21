CREATE TABLE spell_material_components (
    spell_id BLOB NOT NULL
        REFERENCES spells(spell_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(spell_id) = 16),
    item_id  BLOB NOT NULL
        REFERENCES items(item_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(item_id) = 16),
    consumed INTEGER NOT NULL
        DEFAULT FALSE
        CHECK (consumed IN (1, 2)),
    PRIMARY KEY (spell_id, item_id)
) STRICT, WITHOUT ROWID;
