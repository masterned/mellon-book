use turann::Builder;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ManeuverKind {
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
    pub kind: ManeuverKind,
    pub action_point_cost: u64,
    pub stamina_point_cost: u64,
    pub description: String,
}
