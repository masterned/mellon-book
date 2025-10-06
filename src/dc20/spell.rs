use anyhow::anyhow;

#[derive(Clone, Debug, PartialEq)]
pub struct SpellSchool {
    id: uuid::Uuid,
    name: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Range {
    #[default]
    Caster,
    Spaces(u64),
    Touch,
}

impl Range {
    fn parse(kind: &str, value: Option<i64>) -> anyhow::Result<Range> {
        match kind {
            "Self" => Ok(Range::Caster),
            "Touch" => Ok(Range::Touch),
            "Spaces" => Ok(Range::Spaces(value.ok_or(anyhow!("invalid range"))? as u64)),
            _ => Err(anyhow!("invalid range")),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Duration {
    #[default]
    Instant,
    Minutes(u64),
    Hours(u64),
    Rounds(u64),
}

impl Duration {
    fn parse(kind: &str, value: Option<i64>) -> anyhow::Result<Duration> {
        match kind {
            "Instant" => Ok(Duration::Instant),
            "Minutes" => Ok(Duration::Minutes(
                value.ok_or(anyhow!("invalid duration"))? as u64
            )),
            "Hours" => Ok(Duration::Hours(
                value.ok_or(anyhow!("invalid duration"))? as u64
            )),
            "Rounds" => Ok(Duration::Rounds(
                value.ok_or(anyhow!("invalid duration"))? as u64
            )),
            _ => Err(anyhow!("invalid duration")),
        }
    }
}

#[derive(turann::Builder, Clone, Debug, PartialEq)]
pub struct PointEnhancement {
    #[builder(default = uuid::Uuid::now_v7)]
    pub id: uuid::Uuid,
    pub name: String,
    pub action_point_cost: u64,
    pub mana_point_cost: u64,
    pub description: String,
}

#[derive(turann::Builder, Clone, Debug, PartialEq)]
pub struct Spell {
    #[builder(default = uuid::Uuid::now_v7)]
    pub id: uuid::Uuid,
    pub name: String,
    pub school: SpellSchool,
    pub has_verbal: bool,
    pub has_somatic: bool,
    pub has_material: bool,
    pub action_point_cost: u64,
    pub mana_point_cost: u64,
    pub range: Range,
    pub duration: Duration,
    pub sustained: bool,
    pub description: String,
    pub point_enhancements: Vec<PointEnhancement>,
}

impl Spell {
    pub async fn load(pool: &sqlx::SqlitePool, id: uuid::Uuid) -> anyhow::Result<Spell> {
        let row = sqlx::query!(
            r#"
                SELECT `spell_id` AS "id: uuid::Uuid"
                    , spell.`name`
                    , `spell_school_id` AS "school_id: uuid::Uuid"
                    , school.`name` AS "school_name"
                    , has_verbal
                    , has_somatic
                    , COALESCE(EXISTS (SELECT 1 FROM `spell_material_components` WHERE `spell_id` = ?1), false) AS "has_material: bool"
                    , action_point_cost
                    , mana_point_cost
                    , range_kind
                    , range_value
                    , duration_kind
                    , duration_value
                    , sustained
                    , description
                FROM `spells` AS spell
                JOIN `spell_schools` AS school
                    USING (`spell_school_id`)
                WHERE `spell_id` = ?1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        let point_enhancements = sqlx::query_as!(
            PointEnhancement,
            r#"
                SELECT `point_enhancement_id` AS "id: uuid::Uuid"
                    , `name`
                    , `action_point_cost` AS "action_point_cost: u64"
                    , `mana_point_cost` AS "mana_point_cost: u64"
                    , `description`
                FROM `point_enhancements`
                JOIN `point_enhancements_spells`
                    USING (`point_enhancement_id`)
                WHERE `spell_id` = ?1
                ;
            "#,
            id
        )
        .fetch_all(pool)
        .await?;

        Ok(Self {
            id: row.id,
            name: row.name,
            school: SpellSchool {
                id: row.school_id,
                name: row.school_name,
            },
            has_verbal: row.has_verbal,
            has_somatic: row.has_somatic,
            has_material: row.has_material.unwrap_or_default(),
            action_point_cost: row.action_point_cost as u64,
            mana_point_cost: row.mana_point_cost as u64,
            range: Range::parse(&row.range_kind, row.range_value)?,
            duration: Duration::parse(&row.duration_kind, row.duration_value)?,
            sustained: row.sustained,
            description: row.description,
            point_enhancements,
        })
    }
}

#[derive(turann::Builder, Clone, Debug, Default, PartialEq)]
pub struct SpellList {
    #[builder(default = uuid::Uuid::now_v7)]
    pub id: uuid::Uuid,
    pub name: String,
    #[builder(each = "spell")]
    pub spells: Vec<Spell>,
}

impl SpellList {
    pub async fn load(pool: &sqlx::SqlitePool, id: uuid::Uuid) -> anyhow::Result<SpellList> {
        let list_details = sqlx::query!(
            r#"
                SELECT `spell_list_id` AS "id: uuid::Uuid"
                    , `name`
                FROM `spell_lists`
                WHERE `spell_list_id` = ?1
                LIMIT 1
                ;
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        let spell_ids = sqlx::query!(
            r#"
                SELECT `spell_id`
                FROM `spells_spell_lists`
                WHERE `spell_list_id` = ?1
                ;
            "#,
            id
        )
        .fetch_all(pool)
        .await?;

        let mut spells = vec![];

        for spell_id_row in spell_ids {
            spells.push(
                Spell::load(
                    pool,
                    uuid::Uuid::from_slice(spell_id_row.spell_id.as_slice()).expect("boom"),
                )
                .await?,
            );
        }

        Ok(Self {
            id: list_details.id,
            name: list_details.name,
            spells,
        })
    }
}
