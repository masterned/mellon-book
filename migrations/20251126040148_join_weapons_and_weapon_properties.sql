CREATE TABLE weapons_weapon_properties (
    weapon_id           BLOB NOT NULL
        REFERENCES weapons(weapon_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(weapon_id) = 16),
    weapon_property_id  BLOB NOT NULL
        REFERENCES weapon_properties(weapon_property_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(weapon_property_id) = 16),
    PRIMARY KEY (weapon_id, weapon_property_id)
) STRICT, WITHOUT ROWID;
