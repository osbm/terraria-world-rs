use crate::world::enums::LiquidType;
use crate::world::enums::{BLOCK_TYPE_NAMES, WALL_TYPE_NAMES};
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
    pub block_id: u16,
    pub block_frame: Option<FrameImportantData>,
    pub block_paint: Option<u8>,
    pub block_active: bool,
    pub block_shape: u8,
    pub block_illuminant: bool,
    pub block_echo: bool,

    // Wall attributes
    pub wall_id: u16,
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
            block_id: u16::MAX,
            block_frame: None,
            block_paint: None,
            block_active: true,
            block_shape: 0,
            block_illuminant: false,
            block_echo: false,
            wall_id: u16::MAX,
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

    pub fn get_block_name(&self) -> &'static str {
        if let Some(name) = BLOCK_TYPE_NAMES.get(&self.block_id) {
            name
        } else {
            "Unknown Block"
        }
    }

    pub fn set_block_name(&mut self, name: &str) {
        if !BLOCK_TYPE_NAMES.values().any(|&n| n == name) {
            panic!("Block name '{name}' not found in BLOCK_TYPE_NAMES");
        }
        self.block_id = BLOCK_TYPE_NAMES
            .iter()
            .find(|&(_, &v)| v == name)
            .map(|(&k, _)| k)
            .unwrap_or(u16::MAX);
    }

    pub fn get_wall_name(&self) -> &'static str {
        if let Some(name) = WALL_TYPE_NAMES.get(&self.wall_id) {
            name
        } else {
            "Unknown Wall"
        }
    }

    pub fn set_wall_name(&mut self, name: &str) {
        if !WALL_TYPE_NAMES.values().any(|&n| n == name) {
            panic!("Wall name '{name}' not found in WALL_TYPE_NAMES");
        }
        self.wall_id = WALL_TYPE_NAMES
            .iter()
            .find(|&(_, &v)| v == name)
            .map(|(&k, _)| k)
            .unwrap_or(u16::MAX);
    }

    pub fn has_block(&self) -> bool {
        self.block_id != u16::MAX
    }

    pub fn has_wall(&self) -> bool {
        self.wall_id != u16::MAX
    }

    pub fn has_liquid(&self) -> bool {
        self.liquid_type != LiquidType::NoLiquid && self.liquid_amount > 0
    }

    pub fn tiles_equal(&self, other: &Tile) -> bool {
        let block_equal = self.block_id == other.block_id
            && self.block_active == other.block_active
            && self.block_shape == other.block_shape
            && self.block_paint == other.block_paint
            && self.block_illuminant == other.block_illuminant
            && self.block_echo == other.block_echo
            && self.block_frame == other.block_frame;
        let wall_equal = self.wall_id == other.wall_id
            && self.wall_paint == other.wall_paint
            && self.wall_illuminant == other.wall_illuminant
            && self.wall_echo == other.wall_echo;
        let liquid_equal =
            self.liquid_type == other.liquid_type && self.liquid_amount == other.liquid_amount;
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
        let block = if self.has_block() {
            let paint = self
                .block_paint
                .map(|p| format!("[{p}]"))
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
                self.get_block_name(),
                paint,
                frame,
                active,
                illum,
                echo
            )
        } else {
            "·".to_string()
        };

        // Wall representation
        let wall = if self.has_wall() {
            let paint = self
                .wall_paint
                .map(|p| format!("[{p}]"))
                .unwrap_or_default();
            let illum = if self.wall_illuminant { "✨" } else { "" };
            let echo = if self.wall_echo { "🔊" } else { "" };
            format!("|{}{}{}{}|", self.get_wall_name(), paint, illum, echo)
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

        write!(f, "{block} {wall} {liquid} {wires}")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileMatrix {
    pub tiles: Vec<Vec<Tile>>,
    pub size: (usize, usize), // (width, height)
}

impl TileMatrix {
    pub fn new(size: (usize, usize)) -> Self {
        Self {
            tiles: vec![vec![Tile::new(); size.1]; size.0],
            size,
        }
    }

    pub fn add_column(&mut self, column: Vec<Tile>) {
        self.tiles.push(column);
        self.size.0 = self.tiles.len();
        if !self.tiles.is_empty() {
            self.size.1 = self.tiles[0].len();
        }
    }
}

impl std::fmt::Display for TileMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // just print the matrix size
        write!(f, "TileMatrix ({}x{})", self.size.0, self.size.1)
    }
}