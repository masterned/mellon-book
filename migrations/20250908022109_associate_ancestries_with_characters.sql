CREATE TABLE IF NOT EXISTS `ancestries_characters` (
    `ancestry_id` INTEGER NOT NULL,
    `character_id` INTEGER NOT NULL,
    CONSTRAINT `ancestry_fk`
        FOREIGN KEY (`ancestry_id`)
        REFERENCES `ancestries` (`id`)
        ON DELETE CASCADE
        ON UPDATE NO ACTION,
    CONSTRAINT `character_fk`
        FOREIGN KEY (`character_id`)
        REFERENCES `characters` (`id`)
        ON DELETE CASCADE
        ON UPDATE NO ACTION
);
