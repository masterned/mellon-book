CREATE TABLE players (
    player_id BLOB PRIMARY KEY
        CHECK (length(player_id) = 16),
    name      TEXT NOT NULL
        CHECK (name <> '')
) STRICT, WITHOUT ROWID;
