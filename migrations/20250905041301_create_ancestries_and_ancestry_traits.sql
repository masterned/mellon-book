CREATE TABLE `ancestries` (
    `id`   INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    `uuid` BLOB NOT NULL,
    `name` TEXT NOT NULL  
);

CREATE UNIQUE INDEX `ancestries_uuid_uniq`
    ON `ancestries` (`uuid`);

CREATE TABLE `ancestry_traits` (
    `id`   INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    `uuid` BLOB NOT NULL,
    `name` TEXT NOT NULL,
    `description` TEXT NOT NULL,
    `cost` INTEGER NOT NULL
);

CREATE UNIQUE INDEX `ancestry_traits_uuid_uniq`
    ON `ancestry_traits` (`uuid`);

CREATE TABLE `ancestries_ancestry_traits` (
    `ancestry_id`       INTEGER NOT NULL,
    `ancestry_trait_id` INTEGER NOT NULL,
    `expanded`          BOOLEAN NOT NULL DEFAULT FALSE,
    CONSTRAINT `ancestry_fk` FOREIGN KEY (`ancestry_id`)
        REFERENCES `ancestries` (`id`)
        ON UPDATE NO ACTION
        ON DELETE CASCADE,
    CONSTRAINT `ancestry_trait_fk` FOREIGN KEY (`ancestry_trait_id`)
        REFERENCES `ancestry_traits` (`id`)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);
