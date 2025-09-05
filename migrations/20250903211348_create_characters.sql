CREATE TABLE `characters` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    `uuid` BLOB NOT NULL,
    `name` TEXT NOT NULL,
    `creator_id` INTEGER NOT NULL,
    CONSTRAINT `creator_fk` FOREIGN KEY (`creator_id`)
        REFERENCES `players` (`id`)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);

CREATE UNIQUE INDEX `characters_uuid_uniq`
    ON `characters` (`uuid`);
