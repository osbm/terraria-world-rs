use serde::{Deserialize, Serialize};
use super::coordinates::Coordinates;
use super::entity::EntityType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mob {
    pub type_: EntityType,
    pub position: Coordinates,
}

impl Mob {
    pub fn new(type_: EntityType, position: Coordinates) -> Self {
        Self { type_, position }
    }
} 