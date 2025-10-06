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

INSERT INTO `players`
VALUES (X'01991836ac9f75898eff73915fd87018', "Spencer Dent")
ON CONFLICT (`player_id`) DO NOTHING
;

INSERT INTO `characters`
VALUES (X'01991836da1972298430f8ad85a67ee0', "Cygnus", X'01991836ac9f75898eff73915fd87018')
ON CONFLICT (`character_id`) DO NOTHING
;

INSERT INTO `ancestries`
VALUES (X'0199182824927164b25d368464947b6a', "Human")
, (X'019961a4e5427aeaa845989fe8f884c8', "Elf")
ON CONFLICT (`ancestry_id`) DO NOTHING
;

INSERT INTO `ancestry_traits`
VALUES (X'01991828aa3c7fa9a24bc2afacaa349d', "Attribute Increase", "Choose an Attribute. The chosen Attribute increases by 1 (up to the Attribute Limit).", 2)
ON CONFLICT (`ancestry_trait_id`) DO NOTHING
;

INSERT INTO `ancestries_ancestry_traits`
VALUES (X'01993b864c8277f7b9b4790f8e935a32', X'0199182824927164b25d368464947b6a', X'01991828aa3c7fa9a24bc2afacaa349d')
ON CONFLICT (`ancestry_id`, `ancestry_trait_id`) DO NOTHING
;

INSERT INTO `attributes`
VALUES (X'01993b832d6c7e7882b2063d613880b9', "Prime")
, (X'01993b83e9f978d4a5ae97c2011f49c6', "Might")
, (X'01993b8460827289a9e9cc105341940e', "Agility")
, (X'01993b84fcf17fcbb1fed093bfd9853d', "Charisma")
, (X'01993b8556b4774aa4a333bd7f76469e', "Intelligence")
ON CONFLICT (`attribute_id`) DO NOTHING
;

INSERT INTO `classes`
VALUES (X'019964edee4b746ea5a85006d034d3ba', "Artificer")
, (X'019964ee4cef7ec3a2ce3ea8844aadc9', "Barbarian")
, (X'019964ee6f237b65b83cf0cd85bf985f', "Bard")
, (X'019964ee77db78869c6783619eee210c', "Champion")
, (X'019964ee7ef475a68360cc0a3b12cffc', "Cleric")
, (X'019964ee864d7c15b46cbd28180f1f27', "Commander")
, (X'019964ee8cf6787e91d0f48e5c194f7b', "Druid")
, (X'019964eea1ad7225a1692680524594a3', "Hunter")
, (X'019964eeaac57516a28f2fcfd17fdfb3', "Monk")
, (X'019964eeb1ff7b288f9443d7cf59a4e5', "Psion")
, (X'019964eeb92879d19159713bf7c8cdbd', "Rogue")
, (X'019964eebf03730a95530ec528bb68ce', "Sorcerer")
, (X'019964eec5df783bbf5365a4ba231213', "Spellblade")
, (X'019964eecd22732fa0ab349338786aed', "Warlock")
, (X'019964eed31576dea71821db96213fcd', "Wizard")
ON CONFLICT (`class_id`) DO NOTHING
;

INSERT INTO `classes_subclasses`
VALUES (X'019964eeb1ff7b288f9443d7cf59a4e5', X'019964f27af17574a82346e464d01aa7') -- Psion :: Oracle
, (X'019964eeb1ff7b288f9443d7cf59a4e5', X'019964f2835d7929bcdc10d1d9d93a5e') -- Psion :: Psi-Knight
ON CONFLICT (`class_id`, `subclass_id`) DO NOTHING
;

INSERT INTO `skills`
VALUES (X'01993a736a8577e183451a57d7c324de', "Awareness", X'01993b832d6c7e7882b2063d613880b9')
, (X'01993b89eb9d7d71a9481f5dd0e6dd82', "Athletics", X'01993b83e9f978d4a5ae97c2011f49c6')
, (X'01993b8ce1b37b1da8d18ff3c1f3d58e', "Intimidation", X'01993b83e9f978d4a5ae97c2011f49c6')
, (X'01993b8e7add7d34a04620c11a889327', "Acrobatics", X'01993b8460827289a9e9cc105341940e')
, (X'01993b8ec8397c0fa20d9b17738aaf63', "Trickery", X'01993b8460827289a9e9cc105341940e')
, (X'01993b8efd737f49a1c955f3e11f885c', "Stealth", X'01993b8460827289a9e9cc105341940e')
, (X'01993ea183e17b82b3092a188fb6afda', "Animal", X'01993b8460827289a9e9cc105341940e')
, (X'01993ea1a9457087b8e568454a0bdadb', "Influence", X'01993b8460827289a9e9cc105341940e')
, (X'01993ea22e1a7d29952eedc454fa9f95', "Insight", X'01993b8460827289a9e9cc105341940e')
, (X'01993ea24cee74f9a1e9c9d10269ac62', "Investigation", X'01993b8460827289a9e9cc105341940e')
, (X'01993ea27be07dbbbb8c2a79be9df862', "Medicine", X'01993b8460827289a9e9cc105341940e')
, (X'01993ea29d2c70f9bc47df7378496cd0', "Survival", X'01993b8460827289a9e9cc105341940e')
ON CONFLICT (`skill_id`) DO NOTHING
;

