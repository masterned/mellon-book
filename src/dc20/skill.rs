use uuid::Uuid;

use super::Level;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mastery {
    Novice = 2,
    Adept = 4,
    Expert = 6,
    Master = 8,
    GrandMaster = 10,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stat {
    pub name: String,
    pub score: isize,
    pub save_proficiency: bool,
    pub skills: Vec<Skill>,
}

impl Stat {
    #[must_use]
    pub fn calc_save(&self, level: Level) -> isize {
        self.score
            .checked_add_unsigned(level.calc_combat_mastery())
            .unwrap()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Skill {
    pub uuid: Uuid,
    pub name: String,
    pub mastery: Option<Mastery>,
}

impl Skill {
    pub fn new(name: impl Into<String>) -> Self {
        Skill {
            uuid: Uuid::new_v4(),
            name: name.into(),
            mastery: None,
        }
    }

    pub fn set_mastery(&mut self, mastery: Mastery) {
        let _ = self.mastery.insert(mastery);
    }

    #[must_use]
    pub fn calc_score(&self, stat: &Stat) -> isize {
        if let Some(mastery) = self.mastery {
            mastery as isize + stat.score
        } else {
            stat.score
        }
    }
}
