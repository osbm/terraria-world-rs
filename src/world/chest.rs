use serde::{Deserialize, Serialize};
use super::coordinates::Coordinates;
use super::item::ItemStack;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chest {
    pub position: Coordinates,
    pub name: String,
    pub contents: Vec<Option<ItemStack>>,
} 