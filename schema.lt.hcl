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
    null           = false
    type           = integer
    auto_increment = true
  }
  column "uuid" {
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
    expr = "length(uuid) = 16"
  }
  index "players_uuid_uniq" {
    unique  = true
    columns = [column.uuid]
  }
}
table "characters" {
  schema = schema.main
  column "id" {
    null           = false
    type           = integer
    auto_increment = true
  }
  column "uuid" {
    null = false
    type = blob
  }
  column "name" {
    null = false
    type = text
  }
  column "creator_id" {
    null = false
    type = integer
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
    expr = "length(uuid) = 16"
  }
  index "characters_uuid_uniq" {
    unique  = true
    columns = [column.uuid]
  }
}
table "ancestries" {
  schema = schema.main
  column "id" {
    null           = false
    type           = integer
    auto_increment = true
  }
  column "uuid" {
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
    expr = "length(uuid) = 16"
  }
  index "ancestries_uuid_uniq" {
    unique  = true
    columns = [column.uuid]
  }
}
table "ancestry_traits" {
  schema = schema.main
  column "id" {
    null           = false
    type           = integer
    auto_increment = true
  }
  column "uuid" {
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
    expr = "length(uuid) = 16"
  }
  index "ancestry_traits_uuid_uniq" {
    unique  = true
    columns = [column.uuid]
  }
}
table "ancestries_ancestry_traits" {
  schema = schema.main
  column "ancestry_id" {
    null = false
    type = integer
  }
  column "ancestry_trait_id" {
    null = false
    type = integer
  }
  column "expanded" {
    null    = false
    type    = boolean
    default = false
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
  column "character_id" {
    null = false
    type = integer
  }
  column "level" {
    null = false
    type = integer
  }
  column "uuid" {
    null = false
    type = blob
  }
  primary_key {
    columns = [column.character_id, column.level]
  }
  foreign_key "character_fk" {
    columns     = [column.character_id]
    ref_columns = [table.characters.column.id]
    on_update   = NO_ACTION
    on_delete   = CASCADE
  }
  check "16 byte uuid" {
    expr = "length(uuid) = 16"
  }
  index "character_levels_uuid_uniq" {
    unique  = true
    columns = [column.uuid]
  }
}
table "ancestry_traits_character_levels" {
  schema = schema.main
  column "ancestry_trait_id" {
    null = false
    type = integer
  }
  column "character_id" {
    null = false
    type = integer
  }
  column "level" {
    null = false
    type = integer
  }
  foreign_key "character_level_fk" {
    columns     = [column.character_id, column.level]
    ref_columns = [table.character_levels.column.character_id, table.character_levels.column.level]
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
    type = integer
  }
  column "character_id" {
    null = false
    type = integer
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
schema "main" {
}
