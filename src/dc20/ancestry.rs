use turann::Builder;
use uuid::Uuid;

#[derive(Builder, Clone, Debug, PartialEq, Eq)]
pub struct Ancestry {
    #[builder(default = Uuid::now_v7)]
    pub id: Uuid,
    pub name: String,
}

impl Ancestry {
    pub async fn load(pool: &sqlx::SqlitePool, id: Uuid) -> sqlx::Result<Ancestry> {
        sqlx::query_as!(
            Ancestry,
            r#"
                SELECT `ancestry_id` as "id: Uuid", name
                FROM ancestries
                WHERE ancestry_id = ?
                LIMIT 1;
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn save(self, pool: &sqlx::SqlitePool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;

        let Ancestry { id, name } = self;

        sqlx::query!(
            r#"
                INSERT INTO ancestries (`ancestry_id`, `name`)
                VALUES (?1, ?2)
                ON CONFLICT(`ancestry_id`) DO UPDATE SET
                    `name` = ?2;
            "#,
            id,
            name
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}

#[derive(Builder, Clone, Debug, Default, PartialEq)]
pub struct AncestryTrait {
    #[builder(default = Uuid::now_v7)]
    pub id: Uuid,
    pub name: String,
    pub description: String,
    #[builder(default)]
    pub cost: i8,
}

impl AncestryTrait {
    pub async fn load(pool: &sqlx::SqlitePool, id: Uuid) -> sqlx::Result<AncestryTrait> {
        sqlx::query_as!(
            AncestryTrait,
            r#"
                SELECT ancestry_trait_id as "id: Uuid"
                    , name
                    , description
                    , cost as "cost: i8"
                FROM ancestry_traits
                WHERE ancestry_trait_id = ?1
                LIMIT 1;
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn save(self, pool: &mut sqlx::SqlitePool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;

        let AncestryTrait {
            id,
            name,
            description,
            cost,
        } = self;

        sqlx::query!(
            r#"
                INSERT INTO ancestry_traits (
                    `ancestry_trait_id`
                    , `name`
                    , `description`
                    , `cost`
                )
                VALUES (?1, ?2, ?3, ?4)
                ON CONFLICT(`ancestry_trait_id`) DO UPDATE SET
                    `name` = ?2,
                    `description` = ?3,
                    `cost` = ?4;
            "#,
            id,
            name,
            description,
            cost
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;
}
