use serde::{Deserialize, Serialize};
use super::coordinates::Coordinates;
use super::entity::EntityType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NPC {
    pub type_: EntityType,
    pub name: String,
    pub position_x: f32,
    pub position_y: f32,
    pub is_homeless: bool,
    pub home: Coordinates,
    pub variation_index: i32,
}

impl NPC {
    pub fn new(
        type_: EntityType,
        name: String,
        position_x: f32,
        position_y: f32,
        is_homeless: bool,
        home: Coordinates,
        variation_index: i32,
    ) -> Self {
        Self {
            type_,
            name,
            position_x,
            position_y,
            is_homeless,
            home,
            variation_index,
        }
    }
}