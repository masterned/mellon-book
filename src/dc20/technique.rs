use uuid::Uuid;

use super::Points;

#[derive(Clone, Debug, PartialEq)]
pub struct Technique {
    pub uuid: Uuid,
    pub name: String,
    pub cost: Vec<Points>,
    pub description: String,
}
