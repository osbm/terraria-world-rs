#[derive(Debug)]
pub struct Pointers {
    pub file_format: u32,
    pub world_header: u32,
    pub world_tiles: u32,
    pub chests: u32,
    pub signs: u32,
    pub npcs: u32,
    pub tile_entities: u32,
    pub pressure_plates: u32,
    pub town_manager: u32,
    pub bestiary: u32,
    pub journey_powers: u32,
    pub footer: u32,
    pub unknown: u32,
}

impl Pointers {
    pub fn from_vector(vec: &[u32]) -> Self {
        Self {
            file_format: 0u32,
            world_header: *vec.get(0).unwrap_or(&0),
            world_tiles: *vec.get(1).unwrap_or(&0),
            chests: *vec.get(2).unwrap_or(&0),
            signs: *vec.get(3).unwrap_or(&0),
            npcs: *vec.get(4).unwrap_or(&0),
            tile_entities: *vec.get(5).unwrap_or(&0),
            pressure_plates: *vec.get(6).unwrap_or(&0),
            town_manager: *vec.get(7).unwrap_or(&0),
            bestiary: *vec.get(8).unwrap_or(&0),
            journey_powers: *vec.get(9).unwrap_or(&0),
            footer: *vec.get(10).unwrap_or(&0),
        }
    }
}
