use turann::Builder;
use uuid::Uuid;

use super::Points;

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Technique {
    #[builder(default = Uuid::new_v4)]
    pub uuid: Uuid,
    pub name: String,
    #[builder(each = "cost")]
    pub cost: Vec<Points>,
    pub description: String,
}
