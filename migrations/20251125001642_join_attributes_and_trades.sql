CREATE TABLE attributes_trades (
    attribute_id BLOB NOT NULL
        REFERENCES attributes(attribute_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(attribute_id) = 16),
    trade_id     BLOB NOT NULL
        REFERENCES trades(trade_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(trade_id) = 16),
    PRIMARY KEY (attribute_id, trade_id)
) STRICT, WITHOUT ROWID;
