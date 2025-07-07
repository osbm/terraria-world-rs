use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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

impl fmt::Display for LiquidType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiquidType::NoLiquid => write!(f, "NoLiquid"),
            LiquidType::Water => write!(f, "Water"),
            LiquidType::Lava => write!(f, "Lava"),
            LiquidType::Honey => write!(f, "Honey"),
            LiquidType::Shimmer => write!(f, "Shimmer"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    Dirt,
    Stone,
    Grass,
    Plants,
    Torches,
    // Add more block types as needed
    Unknown(u16),
}

impl From<u16> for BlockType {
    fn from(value: u16) -> Self {
        match value {
            0 => BlockType::Dirt,
            1 => BlockType::Stone,
            2 => BlockType::Grass,
            3 => BlockType::Plants,
            4 => BlockType::Torches,
            _ => BlockType::Unknown(value),
        }
    }
}

impl fmt::Display for BlockType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockType::Dirt => write!(f, "Dirt"),
            BlockType::Stone => write!(f, "Stone"),
            BlockType::Grass => write!(f, "Grass"),
            BlockType::Plants => write!(f, "Plants"),
            BlockType::Torches => write!(f, "Torches"),
            BlockType::Unknown(id) => write!(f, "Unknown({})", id),
        }
    }
}

impl BlockType {
    pub fn id(&self) -> u16 {
        match self {
            BlockType::Dirt => 0,
            BlockType::Stone => 1,
            BlockType::Grass => 2,
            BlockType::Plants => 3,
            BlockType::Torches => 4,
            BlockType::Unknown(id) => *id,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WallType {
    None,
    Stone,
    Dirt,
    // Add more wall types as needed
    Unknown(u16),
}

impl From<u16> for WallType {
    fn from(value: u16) -> Self {
        match value {
            0 => WallType::None,
            1 => WallType::Stone,
            2 => WallType::Dirt,
            _ => WallType::Unknown(value),
        }
    }
}

impl fmt::Display for WallType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WallType::None => write!(f, "None"),
            WallType::Stone => write!(f, "Stone"),
            WallType::Dirt => write!(f, "Dirt"),
            WallType::Unknown(id) => write!(f, "Unknown({})", id),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RLEEncoding {
    NoCompression = 0,
    SingleByte = 1,
    DoubleByte = 2,
}

impl From<u8> for RLEEncoding {
    fn from(value: u8) -> Self {
        match value {
            0 => RLEEncoding::NoCompression,
            1 => RLEEncoding::SingleByte,
            2 => RLEEncoding::DoubleByte,
            _ => RLEEncoding::NoCompression,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrameImportantData {
    pub x: u16,
    pub y: u16,
}

impl FrameImportantData {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub type_: BlockType,
    pub frame: Option<FrameImportantData>,
    pub paint: Option<u8>,
    pub is_active: bool,
    pub shape: u8, // TODO: Implement proper Shape enum
    pub is_illuminant: bool,
    pub is_echo: bool,
}

impl Block {
    pub fn new(
        type_: BlockType,
        frame: Option<FrameImportantData>,
        paint: Option<u8>,
        is_active: bool,
        shape: u8,
        is_illuminant: bool,
        is_echo: bool,
    ) -> Self {
        Self {
            type_,
            frame,
            paint,
            is_active,
            shape,
            is_illuminant,
            is_echo,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wall {
    pub type_: WallType,
    pub paint: Option<u8>,
    pub is_illuminant: bool,
    pub is_echo: bool,
}

impl Wall {
    pub fn new(type_: WallType, paint: Option<u8>, is_illuminant: bool, is_echo: bool) -> Self {
        Self {
            type_,
            paint,
            is_illuminant,
            is_echo,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Liquid {
    pub type_: LiquidType,
    pub volume: u8,
}

impl Liquid {
    pub fn new(type_: LiquidType, volume: u8) -> Self {
        Self { type_, volume }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wiring {
    pub red: bool,
    pub blue: bool,
    pub green: bool,
    pub yellow: bool,
}

impl Wiring {
    pub fn new(red: bool, blue: bool, green: bool, yellow: bool) -> Self {
        Self {
            red,
            blue,
            green,
            yellow,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    pub block: Option<Block>,
    pub wall: Option<Wall>,
    pub liquid: Option<Liquid>,
    pub wiring: Wiring,
}

impl Tile {
    pub fn new(block: Option<Block>, wall: Option<Wall>, liquid: Option<Liquid>, wiring: Wiring) -> Self {
        Self {
            block,
            wall,
            liquid,
            wiring,
        }
    }
}

#[derive(Debug)]
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