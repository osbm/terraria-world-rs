use super::coordinates::Coordinates;
use super::item::ItemStack;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chest {
    pub position: Coordinates,
    pub name: String,
    pub contents: Vec<Option<ItemStack>>,
}
