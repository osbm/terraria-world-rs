use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneyPowers {
    pub freeze_time: bool,
    pub time_rate: f32,
    pub freeze_rain: bool,
    pub freeze_wind: bool,
    pub difficulty: f32,
    pub freeze_biome_spread: bool,
    // Store the order of power IDs as they were read from the file
    pub power_order: Vec<i16>,
}

impl JourneyPowers {
    pub fn new() -> Self {
        Self {
            freeze_time: false,
            time_rate: 1.0,
            freeze_rain: false,
            freeze_wind: false,
            difficulty: 1.0,
            freeze_biome_spread: false,
            power_order: Vec::new(),
        }
    }
}