INSERT INTO `subclasses`
VALUES (X'019964f27af17574a82346e464d01aa7', "Oracle")
, (X'019964f2835d7929bcdc10d1d9d93a5e', "Psi-Knight")
ON CONFLICT (`subclass_id`) DO NOTHING
;

INSERT INTO `trades`
VALUES (X'01993c6279d477f690568205594a194f', "Alchemy")
, (X'01993c24fd977213be9b22cf1d9abba6', "Arcana")
, (X'01993c60a4d47cf4b28b04a2cab76f31', "Blacksmithing")
, (X'01993c6263387012b539dae12bce21f3', "Brewing")
, (X'01993c625d70790a88440d0dba805941', "Carpentry")
, (X'01993c624f007611b33b6c5bfc97872c', "Cartography")
, (X'01993c65b4297edeb05e6bbc974a5c58', "Cooking")
, (X'01993c61dfba7a028fdf00240ed1ea7c', "Cryptography")
, (X'01993c63065175abb414c08d8a76222c', "Disguise")
, (X'01993c63288f7ab08bf0bf6dfb02d1ea', "Engineering")
, (X'01993c634b6c7719ba3f26ded46cf7a3', "Gaming")
, (X'01993c635f5b784f80ccb306c5d1e165', "Glassblowing")
, (X'01993c6375607c3aa0ea8f15f2331254', "Herbalism")
, (X'01993c2692b37405ac3c43ce36d4e499', "History")
, (X'01993c3419ae7bd39c1b9dbc51822077', "Illustration")
, (X'01993c638c9e7306b3069151eab22080', "Jeweler")
, (X'01993c63d606793fa1fd6af329f7a33b', "Leatherworking")
, (X'01993c63f87c7e109e3bec9f469be611', "Lockpicking")
, (X'01993c640aac788d9d142249ca54d12e', "Masonry")
, (X'01993c35c6e77510b08ce43982d13b13', "Musician")
, (X'01993c26c3b277ea9e479a1ca7c697bf', "Nature")
, (X'01993c26e3fe743d80e99c7956d25f3a', "Occultism")
, (X'01993c271111702884ae000055dff749', "Religion")
, (X'01993c642fde7df2adae5fa0a7f0b381', "Sculpting")
, (X'01993c64630f78ee93da7ac5e029d4d5', "Theatre")
, (X'01993c6482ea75b8894f17be262687bd', "Tinkering")
, (X'01993c648fd470b48d6105d6b8f167af', "Weaving")
, (X'01993c64a33878f9a402856d3704c9da', "Vehicles")
ON CONFLICT (`trade_id`) DO NOTHING
;

INSERT INTO `attributes_trades`
VALUES (X'01993b8556b4774aa4a333bd7f76469e', X'01993c24fd977213be9b22cf1d9abba6')
, (X'01993b8556b4774aa4a333bd7f76469e', X'01993c2692b37405ac3c43ce36d4e499')
, (X'01993b8556b4774aa4a333bd7f76469e', X'01993c26c3b277ea9e479a1ca7c697bf')
, (X'01993b8556b4774aa4a333bd7f76469e', X'01993c26e3fe743d80e99c7956d25f3a')
, (X'01993b8556b4774aa4a333bd7f76469e', X'01993c271111702884ae000055dff749')
, (X'01993b8460827289a9e9cc105341940e', X'01993c3419ae7bd39c1b9dbc51822077')
, (X'01993b8460827289a9e9cc105341940e', X'01993c35c6e77510b08ce43982d13b13')
, (X'01993b84fcf17fcbb1fed093bfd9853d', X'01993c35c6e77510b08ce43982d13b13')
ON CONFLICT (`attribute_id`, `trade_id`) DO NOTHING
;

