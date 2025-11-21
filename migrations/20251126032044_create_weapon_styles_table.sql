CREATE TABLE weapon_styles (
    weapon_style_id BLOB PRIMARY KEY
        CHECK (length(weapon_style_id) = 16),
    name            TEXT NOT NULL
        CHECK (name <> ''),
    description     TEXT NOT NULL
        CHECK (description <> ''),
    damage_type     TEXT NOT NULL
        CHECK (damage_type IN ('Bludgeoning', 'Piercing', 'Slashing'))
) STRICT, WITHOUT ROWID;
