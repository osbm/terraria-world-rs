use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemStack {
    pub quantity: i16,
    pub type_id: i32,
    pub prefix: u8,
} 