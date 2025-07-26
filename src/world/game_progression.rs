use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(Default)]
pub struct GameProgression {
    // Boss defeats
    pub defeated_eye_of_cthulhu: bool,
    pub defeated_eater_of_worlds: bool,
    pub defeated_skeletron: bool,
    pub defeated_queen_bee: bool,
    pub defeated_the_twins: bool,
    pub defeated_the_destroyer: bool,
    pub defeated_skeletron_prime: bool,
    pub defeated_any_mechanical_boss: bool,
    pub defeated_plantera: bool,
    pub defeated_golem: bool,
    pub defeated_king_slime: bool,
    pub defeated_duke_fishron: bool,
    pub defeated_martian_madness: bool,
    pub defeated_lunatic_cultist: bool,
    pub defeated_moon_lord: bool,
    pub defeated_pumpking: bool,
    pub defeated_mourning_wood: bool,
    pub defeated_ice_queen: bool,
    pub defeated_santa_nk1: bool,
    pub defeated_everscream: bool,
    pub defeated_empress_of_light: bool,
    pub defeated_queen_slime: bool,
    pub defeated_deerclops: bool,

    // Pillars
    pub defeated_solar_pillar: bool,
    pub defeated_vortex_pillar: bool,
    pub defeated_nebula_pillar: bool,
    pub defeated_stardust_pillar: bool,
    pub lunar_events_pillars_present_solar: bool,
    pub lunar_events_pillars_present_vortex: bool,
    pub lunar_events_pillars_present_nebula: bool,
    pub lunar_events_pillars_present_stardust: bool,
    pub lunar_events_are_active: bool,

    // Events
    pub defeated_goblin_army: bool,
    pub defeated_clown: bool,
    pub defeated_frost_moon: bool,
    pub defeated_pirate_invasion: bool,

    // Game state
    pub is_hardmode: bool,
    pub shadow_orbs_smashed_at_least_once: bool,
    pub shadow_orbs_spawn_meteorite: bool,
    pub shadow_orbs_evil_boss_counter: u8,
    pub altars_smashed: i32,
}

// i will write default values for this struct
// to initialize it easily
// using GameProgression::default() will give you a new instance

//implement a function to maximize the game progression
impl GameProgression {
    pub fn maximize(&mut self) {
        self.defeated_eye_of_cthulhu = true;
        self.defeated_eater_of_worlds = true;
        self.defeated_skeletron = true;
        self.defeated_queen_bee = true;
        self.defeated_the_twins = true;
        self.defeated_the_destroyer = true;
        self.defeated_skeletron_prime = true;
        self.defeated_any_mechanical_boss = true;
        self.defeated_plantera = true;
        self.defeated_golem = true;
        self.defeated_king_slime = true;
        self.defeated_duke_fishron = true;
        self.defeated_martian_madness = true;
        self.defeated_lunatic_cultist = true;
        self.defeated_moon_lord = true;
        self.defeated_pumpking = true;
        self.defeated_mourning_wood = true;
        self.defeated_ice_queen = true;
        self.defeated_santa_nk1 = true;
        self.defeated_everscream = true;
        self.defeated_empress_of_light = true;
        self.defeated_queen_slime = true;
        self.defeated_deerclops = true;
        self.defeated_solar_pillar = true;
        self.defeated_vortex_pillar = true;
        self.defeated_nebula_pillar = true;
        self.defeated_stardust_pillar = true;
        self.lunar_events_pillars_present_solar = true;
        self.lunar_events_pillars_present_vortex = true;
        self.lunar_events_pillars_present_nebula = true;
        self.lunar_events_pillars_present_stardust = true;
        self.lunar_events_are_active = true;
        self.defeated_goblin_army = true;
        self.defeated_clown = true;
        self.defeated_frost_moon = true;
        self.defeated_pirate_invasion = true;
        self.is_hardmode = true;
        self.shadow_orbs_smashed_at_least_once = true;
        self.shadow_orbs_spawn_meteorite = true;
        self.shadow_orbs_evil_boss_counter = 2;
        self.altars_smashed = 100;
    }
}