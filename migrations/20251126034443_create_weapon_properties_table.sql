CREATE TABLE weapon_properties (
    weapon_property_id           BLOB    PRIMARY KEY
        CHECK (length(weapon_property_id) = 16),
    name                         TEXT    NOT NULL
        CHECK (name <> ''),
    description                  TEXT    NOT NULL
        CHECK (description <> ''),
    cost                         INTEGER NOT NULL DEFAULT 1,
    required_weapon_property_id  BLOB        NULL DEFAULT NULL
        REFERENCES weapon_properties(weapon_property_id)
            ON DELETE SET DEFAULT
            ON UPDATE CASCADE
        CHECK ((length(required_weapon_property_id) = 16)
            AND (weapon_property_id <> required_weapon_property_id))
) STRICT, WITHOUT ROWID;
