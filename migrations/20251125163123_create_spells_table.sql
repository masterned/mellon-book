CREATE TABLE spells (
    spell_id          BLOB    PRIMARY KEY
        CHECK (length(spell_id) = 16),
    name              TEXT    NOT NULL
        CHECK (name <> ''),
    spell_school_id   BLOB    NOT NULL
        REFERENCES spell_schools(spell_school_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(spell_school_id) = 16),
    has_verbal        INTEGER NOT NULL
        CHECK (has_verbal IN (0, 1))   DEFAULT FALSE,
    has_somatic       INTEGER NOT NULL
        CHECK (has_somatic IN (0, 1))  DEFAULT FALSE,
    action_point_cost INTEGER NOT NULL DEFAULT 1,
    mana_point_cost   INTEGER NOT NULL DEFAULT 0,
    range_kind        TEXT    NOT NULL,
    range_value       INTEGER     NULL,
    duration_kind     TEXT    NOT NULL,
    duration_value    INTEGER     NULL,
    sustained         INTEGER NOT NULL
        CHECK (sustained IN (0, 1))    DEFAULT FALSE,
    description       TEXT        NULL,
    CHECK ((`range_kind` IN ('Self', 'Touch') AND `range_value` IS NULL)
        OR (`range_kind` = 'Spaces' AND `range_value` IS NOT NULL)),
    CHECK ((`duration_kind` = 'Instant' AND `duration_value` IS NULL)
        OR (`duration_kind` IN ('Minute', 'Hour', 'Round') AND `duration_value` IS NOT NULL))
) STRICT, WITHOUT ROWID;
