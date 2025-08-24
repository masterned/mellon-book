use turann::Builder;
use uuid::Uuid;

impl PlayerBuilder {
    fn validate_name(name: String) -> Result<String, PlayerBuilderError> {
        if name.is_empty() {
            return Err(PlayerBuilderError::InvalidField {
                field_name: "name".into(),
                message: "cannot be empty".into(),
            });
        }

        Ok(name)
    }
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Player {
    #[builder(default = Uuid::new_v4)]
    uuid: Uuid,
    #[builder(validate = Self::validate_name)]
    name: String,
}

impl Player {
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
