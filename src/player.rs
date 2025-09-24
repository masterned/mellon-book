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
    id: Uuid,
    #[builder(validate = Self::validate_name)]
    name: String,
}

impl Player {
    pub async fn load(pool: &sqlx::SqlitePool, uuid: Uuid) -> anyhow::Result<Player> {
        let result = sqlx::query_as!(
            Player,
            r#"
                SELECT player_id as "id: Uuid", name
                FROM players
                WHERE player_id = ?
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

        let Player { id, name } = self;

        let id = sqlx::query!(
            r#"
                INSERT INTO players (`player_id`, `name`)
                VALUES ( ?1, ?2 )
                ON CONFLICT(`player_id`) DO UPDATE SET
                    name = ?2;
            "#,
            id,
            name
        )
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
