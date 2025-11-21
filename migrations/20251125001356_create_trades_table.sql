CREATE TABLE trades (
    trade_id BLOB PRIMARY KEY
        CHECK (length(trade_id) = 16),
    name     TEXT NOT NULL
        CHECK (name <> '')
) STRICT, WITHOUT ROWID;
