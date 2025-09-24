use turann::Builder;
use uuid::Uuid;

use crate::dc20::{Language, Skill, Trade};

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Background {
    #[builder(default = Uuid::new_v4)]
    pub id: Uuid,
    #[builder(validate = Self::validate_name)]
    pub name: String,
}

impl BackgroundBuilder {
    fn validate_name(name: String) -> Result<String, BackgroundBuilderError> {
        if name.is_empty() {
            return Err(BackgroundBuilderError::InvalidField {
                field_name: "name".into(),
                message: "cannot be empty".into(),
            });
        }

        Ok(name)
    }
}

impl Background {
    pub async fn load(pool: &sqlx::SqlitePool, id: uuid::Uuid) -> sqlx::Result<Background> {
        sqlx::query_as!(
            Background,
            r#"
                SELECT `background_id` AS "id: uuid::Uuid", `name`
                FROM `backgrounds`
                WHERE `background_id` = ?1
                LIMIT 1;
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn save(self, pool: &sqlx::SqlitePool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;

        let Background { id, name } = self;

        sqlx::query!(
            r#"
                INSERT INTO `backgrounds` VALUES
                (?1, ?2)
                ON CONFLICT (`background_id`) DO UPDATE
                SET `name` = ?2;
            "#,
            id,
            name
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    pub async fn load_languages(&self, pool: &sqlx::SqlitePool) -> sqlx::Result<Vec<Language>> {
        sqlx::query_as!(
            Language,
            r#"
                SELECT l.`language_id` AS "id: uuid::Uuid"
                    , l.`name`
                FROM `languages` AS l
                JOIN `backgrounds_languages` AS b_l
                    USING (`language_id`)
                WHERE b_l.`background_id` = ?1
                ;
            "#,
            self.id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn load_skills(&self, pool: &sqlx::SqlitePool) -> sqlx::Result<Vec<Skill>> {
        let Background { ref id, .. } = self;

        sqlx::query_as!(
            Skill,
            r#"
                SELECT s.`skill_id` AS "id: uuid::Uuid"
                    , s.`name`
                    , s.`attribute_id` AS "attribute_id: uuid::Uuid"
                FROM `skills` AS s
                JOIN `backgrounds_skills` AS b_s
                    USING (`skill_id`)
                WHERE b_s.`background_id` = ?1
                ;
            "#,
            id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn load_trades(&self, pool: &sqlx::SqlitePool) -> sqlx::Result<Vec<Trade>> {
        let Background { ref id, .. } = self;

        sqlx::query_as!(
            Trade,
            r#"
                SELECT t.`trade_id` AS "id: uuid::Uuid"
                    , t.`name`
                FROM `trades` AS t
                JOIN `backgrounds_trades` AS b_t
                    USING (`trade_id`)
                WHERE b_t.`background_id` = ?1
                ;
            "#,
            id
        )
        .fetch_all(pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "API changed"]
    fn _require_name_and_at_least_one_skill_trade_and_language_to_build_background(
    ) -> anyhow::Result<()> {
        todo!()
    }
}
