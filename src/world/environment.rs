use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldEnvironment {
    pub moon_style: u8,
    pub tree_style_separators: Vec<i32>,
    pub tree_style_properties: Vec<i32>,
    pub moss_style_separators: Vec<i32>,
    pub moss_style_properties: Vec<i32>,
    pub snow_background_style: i32,
    pub jungle_background_style: i32,
    pub hell_background_style: i32,
    pub forest_background: i8,
    pub corruption_background: i8,
    pub jungle_background: i8,
    pub snow_background: i8,
    pub hallow_background: i8,
    pub crimson_background: i8,
    pub desert_background: i8,
    pub ocean_background: i8,
    pub mushroom_background: i8,
    pub underworld_background: i8,
    pub forest_background_2: i8,
    pub forest_background_3: i8,
    pub forest_background_4: i8,
    pub cloud_background: i32,
    pub cloud_number: i16,
    pub wind_speed: f32,
    pub treetop_variants: Vec<i32>,
}

impl Default for WorldEnvironment {
    fn default() -> Self {
        Self {
            moon_style: 0,
            tree_style_separators: vec![0; 3],
            tree_style_properties: vec![0; 4],
            moss_style_separators: vec![0; 3],
            moss_style_properties: vec![0; 4],
            snow_background_style: 0,
            jungle_background_style: 0,
            hell_background_style: 0,
            forest_background: 0,
            corruption_background: 0,
            jungle_background: 0,
            snow_background: 0,
            hallow_background: 0,
            crimson_background: 0,
            desert_background: 0,
            ocean_background: 0,
            mushroom_background: 0,
            underworld_background: 0,
            forest_background_2: 0,
            forest_background_3: 0,
            forest_background_4: 0,
            cloud_background: 0,
            cloud_number: 0,
            wind_speed: 0.0,
            treetop_variants: vec![0; 13],
        }
    }
}