INSERT INTO `masteries`
VALUES (X'01993c50621f720b98cc90eed9ddddea', "Novice", 2)
, (X'01993c50879c7d63848474593e4e0c70', "Adept", 4)
, (X'01993c50b7e8763da96d47673a2e12c6', "Expert", 6)
, (X'01993c50d7297fed893796dc5e928c04', "Master", 8)
, (X'01993c50ed66774d913ae96abef0b500', "Grandmaster", 10)
ON CONFLICT (`mastery_id`) DO NOTHING
;

INSERT INTO `languages`
VALUES (X'01993e35ae3470bfb26e86f39c6b8d85', "Common")
, (X'01993e35e4347c668b34a759162219d0', "Common Sign")
, (X'01993e7fe3ef751ea7ebd2208a30435b', "Human")
, (X'01993e80e761757c812cfec2e5ddedf1', "Dwarven")
, (X'01993e8203ef7d83861bf41cca36819e', "Elvish")
, (X'01993e9bde6070a6ab380a2c3f9f023f', "Gnomish")
, (X'01993e9c3c32767bb5cd8d552a436565', "Halfling")
, (X'01993e9cec66705c85350544dc0878d4', "Orcish")
, (X'01993e9d061b74958eb960e2d3f9caa0', "Giant")
, (X'01993e9d1cba7e3b9eeb1ee7b3688c5f', "Draconic")
, (X'01993e9d37657a6f9adab7074e9f48b1', "Fey")
, (X'01993e9d44c8716f95bdb8cecb2b0098', "Elemental")
, (X'01993e9d5249721a93806cb086147cd9', "Celestial")
, (X'01993e9d6d2d7b6dbff04da13988d0fe', "Fiend")
, (X'01993e9d811f7c78b349e1b19561c220', "Deep Speech")
ON CONFLICT (`language_id`) DO NOTHING
;

INSERT INTO `backgrounds`
VALUES (X'01993ea09d21764d9a0b98bb22b619ca', "Human Mercenary")
ON CONFLICT (`background_id`) DO NOTHING
;

INSERT INTO `backgrounds_languages`
VALUES (X'01993ea09d21764d9a0b98bb22b619ca', X'01993e35ae3470bfb26e86f39c6b8d85', 2)
, (X'01993ea09d21764d9a0b98bb22b619ca', X'01993e7fe3ef751ea7ebd2208a30435b', 2)
ON CONFLICT (`background_id`, `language_id`) DO NOTHING
;

INSERT INTO `backgrounds_skills`
VALUES (X'01993ea09d21764d9a0b98bb22b619ca', X'01993a736a8577e183451a57d7c324de')
, (X'01993ea09d21764d9a0b98bb22b619ca', X'01993b89eb9d7d71a9481f5dd0e6dd82')
ON CONFLICT (`background_id`, `skill_id`) DO NOTHING
;

INSERT INTO `backgrounds_trades`
VALUES (X'01993ea09d21764d9a0b98bb22b619ca', X'01993c624f007611b33b6c5bfc97872c') -- Human Mercenery :: Cartography
, (X'01993ea09d21764d9a0b98bb22b619ca', X'01993c64a33878f9a402856d3704c9da') -- Human Mercenery :: Vehicles
ON CONFLICT (`background_id`, `trade_id`) DO NOTHING
;

INSERT INTO `character_levels`
VALUES (X'01991836da1972298430f8ad85a67ee0', X'166ae11a3d404c618d390415e0cae6bb', 1)
, (X'0199593a64d37f6eafcff8363b19d41b', X'166ae11a3d404c618d390415e0cae6bb', 2)
, (X'0199593b03087b8295403e4ed35c2cb6',X'166ae11a3d404c618d390415e0cae6bb', 3)
ON CONFLICT (`character_level_id`) DO NOTHING
;

INSERT INTO `ancestries_character_levels`
    (`ancestry_id`, `character_level_id`)
VALUES (X'0199182824927164b25d368464947b6a', X'01991836da1972298430f8ad85a67ee0')
, (X'0199182824927164b25d368464947b6a', X'0199593b03087b8295403e4ed35c2cb6')
, (X'0199182824927164b25d368464947b6a', X'0199593a64d37f6eafcff8363b19d41b')
, (X'019961a4e5427aeaa845989fe8f884c8', X'0199593b03087b8295403e4ed35c2cb6')
ON CONFLICT (`ancestry_id`, `character_level_id`) DO NOTHING
;

