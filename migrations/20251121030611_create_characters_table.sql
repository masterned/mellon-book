CREATE TABLE characters (
    character_id BLOB PRIMARY KEY
        CHECK (length(character_id) = 16),
    name         TEXT NOT NULL
        CHECK (name <> ''),
    player_id    BLOB NOT NULL
        REFERENCES players(player_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(player_id) = 16)
) STRICT, WITHOUT ROWID;
