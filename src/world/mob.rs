use serde::{Deserialize, Serialize};

use super::entity::EntityType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mob {
    pub type_: EntityType,
    pub position_x: f32,
    pub position_y: f32,
}

impl Mob {
    pub fn new(type_: EntityType, position_x: f32, position_y: f32) -> Self {
        Self {
            type_,
            position_x,
            position_y,
        }
    }
}
