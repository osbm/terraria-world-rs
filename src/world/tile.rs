use crate::world::enums::{BlockType, LiquidType, WallType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameImportantData {
    pub x: u16,
    pub y: u16,
}

impl FrameImportantData {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    // Block attributes
    pub block_type: Option<BlockType>,
    pub block_frame: Option<FrameImportantData>,
    pub block_paint: Option<u8>,
    pub block_active: bool,
    pub block_shape: u8,
    pub block_illuminant: bool,
    pub block_echo: bool,

    // Wall attributes
    pub wall_type: Option<WallType>,
    pub wall_paint: Option<u8>,
    pub wall_illuminant: bool,
    pub wall_echo: bool,

    // Liquid attributes
    pub liquid_type: LiquidType,
    pub liquid_amount: u8,

    // Wiring attributes
    pub red_wire: bool,
    pub blue_wire: bool,
    pub green_wire: bool,
    pub yellow_wire: bool,
    pub activator_wire: bool,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            block_type: None,
            block_frame: None,
            block_paint: None,
            block_active: true,
            block_shape: 0,
            block_illuminant: false,
            block_echo: false,
            wall_type: None,
            wall_paint: None,
            wall_illuminant: false,
            wall_echo: false,
            liquid_type: LiquidType::NoLiquid,
            liquid_amount: 0,
            red_wire: false,
            blue_wire: false,
            green_wire: false,
            yellow_wire: false,
            activator_wire: false,
        }
    }

    pub fn has_block(&self) -> bool {
        self.block_type.is_some()
    }

    pub fn has_wall(&self) -> bool {
        self.wall_type.is_some()
    }

    pub fn has_liquid(&self) -> bool {
        self.liquid_type != LiquidType::NoLiquid && self.liquid_amount > 0
    }

    /// Check if two tiles are equal for RLE compression
    pub fn tiles_equal(&self, other: &Tile) -> bool {
        // Compare blocks
        let block_equal = self.block_type == other.block_type
            && self.block_active == other.block_active
            && self.block_shape == other.block_shape
            && self.block_paint == other.block_paint
            && self.block_illuminant == other.block_illuminant
            && self.block_echo == other.block_echo
            && self.block_frame == other.block_frame;

        // Compare walls
        let wall_equal = self.wall_type == other.wall_type
            && self.wall_paint == other.wall_paint
            && self.wall_illuminant == other.wall_illuminant
            && self.wall_echo == other.wall_echo;

        // Compare liquids
        let liquid_equal =
            self.liquid_type == other.liquid_type && self.liquid_amount == other.liquid_amount;

        // Compare wiring
        let wiring_equal = self.red_wire == other.red_wire
            && self.blue_wire == other.blue_wire
            && self.green_wire == other.green_wire
            && self.yellow_wire == other.yellow_wire
            && self.activator_wire == other.activator_wire;

        block_equal && wall_equal && liquid_equal && wiring_equal
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Block representation
        let block = if let Some(block_type) = &self.block_type {
            let paint = self
                .block_paint
                .map(|p| format!("[{}]", p))
                .unwrap_or_default();
            let frame = self
                .block_frame
                .as_ref()
                .map(|f| format!("({},{})", f.x, f.y))
                .unwrap_or_default();
            let active = if !self.block_active { "!" } else { "" };
            let illum = if self.block_illuminant { "✨" } else { "" };
            let echo = if self.block_echo { "🔊" } else { "" };
            format!(
                "{}{}{}{}{}{}",
                block_type, paint, frame, active, illum, echo
            )
        } else {
            "·".to_string()
        };

        // Wall representation
        let wall = if let Some(wall_type) = &self.wall_type {
            let paint = self
                .wall_paint
                .map(|p| format!("[{}]", p))
                .unwrap_or_default();
            let illum = if self.wall_illuminant { "✨" } else { "" };
            let echo = if self.wall_echo { "🔊" } else { "" };
            format!("|{}{}{}{}|", wall_type, paint, illum, echo)
        } else {
            " ".to_string()
        };

        // Liquid representation
        let liquid = if self.has_liquid() {
            let symbol = match self.liquid_type {
                LiquidType::Water => "💧",
                LiquidType::Lava => "🔥",
                LiquidType::Honey => "🍯",
                LiquidType::Shimmer => "✨",
                LiquidType::NoLiquid => "",
            };
            format!("{}{}", symbol, self.liquid_amount)
        } else {
            " ".to_string()
        };

        // Wire representation
        let wires = [
            if self.red_wire { "🔴" } else { "" },
            if self.blue_wire { "🔵" } else { "" },
            if self.green_wire { "🟢" } else { "" },
            if self.yellow_wire { "🟡" } else { "" },
            if self.activator_wire { "⚡" } else { "" },
        ]
        .join("");

        write!(f, "{} {} {} {}", block, wall, liquid, wires)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileMatrix {
    pub tiles: Vec<Vec<Tile>>,
    pub size: (usize, usize), // (width, height)
}

impl TileMatrix {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new(),
            size: (0, 0),
        }
    }

    pub fn add_column(&mut self, column: Vec<Tile>) {
        self.tiles.push(column);
        self.size.0 = self.tiles.len();
        if !self.tiles.is_empty() {
            self.size.1 = self.tiles[0].len();
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(x)?.get(y)
    }
}

impl Default for TileMatrix {
    fn default() -> Self {
        Self::new()
    }
}
