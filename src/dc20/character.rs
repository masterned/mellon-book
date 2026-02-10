use turann::Builder;
use uuid::Uuid;

use crate::{
    dc20::{Ancestry, AncestryTrait, Attributes, Background, Class, Subclass},
    player::Player,
};

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Character {
    #[builder(default = Uuid::new_v4)]
    id: Uuid,
    player: Player,
    character_name: String,
    #[builder(each = "ancestry_trait")]
    ancestry_traits: Vec<AncestryTrait>,
    background: Background,
}

impl Character {
    #[must_use]
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    #[must_use]
    pub fn player_name(&self) -> &str {
        self.player.name()
    }

    #[must_use]
    pub fn character_name(&self) -> &str {
        &self.character_name
    }

    #[must_use]
    pub fn background(&self) -> &Background {
        &self.background
    }

    pub async fn load_level(&self, pool: &sqlx::SqlitePool, level: u32) -> sqlx::Result<Level> {
        sqlx::query_as!(
            Level,
            r#"
                SELECT `character_level_id` AS "id: uuid::Uuid"
                    , `character_id` AS "character_id: uuid::Uuid"
                    , `level` AS "level: u32"
                FROM `character_levels`
                WHERE `character_id` = ?1
                    AND `level` = ?2
                LIMIT 1
                ;
            "#,
            self.id,
            level
        )
        .fetch_one(pool)
        .await
    }

