CREATE TABLE characters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid BLOB UNIQUE,
    name TEXT NOT NULL,
    creator_id INTEGER REFERENCES players(id)
);
