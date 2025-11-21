CREATE TABLE ancestries_ancestry_traits (
    ancestry_id       BLOB
        REFERENCES ancestries(ancestry_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(ancestry_id) = 16),
    ancestry_trait_id BLOB
        REFERENCES ancestry_traits(ancestry_trait_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(ancestry_trait_id) = 16),
    expanded          INTEGER NOT NULL DEFAULT FALSE,
    PRIMARY KEY (ancestry_id, ancestry_trait_id)
) STRICT, WITHOUT ROWID;
