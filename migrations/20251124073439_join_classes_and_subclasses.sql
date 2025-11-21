CREATE TABLE classes_subclasses (
    class_id BLOB NOT NULL
        REFERENCES classes(class_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(class_id) = 16),
    subclass_id BLOB NOT NULL
        REFERENCES subclasses(subclass_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
        CHECK (length(subclass_id) = 16),
    PRIMARY KEY (class_id, subclass_id)
) STRICT, WITHOUT ROWID;
