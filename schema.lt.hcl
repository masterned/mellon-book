table "_sqlx_migrations" {
  schema = schema.main
  column "version" {
    null = true
    type = bigint
  }
  column "description" {
    null = false
    type = text
  }
  column "installed_on" {
    null    = false
    type    = sql("timestamp")
    default = sql("CURRENT_TIMESTAMP")
  }
  column "success" {
    null = false
    type = boolean
  }
  column "checksum" {
    null = false
    type = blob
  }
  column "execution_time" {
    null = false
    type = bigint
  }
  primary_key {
    columns = [column.version]
  }
}
table "players" {
  schema = schema.main
  column "id" {
    null = false
    type = blob
  }
  column "name" {
    null = false
    type = text
  }
  primary_key {
    columns = [column.id]
  }
  check "16 byte uuid" {
    expr = "length(`id`) = 16"
  }
  without_rowid = true
}
table "characters" {
  schema = schema.main
  column "id" {
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
    columns = [column.id]
  }
  foreign_key "creator_fk" {
    columns     = [column.creator_id]
    ref_columns = [table.players.column.id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  check "16 byte uuid" {
    expr = "length(`id`) = 16"
  }
  without_rowid = true
}
table "ancestries" {
  schema = schema.main
  column "id" {
    null = false
    type = blob
  }
  column "name" {
    null = false
    type = text
  }
  primary_key {
    columns = [column.id]
  }
  check "16 byte uuid" {
    expr = "length(`id`) = 16"
  }
  without_rowid = true
}
table "ancestry_traits" {
  schema = schema.main
  column "id" {
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
    columns = [column.id]
  }
  check "16 byte uuid" {
    expr = "length(`id`) = 16"
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
    ref_columns = [table.ancestry_traits.column.id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  foreign_key "ancestry_fk" {
    columns     = [column.ancestry_id]
    ref_columns = [table.ancestries.column.id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
}
table "character_levels" {
  schema = schema.main
  column "id" {
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
    columns = [column.id]
  }
  foreign_key "character_fk" {
    columns     = [column.character_id]
    ref_columns = [table.characters.column.id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  check "16 byte uuid" {
    expr = "length(`id`) = 16"
  }
  index "character_levels_character_level_uniq" {
    unique  = true
    columns = [column.character_id, column.level]
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
  foreign_key "character_level_fk" {
    columns     = [column.character_level_id]
    ref_columns = [table.character_levels.column.id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  foreign_key "ancestry_trait_fk" {
    columns     = [column.ancestry_trait_id]
    ref_columns = [table.ancestries.column.id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
}
table "ancestries_characters" {
  schema = schema.main
  column "ancestry_id" {
    null = false
    type = blob
  }
  column "character_id" {
    null = false
    type = blob
  }
  foreign_key "character_fk" {
    columns     = [column.character_id]
    ref_columns = [table.characters.column.id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  foreign_key "ancestry_fk" {
    columns     = [column.ancestry_id]
    ref_columns = [table.ancestries.column.id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
}
table "attributes" {
  schema = schema.main
  column "id" {
    type = blob
    null = false
  }
  column "name" {
    type = text
    null = false
  }
  primary_key {
    columns = [column.id]
  }
  check "16 byte id" {
    expr = "length(`id`) = 16"
  }
}
table "skills" {
  schema = schema.main
  column "id" {
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
    columns = [column.id]
  }
  foreign_key "attribute_fk" {
    columns = [column.attribute_id]
    ref_columns = [table.attributes.column.id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  check "16 byte id" {
    expr = "length(`id`) = 16"
  }
  check "16 byte attribute_id" {
    expr = "length(`attribute_id`) = 16"
  }
  without_rowid = true
}
table "trades" {
  schema = schema.main
  column "id" {
    type = blob
    null = false
  }
  column "name" {
    type = text
    null = false
  }
  primary_key {
    columns = [column.id]
  }
  check "16 byte id" {
    expr = "length(`id`) = 16"
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
    ref_columns = [table.attributes.column.id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
  foreign_key "trade_fk" {
    columns = [column.trade_id]
    ref_columns = [table.trades.column.id]
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
schema "main" {
}
