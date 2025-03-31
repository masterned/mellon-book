use uuid::Uuid;

use super::Points;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ManeuverType {
    Attack,
    Save,
    Grapple,
    Defense,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Maneuver {
    pub uuid: Uuid,
    pub name: String,
    pub cost: Points,
    pub description: String,
}
