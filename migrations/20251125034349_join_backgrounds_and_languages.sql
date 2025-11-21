CREATE TABLE backgrounds_languages (
    background_id BLOB    NOT NULL
        REFERENCES backgrounds(background_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(background_id) = 16),
    language_id   BLOB    NOT NULL
        REFERENCES languages(language_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(language_id) = 16),
    fluency       INTEGER NOT NULL
        CHECK (fluency IN (1, 2)),
    PRIMARY KEY (background_id, language_id)
) STRICT, WITHOUT ROWID;
