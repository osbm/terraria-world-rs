use super::coordinates::Coordinates;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sign {
    pub text: String,
    pub position: Coordinates,
}
