INSERT INTO `players`
VALUES (X'01991836ac9f75898eff73915fd87018', "Spencer Dent")
ON CONFLICT (`id`) DO NOTHING
;

INSERT INTO `characters`
VALUES (X'01991836da1972298430f8ad85a67ee0', "Cygnus", X'01991836ac9f75898eff73915fd87018')
ON CONFLICT (`id`) DO NOTHING
;

INSERT INTO `ancestries`
VALUES (X'0199182824927164b25d368464947b6a', "Human")
ON CONFLICT (`id`) DO NOTHING
;

INSERT INTO `ancestry_traits`
VALUES (X'01991828aa3c7fa9a24bc2afacaa349d', "Attribute Increase", "Choose an Attribute. The chosen Attribute increases by 1 (up to the Attribute Limit).", 2)
ON CONFLICT (`id`) DO NOTHING
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
ON CONFLICT (`id`) DO NOTHING
;

INSERT INTO `skills`
VALUES (X'01993a736a8577e183451a57d7c324de', "Awareness", X'01993b832d6c7e7882b2063d613880b9')
, (X'01993b89eb9d7d71a9481f5dd0e6dd82', "Athletics", X'01993b83e9f978d4a5ae97c2011f49c6')
, (X'01993b8ce1b37b1da8d18ff3c1f3d58e', "Intimidation", X'01993b83e9f978d4a5ae97c2011f49c6')
, (X'01993b8e7add7d34a04620c11a889327', "Acrobatics", X'01993b8460827289a9e9cc105341940e')
, (X'01993b8ec8397c0fa20d9b17738aaf63', "Trickery", X'01993b8460827289a9e9cc105341940e')
, (X'01993b8efd737f49a1c955f3e11f885c', "Stealth", X'01993b8460827289a9e9cc105341940e')
ON CONFLICT (`id`) DO NOTHING
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
ON CONFLICT (`id`) DO NOTHING
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
ON CONFLICT (`id`) DO NOTHING
;
