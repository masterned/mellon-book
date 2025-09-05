INSERT INTO `players` (`name`, `uuid`)
    VALUES ("Spencer Dent", "01991836-ac9f-7589-8eff-73915fd87018")
;

INSERT INTO `characters` (`name`, `creator_id`, `uuid`)
    VALUES ("Cygnus", 1, "01991836-da19-7229-8430-f8ad85a67ee0")
;

INSERT INTO `ancestries` (`uuid`, `name`)
    VALUES ("01991828-2492-7164-b25d-368464947b6a", "Human")
;

INSERT INTO `ancestry_traits` (`uuid`, `name`, `description`, `cost`)
    VALUES (
        "01991828-aa3c-7fa9-a24b-c2afacaa349d",
        "Attribute Increase",
        "Choose an Attribute. The chosen Attribute increases by 1 (up to the Attribute Limit).",
        2
    )
;

INSERT INTO `ancestries_ancestry_traits` (`ancestry_id`, `ancestry_trait_id`)
    VALUES (1, 1)
;
