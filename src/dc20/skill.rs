use turann::Builder;
use uuid::Uuid;

use crate::dc20::Attribute;

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Skill {
    #[builder(default = Uuid::now_v7)]
    pub id: Uuid,
    pub name: String,
    pub attribute_id: Uuid,
}

impl Skill {
    pub async fn load(pool: &sqlx::SqlitePool, id: Uuid) -> sqlx::Result<Skill> {
        sqlx::query_as!(
            Skill,
            r#"
                SELECT id AS "id: Uuid", name, attribute_id AS "attribute_id: Uuid"
                FROM skills
                WHERE `id` = ?1
                LIMIT 1;
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn save(self, pool: &sqlx::SqlitePool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;

        let Skill {
            id,
            name,
            attribute_id,
        } = self;

        sqlx::query!(
            r#"
                INSERT INTO skills (`id`, `name`, `attribute_id`)
                VALUES ( ?1, ?2, ?3 )
                ON CONFLICT(`id`) DO UPDATE SET
                    name = ?2,
                    attribute_id = ?3;
            "#,
            id,
            name,
            attribute_id
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    pub async fn load_attribute(&self, pool: &sqlx::SqlitePool) -> sqlx::Result<Attribute> {
        let Skill {
            ref attribute_id, ..
        } = self;

        sqlx::query_as!(
            Attribute,
            r#"
                SELECT `id` AS "id: uuid::Uuid"
                    , `name`
                FROM `attributes`
                WHERE `id` = ?1
                LIMIT 1
                ;
            "#,
            attribute_id
        )
        .fetch_one(pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "API changed"]
    fn _skill_without_mastery_should_have_same_score_as_attribute() {
        todo!()
    }

    #[test]
    #[ignore = "API changed"]
    fn _skill_with_mastery_should_add_mastery_value_to_score() {
        todo!()
    }
}
