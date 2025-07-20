use super::coordinates::Coordinates;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NPC {
    pub type_: i32,
    pub name: String,
    pub position_x: f32,
    pub position_y: f32,
    pub is_homeless: bool,
    pub home: Coordinates,
    pub variation_index: i32,
}

impl NPC {
    pub fn new(
        type_: i32,
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
