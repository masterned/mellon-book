use core::fmt;
use std::error::Error;

use uuid::Uuid;

use crate::utils::SwapResult;

use super::{FieldAggregator, Language, Skill};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Builder {
    pub uuid: Uuid,
    pub name: Option<String>,
    pub skills: Option<Vec<Skill>>,
    pub trades: Option<Vec<Skill>>,
    pub languages: Option<Vec<Language>>,
}

impl Builder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: Some(name.into()),
            ..Self::default()
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        let name = name.into();

        let _ = self.name.insert(name);

        self
    }

    pub fn add_skill(mut self, skill: Skill) -> Self {
        let _ = self.skills.get_or_insert_default().push(skill);

        self
    }

    pub fn add_trade(mut self, trade: Skill) -> Self {
        let _ = self.trades.get_or_insert_default().push(trade);

        self
    }

    pub fn add_language(mut self, language: Language) -> Self {
        let _ = self.languages.get_or_insert_default().push(language);

        self
    }

    pub fn build(self) -> Result<Background, BuildError> {
        self.try_into()
    }
}

#[derive(Debug, PartialEq)]
pub enum BuildError {
    FieldMissing(Vec<&'static str>),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BuildError::FieldMissing(vec) => vec.join(", "),
            }
        )
    }
}

impl TryFrom<FieldAggregator> for BuildError {
    type Error = ();

    fn try_from(value: FieldAggregator) -> Result<Self, Self::Error> {
        value.0.map(BuildError::FieldMissing).ok_or(())
    }
}

impl Error for BuildError {}

impl TryFrom<Builder> for Background {
    type Error = BuildError;

    fn try_from(value: Builder) -> Result<Background, Self::Error> {
        let mut aggregator = FieldAggregator::new();

        aggregator.field_check(&value.name, "Name");
        aggregator.field_check(&value.skills, "Skills");
        aggregator.field_check(&value.trades, "Trades");
        aggregator.field_check(&value.languages, "Languages");

        BuildError::try_from(aggregator).swap()?;

        Ok(Background {
            uuid: value.uuid,
            name: value.name.unwrap(),
            skills: value.skills.unwrap(),
            trades: value.trades.unwrap(),
            languages: value.languages.unwrap(),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Background {
    pub uuid: Uuid,
    pub name: String,
    pub skills: Vec<Skill>,
    pub trades: Vec<Skill>,
    pub languages: Vec<Language>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _require_name_and_at_least_one_skill_trade_and_language_to_build_background(
    ) -> Result<(), Box<dyn Error>> {
        let builder = Builder::new("Soldier");
        assert_eq!(
            builder.clone().build(),
            Err(BuildError::FieldMissing(vec![
                "Skills",
                "Trades",
                "Languages"
            ]))
        );

        let athletics = Skill::new("Athletics");
        let builder = builder.add_skill(athletics.clone());
        assert_eq!(
            builder.clone().build(),
            Err(BuildError::FieldMissing(vec!["Trades", "Languages"]))
        );

        let blacksmithing = Skill::new("Blacksmithing");
        let builder = builder.add_trade(blacksmithing.clone());
        assert_eq!(
            builder.clone().build(),
            Err(BuildError::FieldMissing(vec!["Languages"]))
        );

        let common = Language::new("Common");
        let builder = builder.add_language(common.clone());
        assert_eq!(
            builder.clone().build(),
            Ok(Background {
                uuid: builder.uuid,
                name: "Soldier".into(),
                skills: vec![athletics],
                trades: vec![blacksmithing],
                languages: vec![common]
            })
        );

        Ok(())
    }
}
