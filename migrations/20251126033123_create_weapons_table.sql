CREATE TABLE weapons (
    weapon_id BLOB PRIMARY KEY
        REFERENCES items(item_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(weapon_id) = 16),
    type      TEXT NOT NULL
        CHECK ((type <> '')
            AND (type IN ('Melee', 'Ranged')))
) STRICT, WITHOUT ROWID;
