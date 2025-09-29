table "players" {
  schema = schema.main
  column "player_id" {
    null = false
    type = blob
  }
  column "name" {
    null = false
    type = text
  }
  primary_key {
    columns = [column.player_id]
  }
  check "16 byte player_id" {
    expr = "length(`player_id`) = 16"
  }
  without_rowid = true
}
table "characters" {
  schema = schema.main
  column "character_id" {
    null = false
    type = blob
  }
  column "name" {
    null = false
    type = text
  }
  column "creator_id" {
    null = false
    type = blob
  }
  primary_key {
    columns = [column.character_id]
  }
  foreign_key "creator_fk" {
    columns     = [column.creator_id]
    ref_columns = [table.players.column.player_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  check "16 byte character_id" {
    expr = "length(`character_id`) = 16"
  }
  without_rowid = true
}
table "ancestries" {
  schema = schema.main
  column "ancestry_id" {
    null = false
    type = blob
  }
  column "name" {
    null = false
    type = text
  }
  primary_key {
    columns = [column.ancestry_id]
  }
  check "16 byte ancestry_id" {
    expr = "length(`ancestry_id`) = 16"
  }
  without_rowid = true
}
table "ancestry_traits" {
  schema = schema.main
  column "ancestry_trait_id" {
    null = false
    type = blob
  }
  column "name" {
    null = false
    type = text
  }
  column "description" {
    null = false
    type = text
  }
  column "cost" {
    null = false
    type = integer
  }
  primary_key {
    columns = [column.ancestry_trait_id]
  }
  check "16 byte ancestry_trait_id" {
    expr = "length(`ancestry_trait_id`) = 16"
  }
  without_rowid = true
}
table "ancestries_ancestry_traits" {
  schema = schema.main
  column "ancestry_id" {
    null = false
    type = blob
  }
  column "ancestry_trait_id" {
    null = false
    type = blob
  }
  column "expanded" {
    null    = false
    type    = boolean
    default = false
  }
  primary_key {
    columns = [column.ancestry_id, column.ancestry_trait_id]
  }
  foreign_key "ancestry_trait_fk" {
    columns     = [column.ancestry_trait_id]
    ref_columns = [table.ancestry_traits.column.ancestry_trait_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  foreign_key "ancestry_fk" {
    columns     = [column.ancestry_id]
    ref_columns = [table.ancestries.column.ancestry_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
}
table "character_levels" {
  schema = schema.main
  column "character_level_id" {
    null = false
    type = blob
  }
  column "character_id" {
    null = false
    type = blob
  }
  column "level" {
    null = false
    type = integer
  }
  primary_key {
    columns = [column.character_level_id]
  }
  foreign_key "character_fk" {
    columns     = [column.character_id]
    ref_columns = [table.characters.column.character_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  check "16 byte character_level_id" {
    expr = "length(`character_level_id`) = 16"
  }
  index "character_levels_character_level_uniq" {
    unique  = true
    columns = [column.character_id, column.level]
  }
  without_rowid = true
}
table "ancestries_character_levels" {
  schema = schema.main
  column "ancestry_id" {
    null = false
    type = blob
  }
  column "character_level_id" {
    null = false
    type = blob
  }
  primary_key {
    columns = [column.ancestry_id, column.character_level_id]
  }
  foreign_key "ancestry_fk" {
    columns     = [column.ancestry_id]
    ref_columns = [table.ancestries.column.ancestry_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  foreign_key "character_level_fk" {
    columns     = [column.character_level_id]
    ref_columns = [table.character_levels.column.character_level_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  check "16 byte ancestry_id" {
    expr = "length(`ancestry_id`) = 16"
  }
  check "16 byte character_level_id" {
    expr = "length(`character_level_id`) = 16"
  }
  without_rowid = true
}
table "ancestry_traits_character_levels" {
  schema = schema.main
  column "ancestry_trait_id" {
    null = false
    type = blob
  }
  column "character_level_id" {
    null = false
    type = blob
  }
  primary_key {
    columns = [column.ancestry_trait_id, column.character_level_id]
  }
  foreign_key "character_level_fk" {
    columns     = [column.character_level_id]
    ref_columns = [table.character_levels.column.character_level_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  foreign_key "ancestry_trait_fk" {
    columns     = [column.ancestry_trait_id]
    ref_columns = [table.ancestry_traits.column.ancestry_trait_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
}
table "classes" {
  schema = schema.main
  column "class_id" {
    null = false
    type = blob
  }
  column "name" {
    null = false
    type = text
  }
  primary_key {
    columns = [column.class_id]
  }
  check "16 byte class_id" {
    expr = "length(`class_id`) = 16"
  }
  check "non-blank name" {
    expr = "`name` != \"\""
  }
}
table "subclasses" {
  schema = schema.main
  column "subclass_id" {
    null = false
    type = blob
  }
  column "name" {
    null = false
    type = text
  }
  primary_key {
    columns = [column.subclass_id]
  }
  check "16 byte subclass_id" {
    expr = "length(`subclass_id`) = 16"
  }
  check "non-blank name" {
    expr = "`name` != \"\""
  }
}
table "classes_subclasses" {
  schema = schema.main
  column "class_id" {
    null = false
    type = blob
  }
  column "subclass_id" {
    null = false
    type = blob
  }
  primary_key {
    columns = [column.class_id, column.subclass_id]
  }
  foreign_key "class_fk" {
    columns     = [column.class_id]
    ref_columns = [table.classes.column.class_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  foreign_key "subclass_fk" {
    columns     = [column.subclass_id]
    ref_columns = [table.subclasses.column.subclass_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  check "16 byte class_id" {
    expr = "length(`class_id`) = 16"
  }
  check "16 byte subclass_id" {
    expr = "length(`subclass_id`) = 16"
  }
}
table "backgrounds_character_levels" {
  schema = schema.main
  column "background_id" {
    null = false
    type = blob
  }
  column "character_level_id" {
    null = false
    type = blob
  }
  primary_key {
    columns = [column.background_id, column.character_level_id]
  }
  foreign_key "background_fk" {
    columns     = [column.background_id]
    ref_columns = [table.backgrounds.column.background_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  foreign_key "character_level_fk" {
    columns     = [column.character_level_id]
    ref_columns = [table.character_levels.column.character_level_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
}
table "character_levels_classes" {
  schema = schema.main
  column "character_level_id" {
    null = false
    type = blob
  }
  column "class_id" {
    null = false
    type = blob
  }
  primary_key {
    columns = [column.character_level_id, column.class_id]
  }
  foreign_key "character_level_fk" {
    columns     = [column.character_level_id]
    ref_columns = [table.character_levels.column.character_level_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  foreign_key "class_fk" {
    columns     = [column.class_id]
    ref_columns = [table.classes.column.class_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  check "16 byte character_level_id" {
    expr = "length(`character_level_id`) = 16"
  }
  check "16 byte class_id" {
    expr = "length(`class_id`) = 16"
  }
}
table "character_levels_subclasses" {
  schema = schema.main
  column "character_level_id" {
    null = false
    type = blob
  }
  column "subclass_id" {
    null = false
    type = blob
  }
  primary_key {
    columns = [column.character_level_id, column.subclass_id]
  }
  foreign_key "character_level_fk" {
    columns     = [column.character_level_id]
    ref_columns = [table.character_levels.column.character_level_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  foreign_key "subclass_fk" {
    columns     = [column.subclass_id]
    ref_columns = [table.subclasses.column.subclass_id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  check "16 byte character_level_id" {
    expr = "length(`character_level_id`) = 16"
  }
  check "16 byte subclass_id" {
    expr = "length(`subclass_id`) = 16"
  }
}
table "attributes" {
  schema = schema.main
  column "attribute_id" {
    type = blob
    null = false
  }
  column "name" {
    type = text
    null = false
  }
  primary_key {
    columns = [column.attribute_id]
  }
  check "16 byte attribute_id" {
    expr = "length(`attribute_id`) = 16"
  }
}
table "skills" {
  schema = schema.main
  column "skill_id" {
    type = blob
    null = false
  }
  column "name" {
    type = text
    null = false
  }
  column "attribute_id" {
    type = blob
    null = false
  }
  primary_key {
    columns = [column.skill_id]
  }
  foreign_key "attribute_fk" {
    columns = [column.attribute_id]
    ref_columns = [table.attributes.column.attribute_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  check "16 byte skill_id" {
    expr = "length(`skill_id`) = 16"
  }
  check "16 byte attribute_id" {
    expr = "length(`attribute_id`) = 16"
  }
  without_rowid = true
}
table "trades" {
  schema = schema.main
  column "trade_id" {
    type = blob
    null = false
  }
  column "name" {
    type = text
    null = false
  }
  primary_key {
    columns = [column.trade_id]
  }
  check "16 byte trade_id" {
    expr = "length(`trade_id`) = 16"
  }
  without_rowid = true
}
table "attributes_trades" {
  schema = schema.main
  column "attribute_id" {
    type = blob
    null = false
  }
  column "trade_id" {
    type = blob
    null = false
  }
  primary_key {
    columns = [column.attribute_id, column.trade_id]
  }
  foreign_key "attribute_fk" {
    columns = [column.attribute_id]
    ref_columns = [table.attributes.column.attribute_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  foreign_key "trade_fk" {
    columns = [column.trade_id]
    ref_columns = [table.trades.column.trade_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  check "16 byte attribute_id" {
    expr = "length(`attribute_id`) = 16"
  }
  check "16 byte trade_id" {
    expr = "length(`trade_id`) = 16"
  }
}
table "masteries" {
  schema = schema.main
  column "mastery_id" {
    type = blob
    null = false
  }
  column "name" {
    type = text
    null = false
  }
  column "bonus" {
    type = integer
    null = false
  }
  primary_key {
    columns = [column.mastery_id]
  }
  check "16 byte mastery_id" {
    expr = "length(`mastery_id`) = 16"
  }
  without_rowid = true
}
table "languages" {
  schema = schema.main
  column "language_id" {
    type = blob
    null = false
  }
  column "name" {
    type = text
    null = false
  }
  primary_key {
    columns = [column.language_id]
  }
  check "16 byte language_id" {
    expr = "length(`language_id`) = 16"
  }
  without_rowid = true
}
table "backgrounds" {
  schema = schema.main
  column "background_id" {
    type = blob
    null = false
  }
  column "name" {
    type = text
    null = false
  }
  primary_key {
    columns = [column.background_id]
  }
  check "16 byte background_id" {
    expr = "length(`background_id`) = 16"
  }
  without_rowid = true
}
table "backgrounds_languages" {
  schema = schema.main
  column "background_id" {
    type = blob
    null = false
  }
  column "language_id" {
    type = blob
    null = false
  }
  column "fluency" {
    type = integer
    null = false
  }
  primary_key {
    columns = [column.background_id, column.language_id]
  }
  foreign_key "background_fk" {
    columns = [column.background_id]
    ref_columns = [table.backgrounds.column.background_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  foreign_key "language_fk" {
    columns = [column.language_id]
    ref_columns = [table.languages.column.language_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  check "16 byte background_id" {
    expr = "length(`background_id`) = 16"
  }
  check "16 byte language_id" {
    expr = "length(`language_id`) = 16"
  }
  check "enum fluency" {
    expr = "`fluency` in (1, 2)"
  }
  without_rowid = true
}
table "backgrounds_skills" {
  schema = schema.main
  column "background_id" {
    type = blob
    null = false
  }
  column "skill_id" {
    type = blob
    null = false
  }
  primary_key {
    columns = [column.background_id, column.skill_id]
  }
  foreign_key "background_fk" {
    columns = [column.background_id]
    ref_columns = [table.backgrounds.column.background_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  foreign_key "skill_fk" {
    columns = [column.skill_id]
    ref_columns = [table.skills.column.skill_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  check "16 byte background_id" {
    expr = "length(`background_id`) = 16"
  }
  check "16 byte skill_id" {
    expr = "length(`skill_id`) = 16"
  }
  without_rowid = true
}
table "backgrounds_trades" {
  schema = schema.main
  column "background_id" {
    type = blob
    null = false
  }
  column "trade_id" {
    type = blob
    null = false
  }
  primary_key {
    columns = [column.background_id, column.trade_id]
  }
  foreign_key "background_fk" {
    columns = [column.background_id]
    ref_columns = [table.backgrounds.column.background_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  foreign_key "trade_fk" {
    columns = [column.trade_id]
    ref_columns = [table.trades.column.trade_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  check "16 byte background_id" {
    expr = "length(`background_id`) = 16"
  }
  check "16 byte trade_id" {
    expr = "length(`trade_id`) = 16"
  }
  without_rowid = true
}
table "character_level_base_attribute_values" {
  schema = schema.main
  column "character_level_id" {
    type = blob
    null = false
  }
  column "attribute_id" {
    type = blob
    null = false
  }
  column "value" {
    type = int
    null = false
  }
  primary_key {
    columns = [column.character_level_id, column.attribute_id]
  }
  foreign_key "character_level_fk" {
    columns = [column.character_level_id]
    ref_columns = [table.character_levels.column.character_level_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  foreign_key "attribute_fk" {
    columns = [column.attribute_id]
    ref_columns = [table.attributes.column.attribute_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  check "16 byte character_level_id" {
    expr = "length(`character_level_id`) = 16"
  }
  check "16 byte attribute_id" {
    expr = "length(`attribute_id`) = 16"
  }
  without_rowid = true
}
table "items" {
  schema = schema.main
  column "item_id" {
    type = blob
    null = false
  }
  column "name" {
    type = text
    null = false
  }
  primary_key {
    columns = [column.item_id]
  }
  check "16 byte item_id" {
    expr = "length(`item_id`) = 16"
  }
  check "non-blank name" {
    expr = "`name` != \"\""
  }
  without_rowid = true
}
table "spells" {
  schema = schema.main
  column "spell_id" {
    type = blob
    null = false
  }
  column "name" {
    type = text
    null = false
  }
  column "spell_family_id" {
    type = blob
    null = false
  }
  column "has_verbal" {
    type = boolean
    null = false
    default = false
  }
  column "has_somatic" {
    type = boolean
    null = false
    default = false
  }
  column "action_point_cost" {
    type = integer
    null = false
    default = 1
  }
  column "mana_point_cost" {
    type = integer
    null = false
    default = 0
  }
  column "range" {
    type = text
    null = false
  }
  column "duration" {
    type = text
    null = false
  }
  column "description" {
    type = text
    null = false
  }
  primary_key {
    columns = [column.spell_id]
  }
  foreign_key "spell_family_fk" {
    columns = [column.spell_family_id]
    ref_columns = [table.spell_families.column.spell_family_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  check "16 byte spell_id" {
    expr = "length(`spell_id`) = 16"
  }
  check "non-blank name" {
    expr = "`name` != \"\""
  }
  check "16 byte spell_family_id" {
    expr = "length(`spell_family_id`) = 16"
  }
  check "non-blank range" {
    expr = "`range` != \"\""
  }
  check "non-blank duration" {
    expr = "`duration` != \"\""
  }
  check "non-blank description" {
    expr = "`description` != \"\""
  }
  without_rowid = true
}
table "spell_material_components" {
  schema = schema.main
  column "spell_id" {
    type = blob
    null = false
  }
  column "item_id" {
    type = blob
    null = false
  }
  column "consumed" {
    type = boolean
    null = false
    default = false
  }
  primary_key {
    columns = [column.spell_id, column.item_id]
  }
  foreign_key "spell_fk" {
    columns = [column.spell_id]
    ref_columns = [table.spells.column.spell_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  foreign_key "item_fk" {
    columns = [column.item_id]
    ref_columns = [table.items.column.item_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  check "16 byte spell_id" {
    expr = "length(`spell_id`) = 16"
  }
  check "16 byte item_id" {
    expr = "length(`item_id`) = 16"
  }
  without_rowid = true
}
table "spell_families" {
  schema = schema.main
  column "spell_family_id" {
    type = blob
    null = false
  }
  column "name" {
    type = text
    null = false
  }
  primary_key {
    columns = [column.spell_family_id]
  }
  check "16 byte spell_family_id" {
    expr = "length(`spell_family_id`) = 16"
  }
  check "non-blank name" {
    expr = "`name` != \"\""
  }
  without_rowid = true
}
table "spell_tags" {
  schema = schema.main
  column "spell_tag_id" {
    type = blob
    null = false
  }
  column "name" {
    type = text
    null = false
  }
  primary_key {
    columns = [column.spell_tag_id]
  }
  check "16 byte spell_tag_id" {
    expr = "length(`spell_tag_id`) = 16"
  }
  check "non-blank name" {
    expr = "`name` != \"\""
  }
  without_rowid = true
}
table "spells_spell_tags" {
  schema = schema.main
  column "spell_id" {
    type = blob
    null = false
  }
  column "spell_tag_id" {
    type = blob
    null = false
  }
  primary_key {
    columns = [column.spell_id, column.spell_tag_id]
  }
  foreign_key "spell_fk" {
    columns = [column.spell_id]
    ref_columns = [table.spells.column.spell_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  foreign_key "spell_tag_fk" {
    columns = [column.spell_tag_id]
    ref_columns = [table.spell_tags.column.spell_tag_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  check "16 byte spell_id" {
    expr = "length(`spell_id`) = 16"
  }
  check "16 byte spell_tag_id" {
    expr = "length(`spell_tag_id`) = 16"
  }
  without_rowid = true
}
table "spell_lists" {
  schema = schema.main
  column "spell_list_id" {
    type = blob
    null = false
  }
  column "name" {
    type = text
    null = false
  }
  primary_key {
    columns = [column.spell_list_id]
  }
  check "16 byte spell_list_id" {
    expr = "length(`spell_list_id`) = 16"
  }
  check "non-blank name" {
    expr = "`name` != \"\""
  }
  without_rowid = true
}
table "spells_spell_lists" {
  schema = schema.main
  column "spell_id" {
    type = blob
    null = false
  }
  column "spell_list_id" {
    type = blob
    null = false
  }
  primary_key {
    columns = [column.spell_id, column.spell_list_id]
  }
  foreign_key "spell_fk" {
    columns = [column.spell_id]
    ref_columns = [table.spells.column.spell_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  foreign_key "spell_list_fk" {
    columns = [column.spell_list_id]
    ref_columns = [table.spell_lists.column.spell_list_id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  check "16 byte spell_id" {
    expr = "length(`spell_id`) = 16"
  }
  check "16 byte spell_list_id" {
    expr = "length(`spell_list_id`) = 16"
  }
  without_rowid = true
}
schema "main" {
}
