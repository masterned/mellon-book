use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Background {
    pub uuid: Uuid,
    pub name: String,
}

impl Background {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.into(),
        }
    }
}
