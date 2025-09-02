use turann::Builder;
use uuid::Uuid;

use super::{LanguageFluency, Skill};

#[derive(Builder, Clone, Debug, PartialEq)]
#[builder(validate = Self::has_skills_and_trades_languages)]
pub struct Background {
    #[builder(default = Uuid::new_v4)]
    pub uuid: Uuid,
    #[builder(validate = Self::validate_name)]
    pub name: String,
    #[builder(each = "skill")]
    pub skills: Vec<Skill>,
    #[builder(each = "trade")]
    pub trades: Vec<Skill>,
    #[builder(each = "language_fluency")]
    pub language_fluencies: Vec<LanguageFluency>,
}

impl BackgroundBuilder {
    fn has_skills_and_trades_languages(
        background: Background,
    ) -> Result<Background, BackgroundBuilderError> {
        let mut msg: Option<Vec<&str>> = None;

        if background.skills.is_empty() {
            msg.get_or_insert_default().push("Skills");
        }
        if background.trades.is_empty() {
            msg.get_or_insert_default().push("Trades");
        }
        if background.language_fluencies.is_empty() {
            msg.get_or_insert_default().push("Language Fluencies");
        }

        if let Some(msg) = msg {
            Err(BackgroundBuilderError::missing_fields(&msg))
        } else {
            Ok(background)
        }
    }

    fn validate_name(name: String) -> Result<String, BackgroundBuilderError> {
        if name.is_empty() {
            return Err(BackgroundBuilderError::InvalidField {
                field_name: "name".into(),
                message: "cannot be empty".into(),
            });
        }

        Ok(name)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn _require_name_and_at_least_one_skill_trade_and_language_to_build_background(
    ) -> Result<(), Box<dyn Error>> {
        let mut builder = Background::builder();

        builder.name("Soldier")?;

        assert_eq!(
            builder.clone().build(),
            Err(BackgroundBuilderError::missing_fields(&[
                "Skills",
                "Trades",
                "Language Fluencies"
            ]))
        );

        let athletics = Skill::new("Athletics");
        let builder = builder.skill(athletics.clone());
        assert_eq!(
            builder.clone().build(),
            Err(BackgroundBuilderError::missing_fields(&[
                "Trades",
                "Language Fluencies"
            ]))
        );

        let blacksmithing = Skill::new("Blacksmithing");
        let builder = builder.trade(blacksmithing.clone());
        assert_eq!(
            builder.clone().build(),
            Err(BackgroundBuilderError::missing_fields(&[
                "Language Fluencies"
            ]))
        );

        let common = LanguageFluency::common();
        let builder = builder.language_fluency(common.clone());
        let result = builder.build();
        assert_eq!(
            result.clone(),
            Ok(Background {
                uuid: result.unwrap().uuid,
                name: "Soldier".into(),
                skills: vec![athletics],
                trades: vec![blacksmithing],
                language_fluencies: vec![common]
            })
        );

        Ok(())
    }
}
