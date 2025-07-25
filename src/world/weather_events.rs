use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherAndEvents {
    pub current_time: f64,
    pub is_daytime: bool,
    pub moon_phase: u32,
    pub blood_moon: bool,
    pub eclipse: bool,
    pub is_rain_active: bool,
    pub rain_time_left: i32,
    pub max_rain: f32,
    pub is_sandstorm_active: bool,
    pub sandstorm_time_left: i32,
    pub sandstorm_severity: f32,
    pub sandstorm_intended_severity: f32,
    pub halloween_today: bool,
    pub christmas_today: bool,
    pub party_center_active: bool,
    pub party_natural_active: bool,
    pub party_cooldown: i32,
    pub partying_npcs: Vec<i32>,
    pub party_is_doomed: bool,
    pub lantern_nights_on_cooldown: i32,
    pub lantern_night_genuine: bool,
    pub lantern_night_manual: bool,
    pub next_night_is_lantern_night: bool,
}

impl Default for WeatherAndEvents {
    fn default() -> Self {
        Self {
            current_time: 13500.0,
            is_daytime: true,
            moon_phase: 0,
            blood_moon: false,
            eclipse: false,
            is_rain_active: false,
            rain_time_left: 0,
            max_rain: 0.0,
            is_sandstorm_active: false,
            sandstorm_time_left: 0,
            sandstorm_severity: 0.0,
            sandstorm_intended_severity: 0.0,
            halloween_today: false,
            christmas_today: false,
            party_center_active: false,
            party_natural_active: false,
            party_cooldown: 0,
            partying_npcs: Vec::new(),
            party_is_doomed: false,
            lantern_nights_on_cooldown: 0,
            lantern_night_genuine: false,
            lantern_night_manual: false,
            next_night_is_lantern_night: false,
        }
    }
}