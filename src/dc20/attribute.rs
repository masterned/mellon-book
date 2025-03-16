use std::{error::Error, fmt, str::FromStr};

use uuid::Uuid;

use crate::utils::{FieldAggregator, SwapResult};

use super::Level;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AttributeName {
    Prime,
    Might,
    Agility,
    Charisma,
    Intelligence,
}

impl fmt::Display for AttributeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct AttributeNameParseError();

impl fmt::Display for AttributeNameParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to parse AttributeName")
    }
}

impl Error for AttributeNameParseError {}

impl FromStr for AttributeName {
    type Err = AttributeNameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Prime" => Ok(AttributeName::Prime),
            "Might" => Ok(AttributeName::Might),
            "Agility" => Ok(AttributeName::Agility),
            "Charisma" => Ok(AttributeName::Charisma),
            "Intelligence" => Ok(AttributeName::Intelligence),
            _ => Err(AttributeNameParseError()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mastery {
    Novice = 2,
    Adept = 4,
    Expert = 6,
    Master = 8,
    GrandMaster = 10,
}

#[derive(Clone, Debug, Default)]
pub struct AttributesBuilder {
    prime: Option<Attribute>,
    might: Option<Attribute>,
    agility: Option<Attribute>,
    charisma: Option<Attribute>,
    intelligence: Option<Attribute>,
}

impl AttributesBuilder {
    pub fn new() -> Self {
        AttributesBuilder::default()
    }

    pub fn prime(mut self, prime: Attribute) -> Self {
        let _ = self.prime.insert(prime);

        self
    }

    pub fn might(mut self, might: Attribute) -> Self {
        let _ = self.might.insert(might);

        self
    }

    pub fn agility(mut self, agility: Attribute) -> Self {
        let _ = self.agility.insert(agility);

        self
    }

    pub fn charisma(mut self, charisma: Attribute) -> Self {
        let _ = self.charisma.insert(charisma);

        self
    }

    pub fn intelligence(mut self, intelligence: Attribute) -> Self {
        let _ = self.intelligence.insert(intelligence);

        self
    }

    pub fn build(self) -> Result<Attributes, AttributesBuildError> {
        self.try_into()
    }
}

#[derive(Clone, Debug)]
pub enum AttributesBuildError {
    FieldMissing(Vec<AttributeName>),
}

impl fmt::Display for AttributesBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Unable to build Attributes: {}",
            match self {
                AttributesBuildError::FieldMissing(fields) => format!(
                    "missing field(s): {}",
                    fields
                        .iter()
                        .map(|s| format!("`{s}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            }
        )
    }
}

impl Error for AttributesBuildError {}

impl TryFrom<FieldAggregator> for AttributesBuildError {
    type Error = ();

    fn try_from(value: FieldAggregator) -> Result<Self, Self::Error> {
        value
            .0
            .map(|fields: Vec<&'static str>| {
                AttributesBuildError::FieldMissing(
                    fields
                        .iter()
                        .map(|field| field.parse().expect("I have no clue how you got here..."))
                        .collect(),
                )
            })
            .ok_or(())
    }
}

impl TryFrom<AttributesBuilder> for Attributes {
    type Error = AttributesBuildError;

    fn try_from(value: AttributesBuilder) -> Result<Self, Self::Error> {
        let mut aggregator = FieldAggregator::new();

        aggregator.field_check(&value.prime, "Prime");
        aggregator.field_check(&value.might, "Might");
        aggregator.field_check(&value.agility, "Agility");
        aggregator.field_check(&value.charisma, "Charisma");
        aggregator.field_check(&value.intelligence, "Intelligence");

        AttributesBuildError::try_from(aggregator).swap()?;

        Ok(Attributes {
            prime: value.prime.unwrap(),
            might: value.might.unwrap(),
            agility: value.agility.unwrap(),
            charisma: value.charisma.unwrap(),
            intelligence: value.intelligence.unwrap(),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Attributes {
    prime: Attribute,
    might: Attribute,
    agility: Attribute,
    charisma: Attribute,
    intelligence: Attribute,
}

impl Attributes {}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Attribute {
    pub base_score: isize,
    pub save_proficiency: bool,
    pub skills: Vec<Skill>,
}

impl Attribute {
    pub fn new() -> Self {
        Attribute::default()
    }

    pub fn with_base_score(mut self, score: isize) -> Self {
        self.base_score = score;

        self
    }

    pub fn with_save_proficiency(mut self) -> Self {
        self.save_proficiency = true;

        self
    }

    pub fn with_skill(mut self, skill: Skill) -> Self {
        self.skills.push(skill);

        self
    }

    #[must_use]
    pub fn calc_save(&self, level: Level) -> isize {
        if self.save_proficiency {
            self.base_score
                .checked_add_unsigned(level.calc_combat_mastery())
                .unwrap()
        } else {
            self.base_score
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

    pub fn with_mastery(mut self, mastery: Mastery) -> Self {
        self.set_mastery(mastery);

        self
    }

    #[must_use]
    pub fn calc_score(&self, attribute: &Attribute) -> isize {
        if let Some(mastery) = self.mastery {
            mastery as isize + attribute.base_score
        } else {
            attribute.base_score
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _attribute_without_save_mastery_should_have_save_of_attribute_score() {
        let attribute = Attribute {
            base_score: 3,
            save_proficiency: false,
            skills: vec![],
        };

        assert_eq!(attribute.calc_save(Level::default()), attribute.base_score);
    }

    #[test]
    fn _attribute_with_save_mastery_should_add_combat_mastery_to_save() {
        let attribute = Attribute {
            base_score: 3,
            save_proficiency: true,
            skills: vec![],
        };
        let level = Level::default();
        let combat_mastery = level.calc_combat_mastery() as isize;

        assert_eq!(
            attribute.calc_save(level),
            attribute.base_score + combat_mastery
        )
    }

    mod skill {
        use super::*;

        #[test]
        fn _skill_without_mastery_should_have_same_score_as_attribute() {
            let skill = Skill::new("Test Skill");
            let attribute = Attribute {
                base_score: 3,
                save_proficiency: true,
                skills: vec![skill.clone()],
            };

            assert_eq!(skill.calc_score(&attribute), attribute.base_score);
        }

        #[test]
        fn _skill_with_mastery_should_add_mastery_value_to_score() {
            let mut skill = Skill::new("Test Skill");
            skill.set_mastery(Mastery::Novice);

            let attribute = Attribute {
                base_score: 3,
                save_proficiency: true,
                skills: vec![skill.clone()],
            };

            assert_eq!(skill.calc_score(&attribute), attribute.base_score + 2);
        }
    }
}
