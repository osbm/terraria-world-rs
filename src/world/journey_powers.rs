use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneyPowers {
    pub freeze_time: bool,
    pub time_rate: f32,
    pub freeze_rain: bool,
    pub freeze_wind: bool,
    pub difficulty: f32,
    pub freeze_biome_spread: bool,
}

impl Default for JourneyPowers {
    fn default() -> Self {
        Self::new()
    }
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
        }
    }
}
