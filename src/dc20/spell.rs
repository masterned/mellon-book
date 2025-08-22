use turann::Builder;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpellFamily {
    Conjuration,
    Divination,
    Enchantment,
    Illusion,
    Restoration,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Points {
    Action(usize),
    Mana(usize),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Range {
    #[default]
    Caster,
    Spaces(usize),
    Touch,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Duration {
    #[default]
    Instant,
    Min {
        minutes: usize,
        concentration: bool,
    },
    Hour {
        hours: usize,
        concentration: bool,
    },
    Round {
        rounds: usize,
    },
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

#[derive(Builder, Clone, Debug, Default, PartialEq)]
pub struct Spell {
    #[builder(default = Uuid::new_v4)]
    pub uuid: Uuid,
    pub name: String,
    pub family: Option<SpellFamily>,
    #[builder(each = "cost")]
    pub cost: Vec<Points>,
    pub range: Range,
    pub duration: Duration,
    #[builder(each = "point_enhancement")]
    pub point_enhancements: Vec<PointEnhancement>,
}

#[derive(Builder, Clone, Debug, Default, PartialEq)]
pub struct SpellList {
    #[builder(default = Uuid::new_v4)]
    pub uuid: Uuid,
    pub name: String,
    #[builder(each = "spell")]
    pub spells: Vec<Spell>,
}
