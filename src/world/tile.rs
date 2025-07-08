use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

// Static lookup tables for efficient block/wall type resolution
static BLOCK_TYPE_NAMES: Lazy<HashMap<u16, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    // Add the most common block types for fast lookup
    map.insert(0, "DIRT");
    map.insert(1, "STONE");
    map.insert(2, "GRASS");
    map.insert(3, "PLANTS");
    map.insert(4, "TORCHES");
    map.insert(5, "TREES");
    map.insert(6, "IRON");
    map.insert(7, "COPPER");
    map.insert(8, "GOLD");
    map.insert(9, "SILVER");
    map.insert(10, "CLOSED_DOOR");
    map.insert(11, "OPEN_DOOR");
    map.insert(12, "HEART");
    map.insert(13, "BOTTLES");
    map.insert(14, "TABLES");
    map.insert(15, "CHAIRS");
    map.insert(16, "ANVILS");
    map.insert(17, "FURNACES");
    map.insert(18, "WORK_BENCHES");
    map.insert(19, "PLATFORMS");
    map.insert(20, "SAPLINGS");
    map.insert(21, "CONTAINERS");
    map.insert(22, "DEMONITE");
    map.insert(23, "CORRUPT_GRASS");
    map.insert(24, "CORRUPT_PLANTS");
    map.insert(25, "EBONSTONE");
    map.insert(26, "DEMON_ALTAR");
    map.insert(27, "SUNFLOWER");
    map.insert(28, "POTS");
    map.insert(29, "PIGGY_BANK");
    map.insert(30, "WOOD_BLOCK");
    map.insert(31, "SHADOW_ORBS");
    map.insert(32, "CORRUPT_THORNS");
    map.insert(33, "CANDLES");
    map.insert(34, "CHANDELIERS");
    map.insert(35, "JACKOLANTERNS");
    map.insert(36, "PRESENTS");
    map.insert(37, "METEORITE");
    map.insert(38, "GRAY_BRICK");
    map.insert(39, "RED_BRICK");
    map.insert(40, "CLAY_BLOCK");
    map.insert(41, "BLUE_DUNGEON_BRICK");
    map.insert(42, "HANGING_LANTERNS");
    map.insert(43, "GREEN_DUNGEON_BRICK");
    map.insert(44, "PINK_DUNGEON_BRICK");
    map.insert(45, "GOLD_BRICK");
    map.insert(46, "SILVER_BRICK");
    map.insert(47, "COPPER_BRICK");
    map.insert(48, "SPIKES");
    map.insert(49, "WATER_CANDLE");
    map.insert(50, "BOOKS");
    map.insert(51, "COBWEB");
    map.insert(52, "VINES");
    map.insert(53, "SAND");
    map.insert(54, "GLASS");
    map.insert(55, "SIGNS");
    map.insert(56, "OBSIDIAN");
    map.insert(57, "ASH");
    map.insert(58, "HELLSTONE");
    map.insert(59, "MUD");
    map.insert(60, "JUNGLE_GRASS");
    map.insert(61, "JUNGLE_PLANTS");
    map.insert(62, "JUNGLE_VINES");
    map.insert(63, "SAPPHIRE");
    map.insert(64, "RUBY");
    map.insert(65, "EMERALD");
    map.insert(66, "TOPAZ");
    map.insert(67, "AMETHYST");
    map.insert(68, "DIAMOND");
    map.insert(69, "JUNGLE_THORNS");
    map.insert(70, "MUSHROOM_GRASS");
    map.insert(71, "MUSHROOM_PLANTS");
    map.insert(72, "MUSHROOM_TREES");
    map.insert(73, "PLANTS2");
    map.insert(74, "JUNGLE_PLANTS2");
    map.insert(75, "OBSIDIAN_BRICK");
    map.insert(76, "HELLSTONE_BRICK");
    map.insert(77, "HELLFORGE");
    map.insert(78, "CLAY_POT");
    map.insert(79, "BEDS");
    map.insert(80, "CACTUS");
    map.insert(81, "CORAL");
    map.insert(82, "IMMATURE_HERBS");
    map.insert(83, "MATURE_HERBS");
    map.insert(84, "BLOOMING_HERBS");
    map.insert(85, "TOMBSTONES");
    map.insert(86, "LOOM");
    map.insert(87, "PIANOS");
    map.insert(88, "DRESSERS");
    map.insert(89, "BENCHES");
    map.insert(90, "BATHTUBS");
    map.insert(91, "BANNERS");
    map.insert(92, "LAMPPOSTS");
    map.insert(93, "LAMPS");
    map.insert(94, "KEGS");
    map.insert(95, "CHINESE_LANTERNS");
    map.insert(96, "COOKING_POTS");
    map.insert(97, "SAFES");
    map.insert(98, "SKULL_LANTERNS");
    map.insert(99, "TRASH_CAN");
    map.insert(100, "CANDELABRAS");
    // Add more as needed - this is just a sample
    map
});

