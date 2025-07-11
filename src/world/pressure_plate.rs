use serde::{Deserialize, Serialize};
use super::coordinates::Coordinates;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeighedPressurePlate {
    pub position: Coordinates,
}

impl WeighedPressurePlate {
    pub fn new(position: Coordinates) -> Self {
        Self { position }
    }
} 