CREATE TABLE IF NOT EXISTS `ancestry_traits_character_levels` (
    `ancestry_trait_id` INTEGER NOT NULL,
    `character_id` INTEGER NOT NULL,
    `level` INTEGER NOT NULL,
    CONSTRAINT `ancestry_trait_fk`
        FOREIGN KEY (`ancestry_trait_id`)
        REFERENCES `ancestries` (`id`)
        ON DELETE CASCADE
        ON UPDATE NO ACTION,
    CONSTRAINT `character_level_fk`
        FOREIGN KEY (`character_id`, `level`)
        REFERENCES `character_levels` (`character_id`, `level`)
        ON DELETE CASCADE
        ON UPDATE NO ACTION
);
