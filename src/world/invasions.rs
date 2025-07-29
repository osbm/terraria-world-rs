use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvasionData {
    pub invasion_delay: i32,
    pub invasion_size: i32,
    pub invasion_type: i32,
    pub invasion_position: f64,
    pub invasion_size_start: i32,
    pub cultist_delay: i32,
    pub time_left_slime_rain: f64,
    pub old_ones_army_tier_1: bool,
    pub old_ones_army_tier_2: bool,
    pub old_ones_army_tier_3: bool,
}

impl Default for InvasionData {
    fn default() -> Self {
        Self {
            invasion_delay: 0,
            invasion_size: 0,
            invasion_type: 0,
            invasion_position: 0.0,
            invasion_size_start: 0,
            cultist_delay: 86400,
            time_left_slime_rain: -235157.0,
            old_ones_army_tier_1: false,
            old_ones_army_tier_2: false,
            old_ones_army_tier_3: false,
        }
    }
}
