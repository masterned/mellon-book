CREATE TABLE IF NOT EXISTS `character_levels` (
    `character_id` INTEGER NOT NULL,
    `level`        INTEGER NOT NULL,
    `uuid`         BLOB NOT NULL,
    PRIMARY KEY (`character_id`, `level`),
    CONSTRAINT `character_fk`
        FOREIGN KEY (`character_id`)
        REFERENCES `characters` (`id`)
        ON DELETE CASCADE
        ON UPDATE NO ACTION
);

CREATE UNIQUE INDEX `character_levels_uuid_uniq`
    ON `character_levels` (`uuid`);
