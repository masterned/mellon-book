use turann::Builder;
use uuid::Uuid;

impl PlayerBuilder {
    fn validate_name(name: String) -> Result<String, PlayerBuilderError> {
        if name.is_empty() {
            return Err(PlayerBuilderError::InvalidField {
                field_name: "name".into(),
                message: "cannot be empty".into(),
            });
        }

        Ok(name)
    }
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Player {
    #[builder(default = Uuid::new_v4)]
    uuid: Uuid,
    #[builder(validate = Self::validate_name)]
    name: String,
}

impl Player {
    pub async fn get_player_by_uuid(pool: &sqlx::SqlitePool, uuid: Uuid) -> anyhow::Result<Player> {
        let result = sqlx::query_as!(
            Player,
            r#"
                SELECT uuid as "uuid: Uuid", name
                FROM players
                WHERE uuid = ?
                LIMIT 1
            "#,
            uuid
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn save(self, pool: &sqlx::SqlitePool) -> anyhow::Result<i64> {
        let mut conn = pool.acquire().await?;

        let Player { uuid, name } = self;

        let id = sqlx::query!(
            r#"
                INSERT INTO players (`uuid`, `name`)
                VALUES ( ?, ? );
            "#,
            uuid,
            name
        )
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
