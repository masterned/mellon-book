use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Ancestry {
    pub uuid: Uuid,
    pub name: String,
}

impl Ancestry {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.into(),
        }
    }
}
