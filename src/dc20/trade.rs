use turann::Builder;

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Trade {
    #[builder(default = uuid::Uuid::now_v7)]
    pub id: uuid::Uuid,
    pub name: String,
    pub attribute_id: uuid::Uuid,
}

impl Trade {
    pub async fn load(pool: &sqlx::SqlitePool, id: uuid::Uuid) -> sqlx::Result<Trade> {
        sqlx::query_as!(
            Trade,
            r#"
                SELECT
                    `id` AS "id: uuid::Uuid",
                    `name`,
                    `attribute_id` AS "attribute_id: uuid::Uuid"
                FROM `trades`
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

        let Trade {
            id,
            name,
            attribute_id,
        } = self;

        sqlx::query!(
            r#"
                INSERT INTO trades (`id`, `name`, `attribute_id`)
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
}
