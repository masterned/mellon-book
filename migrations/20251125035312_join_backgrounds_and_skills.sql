CREATE TABLE backgrounds_skills (
    background_id BLOB NOT NULL
        REFERENCES backgrounds(background_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(background_id) = 16),
    skill_id      BLOB NOT NULL
        REFERENCES skills(skill_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(skill_id) = 16),
    PRIMARY KEY (background_id, skill_id)
) STRICT, WITHOUT ROWID;
