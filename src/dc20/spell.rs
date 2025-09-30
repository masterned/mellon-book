use anyhow::anyhow;
use turann::Builder;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct SpellSchool {
    id: uuid::Uuid,
    name: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Points {
    Action(u64),
    Mana(u64),
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
            "self" => Ok(Range::Caster),
            "touch" => Ok(Range::Touch),
            "spaces" => Ok(Range::Spaces(value.ok_or(anyhow!("invalid range"))? as u64)),
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
            "instant" => Ok(Duration::Instant),
            "minutes" => Ok(Duration::Minutes(
                value.ok_or(anyhow!("invalid duration"))? as u64
            )),
            "hours" => Ok(Duration::Hours(
                value.ok_or(anyhow!("invalid duration"))? as u64
            )),
            "rounds" => Ok(Duration::Rounds(
                value.ok_or(anyhow!("invalid duration"))? as u64
            )),
            _ => Err(anyhow!("invalid duration")),
        }
    }
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct PointEnhancement {
    #[builder(default = Uuid::new_v4)]
    pub uuid: Uuid,
    pub name: String,
    #[builder(each = "cost")]
    pub cost: Vec<Points>,
    pub description: String,
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Spell {
    #[builder(default = Uuid::new_v4)]
    pub uuid: Uuid,
    pub name: String,
    pub school: SpellSchool,
    pub has_verbal: bool,
    pub has_somatic: bool,
    pub has_material: bool,
    pub cost: Vec<Points>,
    pub range: Range,
    pub duration: Duration,
    pub sustained: bool,
    pub description: String,
}

impl Spell {
    pub async fn load(pool: &sqlx::SqlitePool, id: uuid::Uuid) -> anyhow::Result<Spell> {
        let row = sqlx::query!(
            r#"
                SELECT `spell_id` AS "uuid: uuid::Uuid"
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

        Ok(Self {
            uuid: row.uuid,
            name: row.name,
            school: SpellSchool {
                id: row.school_id,
                name: row.school_name,
            },
            has_verbal: row.has_verbal,
            has_somatic: row.has_somatic,
            has_material: row.has_material.unwrap_or_default(),
            cost: vec![
                Points::Action(row.action_point_cost as u64),
                Points::Mana(row.mana_point_cost as u64),
            ],
            range: Range::parse(&row.range_kind, row.range_value)?,
            duration: Duration::parse(&row.duration_kind, row.duration_value)?,
            sustained: row.sustained,
            description: row.description,
        })
    }
}

#[derive(Builder, Clone, Debug, Default, PartialEq)]
pub struct SpellList {
    #[builder(default = Uuid::new_v4)]
    pub uuid: Uuid,
    pub name: String,
    #[builder(each = "spell")]
    pub spells: Vec<Spell>,
}
