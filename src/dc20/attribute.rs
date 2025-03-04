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
pub struct Attribute {
    pub name: String,
    pub score: isize,
    pub save_proficiency: bool,
    pub skills: Vec<Skill>,
}

impl Attribute {
    #[must_use]
    pub fn calc_save(&self, level: Level) -> isize {
        if self.save_proficiency {
            self.score
                .checked_add_unsigned(level.calc_combat_mastery())
                .unwrap()
        } else {
            self.score
        }
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
    pub fn calc_score(&self, attribute: &Attribute) -> isize {
        if let Some(mastery) = self.mastery {
            mastery as isize + attribute.score
        } else {
            attribute.score
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _attribute_without_save_mastery_should_have_save_of_attribute_score() {
        let attribute = Attribute {
            name: "Test Stat".into(),
            score: 3,
            save_proficiency: false,
            skills: vec![],
        };

        assert_eq!(attribute.calc_save(Level::default()), attribute.score);
    }

    #[test]
    fn _attribute_with_save_mastery_should_add_combat_mastery_to_save() {
        let attribute = Attribute {
            name: "Test Stat".into(),
            score: 3,
            save_proficiency: true,
            skills: vec![],
        };
        let level = Level::default();
        let combat_mastery = level.calc_combat_mastery() as isize;

        assert_eq!(attribute.calc_save(level), attribute.score + combat_mastery)
    }

    mod skill {
        use super::*;

        #[test]
        fn _skill_without_mastery_should_have_same_score_as_attribute() {
            let skill = Skill::new("Test Skill");
            let attribute = Attribute {
                name: "Intelligence".into(),
                score: 3,
                save_proficiency: true,
                skills: vec![skill.clone()],
            };

            assert_eq!(skill.calc_score(&attribute), attribute.score);
        }

        #[test]
        fn _skill_with_mastery_should_add_mastery_value_to_score() {
            let mut skill = Skill::new("Test Skill");
            skill.set_mastery(Mastery::Novice);

            let attribute = Attribute {
                name: "Intelligence".into(),
                score: 3,
                save_proficiency: true,
                skills: vec![skill.clone()],
            };

            assert_eq!(skill.calc_score(&attribute), attribute.score + 2);
        }
    }
}
