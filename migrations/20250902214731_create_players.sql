CREATE TABLE `players` (
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    `uuid` BLOB NOT NULL,
    `name` TEXT NOT NULL
);

CREATE UNIQUE INDEX `players_uuid_uniq`
    ON `players` (`uuid`);
