#[derive(turann::Builder, Clone, Debug, PartialEq, Eq)]
pub struct Language {
    #[builder(default = uuid::Uuid::now_v7)]
    pub id: uuid::Uuid,
    #[builder(validate = LanguageBuilder::validate_name)]
    pub name: String,
}

impl LanguageBuilder {
    fn validate_name(name: impl Into<String>) -> Result<String, LanguageBuilderError> {
        let name: String = name.into();

        if name.is_empty() {
            return Err(LanguageBuilderError::InvalidField {
                field_name: "name".into(),
                message: "cannot be empty".into(),
            });
        }

        Ok(name)
    }
}

impl Language {
    pub async fn load(pool: &sqlx::SqlitePool, id: uuid::Uuid) -> sqlx::Result<Language> {
        sqlx::query_as!(
            Language,
            r#"
                SELECT `id` AS "id: uuid::Uuid"
                    , `name`
                FROM `languages`
                WHERE `id` = ?1
                LIMIT 1
                ;
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn save(self, pool: &mut sqlx::SqlitePool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;

        let Language { id, name } = self;

        sqlx::query!(
            r#"
                INSERT INTO `languages`
                VALUES (?1, ?2)
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
