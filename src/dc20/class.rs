use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    pub uuid: Uuid,
    pub name: String,
    subclass: Option<Subclass>,
}

impl Class {
    pub fn new(name: impl Into<String>) -> Self {
        Class {
            uuid: Uuid::new_v4(),
            name: name.into(),
            subclass: None,
        }
    }

    #[must_use]
    pub fn add_subclass(mut self, subclass: Subclass) -> Self {
        let _ = self.subclass.insert(subclass);

        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Subclass {
    pub uuid: Uuid,
    pub name: String,
}

impl Subclass {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.into(),
        }
    }
}