INSERT INTO `character_levels_classes`
VALUES (X'01991836da1972298430f8ad85a67ee0',X'019964eeb1ff7b288f9443d7cf59a4e5')
, (X'0199593a64d37f6eafcff8363b19d41b',X'019964eeb1ff7b288f9443d7cf59a4e5')
, (X'0199593b03087b8295403e4ed35c2cb6',X'019964eeb1ff7b288f9443d7cf59a4e5')
ON CONFLICT (`character_level_id`, `class_id`) DO NOTHING
;

INSERT INTO `character_levels_subclasses`
VALUES (X'0199593b03087b8295403e4ed35c2cb6', X'019964f2835d7929bcdc10d1d9d93a5e')
ON CONFLICT (`character_level_id`, `subclass_id`) DO NOTHING
;

-- 01993b832d6c7e7882b2063d613880b9
INSERT INTO `character_level_base_attribute_values`
    (`character_level_id`, `attribute_id`, `value`)
VALUES (X'01991836da1972298430f8ad85a67ee0', X'01993b832d6c7e7882b2063d613880b9', 3)
, (X'01991836da1972298430f8ad85a67ee0', X'01993b83e9f978d4a5ae97c2011f49c6', 0)
, (X'01991836da1972298430f8ad85a67ee0', X'01993b8460827289a9e9cc105341940e', 1)
, (X'01991836da1972298430f8ad85a67ee0', X'01993b84fcf17fcbb1fed093bfd9853d', 0)
, (X'01991836da1972298430f8ad85a67ee0', X'01993b8556b4774aa4a333bd7f76469e', 3)
, (X'0199593a64d37f6eafcff8363b19d41b', X'01993b832d6c7e7882b2063d613880b9', 3)
, (X'0199593a64d37f6eafcff8363b19d41b', X'01993b83e9f978d4a5ae97c2011f49c6', 0)
, (X'0199593a64d37f6eafcff8363b19d41b', X'01993b8460827289a9e9cc105341940e', 1)
, (X'0199593a64d37f6eafcff8363b19d41b', X'01993b84fcf17fcbb1fed093bfd9853d', 0)
, (X'0199593a64d37f6eafcff8363b19d41b', X'01993b8556b4774aa4a333bd7f76469e', 3)
, (X'0199593b03087b8295403e4ed35c2cb6', X'01993b832d6c7e7882b2063d613880b9', 3)
, (X'0199593b03087b8295403e4ed35c2cb6', X'01993b83e9f978d4a5ae97c2011f49c6', 0)
, (X'0199593b03087b8295403e4ed35c2cb6', X'01993b8460827289a9e9cc105341940e', 1)
, (X'0199593b03087b8295403e4ed35c2cb6', X'01993b84fcf17fcbb1fed093bfd9853d', 0)
, (X'0199593b03087b8295403e4ed35c2cb6', X'01993b8556b4774aa4a333bd7f76469e', 3)
ON CONFLICT (`character_level_id`, `attribute_id`) DO NOTHING
;

INSERT INTO `spell_schools`
    (`spell_school_id`, `name`)
VALUES (X'01999676d58d7840b02951505eb57504', "Destruction")
ON CONFLICT (`spell_school_id`) DO NOTHING
;

INSERT INTO `spells`
    (`spell_id`, `name`, `spell_school_id`, `action_point_cost`, `mana_point_cost`, `range_kind`, `range_value`, `duration_kind`, `duration_value`, `sustained`, `description`)
VALUES (X'0199967326fe7954825fbbc78d2300a2', "Fire Bolt", X'01999676d58d7840b02951505eb57504', 1, 0, "Spaces", 10, "Instant", NULL, false, NULL)
ON CONFLICT (`spell_id`) DO NOTHING
;

INSERT INTO `spell_effects`
    (`spell_effect_id`, `name`, `description`)
VALUES (X'0199b77066f17129b38fbae190de4194', "Spell Attack (Fire II)", "You can make a Spell Check against the PD of a target within range. Hit: The target takes 2 Fire damage.")
, (X'0199b787e5677f238326c33865489868', "Fire Orb", "A flickering flame appears in your hand...")
, (X'0199b788024c75f9ae90fc6265abbae8', "Cantrip Passive (Burning)", "You deal +1 damage against creatures that are Burning.")
ON CONFLICT (`spell_effect_id`) DO NOTHING
;

INSERT INTO `spells_spell_effects`
    (`spell_id`, `spell_effect_id`)
