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

#[derive(Clone, Debug, PartialEq)]
pub struct PointEnhancement {
    pub name: String,
    pub cost: Vec<Points>,
    pub description: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Spell {
    pub uuid: Uuid,
    pub name: String,
    pub family: Option<SpellFamily>,
    pub cost: Vec<Points>,
    pub range: Range,
    pub duration: Duration,
    pub point_enhancements: Vec<PointEnhancement>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SpellList {
    pub uuid: Uuid,
    pub name: String,
    pub spells: Vec<Spell>,
}
