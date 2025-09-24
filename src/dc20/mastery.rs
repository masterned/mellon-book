#[derive(turann::Builder, Clone, Debug, PartialEq, Eq)]
pub struct Mastery {
    #[builder(default = uuid::Uuid::now_v7)]
    pub id: uuid::Uuid,
    pub name: String,
    pub bonus: u8,
}

impl Mastery {
    pub async fn load(pool: &sqlx::SqlitePool, id: uuid::Uuid) -> sqlx::Result<Mastery> {
        sqlx::query_as!(
            Mastery,
            r#"
                SELECT `mastery_id` AS "id: uuid::Uuid",
                    `name`,
                    `bonus` AS "bonus: u8"
                FROM `masteries`
                WHERE `mastery_id` = ?1
                LIMIT 1;
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }
}
