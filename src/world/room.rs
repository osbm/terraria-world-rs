use super::coordinates::Coordinates;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub npc: i32,
    pub position: Coordinates,
}

impl Room {
    pub fn new(npc: i32, position: Coordinates) -> Self {
        Self { npc, position }
    }
}
