use std::{error::Error, fmt, str::FromStr};

use turann::Builder;

use crate::dc20::{Level, Skill};

#[derive(Builder, Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub id: uuid::Uuid,
    pub name: String,
}

impl Attribute {
    pub async fn load(pool: &sqlx::SqlitePool, id: uuid::Uuid) -> sqlx::Result<Attribute> {
        sqlx::query_as!(
            Attribute,
            r#"
                SELECT `id` AS "id: uuid::Uuid", `name`
                FROM attributes 
                WHERE `id` = ?
                LIMIT 1;
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn save(self, pool: &sqlx::SqlitePool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;

        let Attribute { id, name } = self;

        sqlx::query!(
            r#"
                INSERT INTO `attributes` VALUES
                (?1, ?2)
                ON CONFLICT (`id`) DO UPDATE
                    SET `name` = ?2
                ;
            "#,
            id,
            name
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AttributeName {
    Prime,
    Might,
    Agility,
    Charisma,
    Intelligence,
}

impl fmt::Display for AttributeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct AttributeNameParseError();

impl fmt::Display for AttributeNameParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse AttributeName")
    }
}

impl Error for AttributeNameParseError {}

impl FromStr for AttributeName {
    type Err = AttributeNameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Prime" => Ok(AttributeName::Prime),
            "Might" => Ok(AttributeName::Might),
            "Agility" => Ok(AttributeName::Agility),
            "Charisma" => Ok(AttributeName::Charisma),
            "Intelligence" => Ok(AttributeName::Intelligence),
            _ => Err(AttributeNameParseError()),
        }
    }
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Attributes {
    prime: AttributeLevel,
    might: AttributeLevel,
    agility: AttributeLevel,
    charisma: AttributeLevel,
    intelligence: AttributeLevel,
}

impl Attributes {
    pub fn prime(&self) -> &AttributeLevel {
        &self.prime
    }

    pub fn might(&self) -> &AttributeLevel {
        &self.might
    }

    pub fn agility(&self) -> &AttributeLevel {
        &self.agility
    }

    pub fn charisma(&self) -> &AttributeLevel {
        &self.charisma
    }

    pub fn intelligence(&self) -> &AttributeLevel {
        &self.intelligence
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AttributeLevel {
    pub base_score: isize,
    pub save_proficiency: bool,
    pub skills: Vec<Skill>,
}

impl AttributeLevel {
    pub fn new() -> Self {
        AttributeLevel::default()
    }

    pub fn with_base_score(mut self, score: isize) -> Self {
        self.base_score = score;

        self
    }

    pub fn with_save_proficiency(mut self) -> Self {
        self.save_proficiency = true;

        self
    }

    pub fn with_skill(mut self, skill: Skill) -> Self {
        self.skills.push(skill);

        self
    }

    #[must_use]
    pub fn calc_save(&self, level: Level) -> isize {
        if self.save_proficiency {
            self.base_score
                .checked_add_unsigned(level.calc_combat_mastery())
                .unwrap()
        } else {
            self.base_score
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _attribute_without_save_mastery_should_have_save_of_attribute_score() {
        let attribute = AttributeLevel {
            base_score: 3,
            save_proficiency: false,
            skills: vec![],
        };

        let level = Level::builder()
            .character_id(uuid::Uuid::default())
            .build()
            .unwrap();

        assert_eq!(attribute.calc_save(level), attribute.base_score);
    }

    #[test]
    fn _attribute_with_save_mastery_should_add_combat_mastery_to_save() {
        let attribute = AttributeLevel {
            base_score: 3,
            save_proficiency: true,
            skills: vec![],
        };
        let level = Level::builder()
            .character_id(uuid::Uuid::default())
            .build()
            .unwrap();
        let combat_mastery = level.calc_combat_mastery() as isize;

        assert_eq!(
            attribute.calc_save(level),
            attribute.base_score + combat_mastery
        )
    }
}
