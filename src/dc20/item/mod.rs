use uuid::Uuid;

pub mod weapon;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Item {
    pub uuid: Uuid,
    pub name: String,
}
