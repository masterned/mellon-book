CREATE TABLE backgrounds_trades (
    background_id BLOB NOT NULL
        REFERENCES backgrounds(background_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(background_id) = 16),
    trade_id      BLOB NOT NULL
        REFERENCES trades(trade_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(trade_id) = 16),
    PRIMARY KEY (background_id, trade_id)
) STRICT, WITHOUT ROWID;