    pub async fn load_max_level(&self, pool: &sqlx::SqlitePool) -> sqlx::Result<Level> {
        sqlx::query_as!(
            Level,
            r#"
                SELECT `character_level_id` AS "id: uuid::Uuid"
                    , `character_id` AS "character_id: uuid::Uuid"
                    , `level` AS "level: u32"
                FROM `character_levels`
                WHERE `character_id` = ?1
                ORDER BY `level` DESC
                LIMIT 1
                ;
            "#,
            self.id
        )
        .fetch_one(pool)
        .await
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BaseAttributeValue {
    pub character_level_id: uuid::Uuid,
    pub attribute_id: uuid::Uuid,
    pub value: i32,
}

#[derive(Builder, Clone, Debug, PartialEq, PartialOrd)]
pub struct Level {
    #[builder(default = uuid::Uuid::now_v7)]
    pub id: uuid::Uuid,
    #[builder(default)]
    pub character_id: uuid::Uuid,
    #[builder(default = LevelBuilder::default_level)]
    pub level: u32,
}

impl LevelBuilder {
    fn default_level() -> u32 {
        1
    }
}

impl Level {
    pub fn calc_combat_mastery(&self) -> usize {
        let level = self.level as usize;
        level.div_ceil(2)
    }

    pub async fn load(pool: &sqlx::SqlitePool, id: uuid::Uuid) -> sqlx::Result<Level> {
        sqlx::query_as!(
            Level,
            r#"
                SELECT `character_level_id` AS "id: uuid::Uuid"
                    , `character_id` AS "character_id: uuid::Uuid"
                    , `level` AS "level: u32"
                FROM `character_levels`
                WHERE `character_level_id` = ?1
                LIMIT 1
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn save(self, pool: &sqlx::SqlitePool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;

        let Level {
            id,
            character_id,
            level,
        } = self;

        sqlx::query!(
            "
                INSERT INTO `character_levels`
                VALUES (?1 ,?2 ,?3)
                ON CONFLICT (`character_level_id`) DO UPDATE
                    SET `character_id` = ?2
                    , `level` = ?3
                ;
            ",
            id,
            character_id,
            level
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    pub async fn load_ancestries(&self, pool: &sqlx::SqlitePool) -> sqlx::Result<Vec<Ancestry>> {
        sqlx::query_as!(
            Ancestry,
            r#"
                SELECT `ancestry_id` AS "id: uuid::Uuid"
                    , `name`
                FROM `ancestries`
                JOIN `ancestries_character_levels`
                    USING (`ancestry_id`)
                WHERE `character_level_id` = ?1
                ;
            "#,
            self.id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn load_base_attributes(&self, pool: &sqlx::SqlitePool) -> sqlx::Result<Attributes> {
        sqlx::query_as!(
            Attributes,
            r#"
                SELECT `prime`
                    , `might`
                    , `agility`
                    , `charisma`
                    , `intelligence`
                FROM `character_level_attributes`
                WHERE `character_level_id` = ?1
            "#,
            self.id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn load_classes(&self, pool: &sqlx::SqlitePool) -> sqlx::Result<Vec<Class>> {
        sqlx::query_as!(
            Class,
            r#"
                SELECT `class_id` AS "id: uuid::Uuid"
                    , `name`
                FROM `classes`
                JOIN `character_levels_classes`
                    USING (`class_id`)
                WHERE `character_level_id` = ?1
                ;
            "#,
            self.id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn load_sublasses(&self, pool: &sqlx::SqlitePool) -> sqlx::Result<Vec<Subclass>> {
        sqlx::query_as!(
            Subclass,
            r#"
                SELECT `subclass_id` AS "id: uuid::Uuid"
                    , `name`
                FROM `subclasses`
                JOIN `character_levels_subclasses`
                    USING (`subclass_id`)
                WHERE `character_level_id` = ?1
                ;
            "#,
            self.id
        )
        .fetch_all(pool)
        .await
    }
}

impl Default for Level {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
            character_id: Default::default(),
            level: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Defense {
    pub score: usize,
    pub reduction: usize,
}

impl Defense {
    #[must_use]
    pub fn heavy(&self) -> usize {
        self.score + 5
    }

    #[must_use]
    pub fn brutal(&self) -> usize {
        self.score + 10
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::dc20::*;

    use super::*;

    #[test]
    #[ignore = "API changed"]
    fn _all_fields_present_to_build_character() -> Result<(), Box<dyn Error>> {
        let mut builder = Character::builder();

        let result = builder.clone().build();
        assert_eq!(
            result,
            Err(CharacterBuilderError::missing_fields(&[
                "Player Name",
                "Character Name",
                "Class",
                "Background",
                "Attributes",
            ]))
        );

        let john_doe = Player::builder().name("John Doe")?.build()?;

        builder.player(john_doe.clone());

        let result = builder.clone().build();
        assert_eq!(
            result,
            Err(CharacterBuilderError::missing_fields(&[
                "Character Name",
                "Class",
                "Background",
                "Attributes",
            ]))
        );

        let result = builder.clone().build();
        assert_eq!(
            result,
            Err(CharacterBuilderError::missing_fields(&[
                "Character Name",
                "Class",
                "Background",
                "Attributes",
            ]))
        );

        let soldier = Background::builder().name("Soldier")?.build()?;

        builder
            .character_name("Johannas Doeworth")
            .background(soldier.clone());
        let mut character = builder.build()?;

        let char_id = Uuid::new_v4();
        character.id = char_id;

        assert_eq!(
            character,
            Character {
                id: char_id,
                player: john_doe,
                character_name: "Johannas Doeworth".to_string(),
                ancestry_traits: vec![],
                background: soldier,
            }
        );

        Ok(())
    }

    #[test]
    fn _combat_mastery_half_level_rounded_up() {
        let mut level = Level::default();
        assert_eq!(level.calc_combat_mastery(), 1);

        level.level = 2;
        assert_eq!(level.calc_combat_mastery(), 1);

        level.level = 3;
        assert_eq!(level.calc_combat_mastery(), 2);

        level.level = 4;
        assert_eq!(level.calc_combat_mastery(), 2);

        level.level = 5;
        assert_eq!(level.calc_combat_mastery(), 3);

        level.level = 6;
        assert_eq!(level.calc_combat_mastery(), 3);

        level.level = 7;
        assert_eq!(level.calc_combat_mastery(), 4);

        level.level = 8;
        assert_eq!(level.calc_combat_mastery(), 4);

        level.level = 9;
        assert_eq!(level.calc_combat_mastery(), 5);

        level.level = 10;
        assert_eq!(level.calc_combat_mastery(), 5);

        level.level = 11;
        assert_eq!(level.calc_combat_mastery(), 6);

        level.level = 12;
        assert_eq!(level.calc_combat_mastery(), 6);

        level.level = 13;
        assert_eq!(level.calc_combat_mastery(), 7);

        level.level = 14;
        assert_eq!(level.calc_combat_mastery(), 7);

        level.level = 15;
        assert_eq!(level.calc_combat_mastery(), 8);

        level.level = 16;
        assert_eq!(level.calc_combat_mastery(), 8);

        level.level = 17;
        assert_eq!(level.calc_combat_mastery(), 9);

        level.level = 18;
        assert_eq!(level.calc_combat_mastery(), 9);

        level.level = 19;
        assert_eq!(level.calc_combat_mastery(), 10);

        level.level = 20;
        assert_eq!(level.calc_combat_mastery(), 10);
    }
}