VALUES (X'0199967326fe7954825fbbc78d2300a2', X'0199b77066f17129b38fbae190de4194')
, (X'0199967326fe7954825fbbc78d2300a2', X'0199b787e5677f238326c33865489868')
, (X'0199967326fe7954825fbbc78d2300a2', X'0199b788024c75f9ae90fc6265abbae8')
ON CONFLICT (`spell_id`, `spell_effect_id`) DO NOTHING
;

INSERT INTO `spell_lists`
    (`spell_list_id`, `name`)
VALUES (X'01999679b6397fdf9dafaf18ce8ba4ef', "Arcane")
, (X'01999679f92373b19969345c1d68c409', "Primal")
, (X'0199967a1e3e74c884153d81011de5a2', "Divine")
ON CONFLICT (`spell_list_id`) DO NOTHING
;

INSERT INTO `spells_spell_lists`
VALUES (X'0199967326fe7954825fbbc78d2300a2', X'01999679b6397fdf9dafaf18ce8ba4ef')
, (X'0199967326fe7954825fbbc78d2300a2', X'01999679f92373b19969345c1d68c409')
ON CONFLICT (`spell_id`, `spell_list_id`) DO NOTHING
;

INSERT INTO `spell_tags`
VALUES (X'0199968302bc78d9a245f312ddecb3e6', "Cantrip")
, (X'0199968323f37a34bee4ab00d551238d', "Ritual")
ON CONFLICT (`spell_tag_id`) DO NOTHING
;

INSERT INTO `spells_spell_tags`
VALUES (X'0199967326fe7954825fbbc78d2300a2', X'0199968302bc78d9a245f312ddecb3e6')
ON CONFLICT (`spell_id`, `spell_tag_id`) DO NOTHING
;

INSERT INTO `point_enhancements`
    (`point_enhancement_id`, `name`, `action_point_cost`, `mana_point_cost`, `description`)
VALUES (X'01999c5dc6b67c4cb5ae6b5e67fe9295', "Damage I (Fire)", 1, 0, "You deal +1 Fire damage.")
, (X'01999c62e5ca7421b835b9bd813da70d', "Range", 1, 0, "You increase the range by +5 Spaces.")
ON CONFLICT (`point_enhancement_id`) DO NOTHING
;

INSERT INTO `point_enhancements_spells`
VALUES (X'01999c5dc6b67c4cb5ae6b5e67fe9295', X'0199967326fe7954825fbbc78d2300a2')
, (X'01999c62e5ca7421b835b9bd813da70d', X'0199967326fe7954825fbbc78d2300a2')
ON CONFLICT (`point_enhancement_id`, `spell_id`) DO NOTHING
;

INSERT INTO `items`
    (`item_id`, `name`)
VALUES (X'0199b6adea827ccea30d79b0b5820617', "Hand Axe")
ON CONFLICT (`item_id`) DO NOTHING
;

INSERT INTO `weapons`
    (`weapon_id`, `type`)
VALUES (X'0199b6adea827ccea30d79b0b5820617', "Melee")
ON CONFLICT (`weapon_id`) DO NOTHING
;

INSERT INTO `weapon_styles`
    (`weapon_style_id`, `name`, `description`, `damage_type`)
VALUES (X'0199b6b3df197145a60fef49ad9698aa', "Axe", "You deal +1 damage against creatures that are Bleeding", "Slashing")
ON CONFLICT (`weapon_style_id`) DO NOTHING
;

INSERT INTO `weapons_weapon_styles`
    (`weapon_id`, `weapon_style_id`)
VALUES (X'0199b6adea827ccea30d79b0b5820617', X'0199b6b3df197145a60fef49ad9698aa')
ON CONFLICT (`weapon_id`, `weapon_style_id`) DO NOTHING
;

INSERT INTO `weapon_properties`
    (`weapon_property_id`, `name`, `description`, `required_weapon_property_id`)
VALUES (X'0199b6ba413671bdbb86189f5beacbb5', "Concealable", "Drawing the Weapon doesn't provoke Opportunity Attacks.", NULL)
, (X'0199b6ba5b2a768fb474f05b92027443', "Toss (5/10)", "You can throw the Weapon to make a Ranged Martial Attack (5/10)", NULL)
ON CONFLICT (`weapon_property_id`) DO NOTHING
;

INSERT INTO `weapons_weapon_properties`
    (`weapon_id`, `weapon_property_id`)
VALUES (X'0199b6adea827ccea30d79b0b5820617', X'0199b6ba413671bdbb86189f5beacbb5')
, (X'0199b6adea827ccea30d79b0b5820617', X'0199b6ba5b2a768fb474f05b92027443')
ON CONFLICT (`weapon_id`, `weapon_property_id`) DO NOTHING
;
