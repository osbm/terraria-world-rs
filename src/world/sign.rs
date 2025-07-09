use serde::{Deserialize, Serialize};
use super::coordinates::Coordinates;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sign {
    pub text: String,
    pub position: Coordinates,
} 