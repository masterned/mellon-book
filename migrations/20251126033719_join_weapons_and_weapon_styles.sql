CREATE TABLE weapons_weapon_styles (
    weapon_id       BLOB NOT NULL
        REFERENCES weapons(weapon_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(weapon_id) = 16),
    weapon_style_id BLOB NOT NULL
        REFERENCES weapon_styles(weapon_style_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(weapon_style_id) = 16),
    PRIMARY KEY (weapon_id, weapon_style_id)
) STRICT, WITHOUT ROWID;
