use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fluency {
    Limited,
    Fluent,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Language {
    pub uuid: Uuid,
    pub name: String,
    pub fluency: Fluency,
}

impl Language {
    pub fn new(name: impl Into<String>) -> Self {
        Language {
            uuid: Uuid::new_v4(),
            name: name.into(),
            fluency: Fluency::Limited,
        }
    }

    pub fn set_fluency(&mut self, fluency: Fluency) {
        self.fluency = fluency;
    }
}
