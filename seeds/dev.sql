INSERT INTO `players` (`name`, `uuid`)
    VALUES ("Spencer Dent", X'01991836ac9f75898eff73915fd87018')
;

INSERT INTO `characters` (`name`, `creator_id`, `uuid`)
    VALUES ("Cygnus", 1, X'01991836da1972298430f8ad85a67ee0')
;

INSERT INTO `ancestries` (`uuid`, `name`)
    VALUES (X'0199182824927164b25d368464947b6a', "Human")
;

INSERT INTO `ancestry_traits` (`uuid`, `name`, `description`, `cost`)
    VALUES (
        X'01991828aa3c7fa9a24bc2afacaa349d',
        "Attribute Increase",
        "Choose an Attribute. The chosen Attribute increases by 1 (up to the Attribute Limit).",
        2
    )
;

INSERT INTO `ancestries_ancestry_traits` (`ancestry_id`, `ancestry_trait_id`)
    VALUES (1, 1)
;
