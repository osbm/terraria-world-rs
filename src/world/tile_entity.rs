use serde::{Deserialize, Serialize};
use super::coordinates::Coordinates;
use super::item::ItemStack;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileEntityExtra {
    TargetDummy {
        npc: i16,
    },
    ItemFrame {
        item: ItemStack,
    },
    LogicSensor {
        logic_check: u8,
        enabled: bool,
    },
    Mannequin {
        items: Vec<Option<ItemStack>>,
        dyes: Vec<Option<ItemStack>>,
    },
    WeaponRack {
        item: ItemStack,
    },
    HatRack {
        items: Vec<Option<ItemStack>>,
        dyes: Vec<Option<ItemStack>>,
    },
    Plate {
        item: ItemStack,
    },
    Pylon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileEntity {
    pub id: i32,
    pub position: Coordinates,
    pub extra: Option<TileEntityExtra>,
}

impl TileEntity {
    pub fn new(id: i32, position: Coordinates, extra: Option<TileEntityExtra>) -> Self {
        Self {
            id,
            position,
            extra,
        }
    }
} 