static WALL_TYPE_NAMES: Lazy<HashMap<u16, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    // Add the most common wall types for fast lookup
    map.insert(0, "NONE");
    map.insert(1, "STONE");
    map.insert(2, "DIRT_UNSAFE");
    map.insert(3, "EBONSTONE_UNSAFE");
    map.insert(4, "WOOD");
    map.insert(5, "GRAY_BRICK");
    map.insert(6, "RED_BRICK");
    map.insert(7, "BLUE_DUNGEON_UNSAFE");
    map.insert(8, "GREEN_DUNGEON_UNSAFE");
    map.insert(9, "PINK_DUNGEON_UNSAFE");
    map.insert(10, "GOLD_BRICK");
    map.insert(11, "SILVER_BRICK");
    map.insert(12, "COPPER_BRICK");
    map.insert(13, "HELLSTONE_BRICK_UNSAFE");
    map.insert(14, "OBSIDIAN_BRICK_UNSAFE");
    map.insert(15, "MUD_UNSAFE");
    map.insert(16, "DIRT");
    map.insert(17, "BLUE_DUNGEON");
    map.insert(18, "GREEN_DUNGEON");
    map.insert(19, "PINK_DUNGEON");
    map.insert(20, "OBSIDIAN_BRICK");
    map.insert(21, "GLASS");
    map.insert(22, "PEARLSTONE_BRICK");
    map.insert(23, "IRIDESCENT_BRICK");
    map.insert(24, "MUDSTONE_BRICK");
    map.insert(25, "COBALT_BRICK");
    map.insert(26, "MYTHRIL_BRICK");
    map.insert(27, "PLANKED");
    map.insert(28, "PEARLSTONE_BRICK_UNSAFE");
    map.insert(29, "CANDY_CANE");
    map.insert(30, "GREEN_CANDY_CANE");
    map.insert(31, "SNOW_BRICK");
    map.insert(32, "ADAMANTITE_BEAM");
    map.insert(33, "DEMONITE_BRICK");
    map.insert(34, "SANDSTONE_BRICK");
    map.insert(35, "EBONSTONE_BRICK");
    map.insert(36, "RED_STUCCO");
    map.insert(37, "YELLOW_STUCCO");
    map.insert(38, "GREEN_STUCCO");
    map.insert(39, "GRAY");
    map.insert(40, "SNOW_WALL_UNSAFE");
    map.insert(41, "EBONWOOD");
    map.insert(42, "RICH_MAOGANY");
    map.insert(43, "PEARLWOOD");
    map.insert(44, "RAINBOW_BRICK");
    map.insert(45, "TIN_BRICK");
    map.insert(46, "TUNGSTEN_BRICK");
    map.insert(47, "PLATINUM_BRICK");
    map.insert(48, "AMETHYST_UNSAFE");
    map.insert(49, "TOPAZ_UNSAFE");
    map.insert(50, "SAPPHIRE_UNSAFE");
    map.insert(51, "EMERALD_UNSAFE");
    map.insert(52, "RUBY_UNSAFE");
    map.insert(53, "DIAMOND_UNSAFE");
    map.insert(54, "CAVE_UNSAFE");
    map.insert(55, "CAVE2UNSAFE");
    map.insert(56, "CAVE3UNSAFE");
    map.insert(57, "CAVE4UNSAFE");
    map.insert(58, "CAVE5UNSAFE");
    map.insert(59, "CAVE6UNSAFE");
    map.insert(60, "LIVING_LEAF");
    map.insert(61, "CAVE7UNSAFE");
    map.insert(62, "SPIDER_UNSAFE");
    map.insert(63, "GRASS_UNSAFE");
    map.insert(64, "JUNGLE_UNSAFE");
    map.insert(65, "FLOWER_UNSAFE");
    map.insert(66, "GRASS");
    map.insert(67, "JUNGLE");
    map.insert(68, "FLOWER");
    map.insert(69, "CORRUPT_GRASS_UNSAFE");
    map.insert(70, "HALLOWED_GRASS_UNSAFE");
    map.insert(71, "ICE_UNSAFE");
    map.insert(72, "CACTUS");
    map.insert(73, "CLOUD");
    map.insert(74, "MUSHROOM");
    map.insert(75, "BONE");
    map.insert(76, "SLIME");
    map.insert(77, "FLESH");
    map.insert(78, "LIVING_WOOD");
    map.insert(79, "OBSIDIAN_BACK_UNSAFE");
    map.insert(80, "MUSHROOM_UNSAFE");
    map.insert(81, "CRIMSON_GRASS_UNSAFE");
    map.insert(82, "DISC_WALL");
    map.insert(83, "CRIMSTONE_UNSAFE");
    map.insert(84, "ICE_BRICK");
    map.insert(85, "SHADEWOOD");
    map.insert(86, "HIVE_UNSAFE");
    map.insert(87, "LIHZAHRD_BRICK_UNSAFE");
    map.insert(88, "PURPLE_STAINED_GLASS");
    map.insert(89, "YELLOW_STAINED_GLASS");
    map.insert(90, "BLUE_STAINED_GLASS");
    map.insert(91, "GREEN_STAINED_GLASS");
    map.insert(92, "RED_STAINED_GLASS");
    map.insert(93, "RAINBOW_STAINED_GLASS");
    map.insert(94, "BLUE_DUNGEON_SLAB_UNSAFE");
    map.insert(95, "BLUE_DUNGEON_TILE_UNSAFE");
    map.insert(96, "PINK_DUNGEON_SLAB_UNSAFE");
    map.insert(97, "PINK_DUNGEON_TILE_UNSAFE");
    map.insert(98, "GREEN_DUNGEON_SLAB_UNSAFE");
    map.insert(99, "GREEN_DUNGEON_TILE_UNSAFE");
    map.insert(100, "BLUE_DUNGEON_SLAB");
    // Add more as needed - this is just a sample
    map
});

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockType(u16);

impl BlockType {
    pub fn new(id: u16) -> Self {
        Self(id)
    }

    pub fn id(&self) -> u16 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        BLOCK_TYPE_NAMES.get(&self.0).unwrap_or(&"UNKNOWN")
    }
}

impl From<u16> for BlockType {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl fmt::Display for BlockType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WallType(u16);

impl WallType {
    pub fn new(id: u16) -> Self {
        Self(id)
    }

    pub fn id(&self) -> u16 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        WALL_TYPE_NAMES.get(&self.0).unwrap_or(&"UNKNOWN")
    }
}

impl From<u16> for WallType {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl fmt::Display for WallType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Liquid {
    pub type_: LiquidType,
    pub volume: u8,
}

impl Liquid {
    pub fn new(type_: LiquidType, volume: u8) -> Self {
        Self { type_, volume }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    pub block: Option<Block>,
    pub wall: Option<Wall>,
    pub liquid: Option<Liquid>,
    pub wiring: Wiring,
}

impl Tile {
    pub fn new(
        block: Option<Block>,
        wall: Option<Wall>,
        liquid: Option<Liquid>,
        wiring: Wiring,
    ) -> Self {
        Self {
            block,
            wall,
            liquid,
            wiring,
        }
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
