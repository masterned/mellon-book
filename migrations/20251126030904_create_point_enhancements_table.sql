CREATE TABLE point_enhancements (
    point_enhancement_id BLOB    PRIMARY KEY
        CHECK (length(point_enhancement_id) = 16),
    name                 TEXT    NOT NULL
        CHECK (name <> ''),
    action_point_cost    INTEGER NOT NULL
        DEFAULT 0,
    mana_point_cost      INTEGER NOT NULL
        DEFAULT 0,
    description          TEXT    NOT NULL
        CHECK (description <> '')
) STRICT, WITHOUT ROWID;
