use super::coordinates::Coordinates;
use super::entity::EntityType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub npc: EntityType,
    pub position: Coordinates,
}

impl Room {
    pub fn new(npc: EntityType, position: Coordinates) -> Self {
        Self { npc, position }
    }
}
