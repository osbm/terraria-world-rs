use serde::{Deserialize, Serialize};
use super::coordinates::Coordinates;
use super::entity::EntityType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NPC {
    pub type_: EntityType,
    pub name: String,
    pub position: Coordinates,
    pub is_homeless: bool,
    pub home: Coordinates,
    pub variation_index: i32,
}

impl NPC {
    pub fn new(
        type_: EntityType,
        name: String,
        position: Coordinates,
        is_homeless: bool,
        home: Coordinates,
        variation_index: i32,
    ) -> Self {
        Self {
            type_,
            name,
            position,
            is_homeless,
            home,
            variation_index,
        }
    }
}