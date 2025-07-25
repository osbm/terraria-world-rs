use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LiquidType {
    NoLiquid = 0,
    Water = 1,
    Lava = 2,
    Honey = 3,
    Shimmer = 4,
}

impl From<u8> for LiquidType {
    fn from(value: u8) -> Self {
        match value {
            0 => LiquidType::NoLiquid,
            1 => LiquidType::Water,
            2 => LiquidType::Lava,
            3 => LiquidType::Honey,
            4 => LiquidType::Shimmer,
            _ => LiquidType::NoLiquid,
        }
    }
}

impl std::fmt::Display for LiquidType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiquidType::NoLiquid => write!(f, "NoLiquid"),
            LiquidType::Water => write!(f, "Water"),
            LiquidType::Lava => write!(f, "Lava"),
            LiquidType::Honey => write!(f, "Honey"),
            LiquidType::Shimmer => write!(f, "Shimmer"),
        }
    }
}

pub static BLOCK_TYPE_NAMES: Lazy<HashMap<u16, &'static str>> = Lazy::new(|| {
    let raw_csv = include_str!("../../data/blocks.csv");
    let mut map = HashMap::new();
    map.insert(u16::MAX, "EMPTY");
    for line in raw_csv.lines().skip(1) {
        let mut parts = line.split(',');
        if let (Some(id), Some(name)) = (parts.next(), parts.next()) {
            if let Ok(id) = id.parse::<u16>() {
                map.insert(id, name);
            }
        }
    }
    map
});

pub static WALL_TYPE_NAMES: Lazy<HashMap<u16, &'static str>> = Lazy::new(|| {
    let raw_csv = include_str!("../../data/walls.csv");
    let mut map = HashMap::new();
    for line in raw_csv.lines().skip(1) {
        let mut parts = line.split(',');
        if let (Some(id), Some(name)) = (parts.next(), parts.next()) {
            if let Ok(id) = id.parse::<u16>() {
                map.insert(id, name);
            }
        }
    }
    map
});
