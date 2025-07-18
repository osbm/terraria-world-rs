use crate::world::enums::{BlockType, WallType, LiquidType};
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
        let block_equal = self.block_type == other.block_type &&
                         self.block_active == other.block_active &&
                         self.block_shape == other.block_shape &&
                         self.block_paint == other.block_paint &&
                         self.block_illuminant == other.block_illuminant &&
                         self.block_echo == other.block_echo &&
                         self.block_frame == other.block_frame;

        // Compare walls
        let wall_equal = self.wall_type == other.wall_type &&
                        self.wall_paint == other.wall_paint &&
                        self.wall_illuminant == other.wall_illuminant &&
                        self.wall_echo == other.wall_echo;

        // Compare liquids
        let liquid_equal = self.liquid_type == other.liquid_type &&
                          self.liquid_amount == other.liquid_amount;

        // Compare wiring
        let wiring_equal = self.red_wire == other.red_wire &&
                          self.blue_wire == other.blue_wire &&
                          self.green_wire == other.green_wire &&
                          self.yellow_wire == other.yellow_wire &&
                          self.activator_wire == other.activator_wire;

        block_equal && wall_equal && liquid_equal && wiring_equal
    }

    /// Serialize tile data to bytes for writing to file
    pub fn serialize_tile_data(&self, tile_frame_important: &[bool], version_integer: i32) -> Vec<u8> {
        // Prepare headers
        let mut header1 = 0u8;
        let mut header2 = 0u8;
        let mut header3 = 0u8;
        let mut header4 = 0u8;
        let mut data = Vec::new();

        // Determine if we need additional headers
        let mut has_flags2 = false;
        let mut has_flags3 = false;
        let mut has_flags4 = false;

        // Block handling
        if let Some(block_type) = &self.block_type {
            if self.block_active && block_type.id() <= 520 && block_type.id() != 423 {
                // Set has_block flag (bit 1)
                header1 |= 0b_0000_0010;

                // Check if we need extended block ID
                if block_type.id() > 255 {
                    header1 |= 0b_0010_0000; // has_extended_block_id
                }

                // Write block type
                data.push(block_type.id() as u8); // low byte
                if block_type.id() > 255 {
                    data.push((block_type.id() >> 8) as u8); // high byte
                }

                // Handle frame data
                if let Some(frame) = &self.block_frame {
                    data.push((frame.x & 0xFF) as u8);
                    data.push(((frame.x & 0xFF00) >> 8) as u8);
                    data.push((frame.y & 0xFF) as u8);
                    data.push(((frame.y & 0xFF00) >> 8) as u8);
                } else if (block_type.id() as usize) < tile_frame_important.len() && tile_frame_important[block_type.id() as usize] {
                    data.push(0); data.push(0); data.push(0); data.push(0);
                }

                // Handle paint and illuminant
                if version_integer < 269 {
                    if let Some(paint) = self.block_paint {
                        if paint != 0 || self.block_illuminant {
                            let mut color = paint;
                            if color == 0 && self.block_illuminant { color = 31; }
                            header3 |= 0b_0000_1000; // is_block_painted
                            data.push(color);
                            has_flags3 = true;
                        }
                    }
                } else {
                    if let Some(paint) = self.block_paint {
                        if paint != 0 && paint != 31 {
                            header3 |= 0b_0000_1000; // is_block_painted
                            data.push(paint);
                            has_flags3 = true;
                        }
                    }
                }

                // Handle block active state (inverted in flags3)
                if !self.block_active {
                    header3 |= 0b_0000_0100; // is_block_active (inverted)
                    has_flags3 = true;
                }

                // Handle echo and illuminant for version >= 269
                if version_integer >= 269 {
                    if self.block_echo {
                        header4 |= 0b_0000_0010; // is_block_echo
                        has_flags4 = true;
                    }
                    if self.block_illuminant || self.block_paint == Some(31) {
                        header4 |= 0b_0000_1000; // is_block_illuminant
                        has_flags4 = true;
                    }
                }
            }
        }

        // Wall handling
        if let Some(wall_type) = &self.wall_type {
            if wall_type.id() != 0 {
                header1 |= 0b_0000_0100; // has_wall

                // Write wall type (low byte first)
                data.push(wall_type.id() as u8);

                // Handle paint for walls
                if version_integer < 269 {
                    if let Some(paint) = self.wall_paint {
                        if paint != 0 || self.wall_illuminant {
                            let mut color = paint;
                            if color == 0 && self.wall_illuminant { color = 31; }
                            header3 |= 0b_0001_0000; // is_wall_painted
                            data.push(color);
                            has_flags3 = true;
                        }
                    }
                } else {
                    if let Some(paint) = self.wall_paint {
                        if paint != 0 && paint != 31 {
                            header3 |= 0b_0001_0000; // is_wall_painted
                            data.push(paint);
                            has_flags3 = true;
                        }
                    }
                }

                // Handle extended wall ID
                if wall_type.id() > 255 && version_integer >= 222 {
                    header3 |= 0b_0100_0000; // has_extended_wall_id
                    data.push((wall_type.id() >> 8) as u8); // high byte
                    has_flags3 = true;
                }

                // Handle echo and illuminant for version >= 269
                if version_integer >= 269 {
                    if self.wall_echo {
                        header4 |= 0b_0000_0100; // is_wall_echo
                        has_flags4 = true;
                    }
                    if self.wall_illuminant || self.wall_paint == Some(31) {
                        header4 |= 0b_0001_0000; // is_wall_illuminant
                        has_flags4 = true;
                    }
                }
            }
        }

        // Liquid handling
        if self.liquid_amount != 0 && self.liquid_type != LiquidType::NoLiquid {
            match self.liquid_type {
                LiquidType::Shimmer if version_integer >= 269 => {
                    header3 |= 0b_1000_0000; // shimmer flag
                    header1 |= 0b_0000_1000; // water flag
                    has_flags3 = true;
                }
                LiquidType::Lava => {
                    header1 |= 0b_0001_0000; // lava flag
                }
                LiquidType::Honey => {
                    header1 |= 0b_0001_1000; // honey flag (both water and lava)
                }
                _ => {
                    header1 |= 0b_0000_1000; // water flag
                }
            }
            data.push(self.liquid_amount);
        }

        // Wiring handling
        if self.red_wire {
            header2 |= 0b_0000_0010; // red wire
            has_flags2 = true;
        }
        if self.blue_wire {
            header2 |= 0b_0000_0100; // blue wire
            has_flags2 = true;
        }
        if self.green_wire {
            header2 |= 0b_0000_1000; // green wire
            has_flags2 = true;
        }
        if self.yellow_wire {
            header3 |= 0b_0010_0000; // yellow wire
            has_flags3 = true;
        }
        if self.activator_wire {
            header3 |= 0b_0000_0010; // activator wire
            has_flags3 = true;
        }
        // Block shape (brick style)
        if self.block_type.is_some() {
            let brick_style = (self.block_shape << 4) as u8;
            header2 |= brick_style;
            if brick_style != 0 {
                has_flags2 = true;
            }
        }

        // Set header flags
        if has_flags4 {
            header3 |= 0b_0000_0001; // has_flags4
            has_flags3 = true;
        }
        if has_flags3 {
            header2 |= 0b_0000_0001; // has_flags3
            has_flags2 = true;
        }
        if has_flags2 {
            header1 |= 0b_0000_0001; // has_flags2
        }

        // Build the output
        let mut out = Vec::new();
        out.push(header1);
        if has_flags2 {
            out.push(header2);
            if has_flags3 {
                out.push(header3);
                if has_flags4 {
                    out.push(header4);
                }
            }
        }
        out.extend(data);
        out
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Block representation
        let block = if let Some(block_type) = &self.block_type {
            let paint = self.block_paint.map(|p| format!("[{}]", p)).unwrap_or_default();
            let frame = self.block_frame.as_ref()
                .map(|f| format!("({},{})", f.x, f.y))
                .unwrap_or_default();
            let active = if !self.block_active { "!" } else { "" };
            let illum = if self.block_illuminant { "✨" } else { "" };
            let echo = if self.block_echo { "🔊" } else { "" };
            format!("{}{}{}{}{}{}", block_type, paint, frame, active, illum, echo)
        } else {
            "·".to_string()
        };

        // Wall representation
        let wall = if let Some(wall_type) = &self.wall_type {
            let paint = self.wall_paint.map(|p| format!("[{}]", p)).unwrap_or_default();
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
        ].join("");

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
