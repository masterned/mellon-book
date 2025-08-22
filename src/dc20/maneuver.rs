use turann::Builder;
use uuid::Uuid;

use super::Points;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ManeuverType {
    Attack,
    Save,
    Grapple,
    Defense,
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Maneuver {
    #[builder(default = Uuid::new_v4)]
    pub uuid: Uuid,
    pub name: String,
    pub cost: Points,
    pub description: String,
}
