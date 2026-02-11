CREATE VIEW IF NOT EXISTS `character_level_attributes`
    ( `character_level_id`
    , `prime`
    , `might`
    , `agility`
    , `charisma`
    , `intelligence`
    )
AS
SELECT `character_level_id`
    , `prime`
    , `might`
    , `agility`
    , `charisma`
    , `intelligence`
FROM
    (SELECT `character_level_id`
        , `value` as "prime"
    FROM `character_level_base_attribute_values`
    JOIN `attributes`
    USING (`attribute_id`)
    WHERE `name` LIKE "%Prime%")
    JOIN
    ( SELECT `character_level_id`
        , `value` as "might"
    FROM `character_level_base_attribute_values`
    JOIN `attributes`
    USING (`attribute_id`)
    WHERE `name` LIKE "%Might%"
    ) USING (`character_level_id`)
    JOIN
    ( SELECT `character_level_id`
        , `value` as "agility"
    FROM `character_level_base_attribute_values`
    JOIN `attributes`
    USING (`attribute_id`)
    WHERE `name` LIKE "%Agility%"
    ) USING (`character_level_id`)
    JOIN
    (SELECT `character_level_id`
        , `value` as "charisma"
    FROM `character_level_base_attribute_values`
    JOIN `attributes`
    USING (`attribute_id`)
    WHERE `name` LIKE "%Charisma%"
    ) USING (`character_level_id`)
    JOIN
    (SELECT `character_level_id`
        , `value` as "intelligence"
    FROM `character_level_base_attribute_values`
    JOIN `attributes`
    USING (`attribute_id`)
    WHERE `name` LIKE "%Intelligence%"
    ) USING (`character_level_id`)
;
