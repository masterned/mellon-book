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
    #[test]
    #[ignore = "API changed"]
    fn _require_name_and_at_least_one_skill_trade_and_language_to_build_background(
    ) -> anyhow::Result<()> {
        todo!()
    }
}
