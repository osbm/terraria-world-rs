use crate::reader::ByteReader;

mod pointers;
use pointers::Pointers;

#[derive(Debug)]
pub struct World {
    pub version_integer: i32,
    pub magic: String,
    pub savefile_type: u8,
    pub revision: u32,
    pub is_favorite: u64,
    pub pointer_count: u16,
    pub pointer_vector: Vec<u32>,
    pub tile_frame_important: Vec<bool>,
    pub unknown_file_format_data: Vec<u8>, // TODO: find out what this is
    pub world_name: String,
    pub generator_seed: String,
    pub generator_version: u64,
    pub uuid: String,
    pub id: i32,
    pub bounds_vec: Vec<i32>,
    pub world_height: i32,
    pub world_width: i32,
    pub difficulty_value: i32,
    pub is_drunk_world: bool,
    pub is_for_the_worthy: bool,
    pub is_tenth_anniversary: bool,
    pub is_the_constant: bool,
    pub is_bee_world: bool,
    pub is_upside_down: bool,
    pub is_trap_world: bool,
    pub is_zenith_world: bool,
    pub created_on: String,
    pub moon_style: u8,
    pub tree_style_seperators: Vec<i32>,
    pub tree_style_properties: Vec<i32>,
    pub moss_style_seperators: Vec<i32>,
    pub moss_style_properties: Vec<i32>,
    pub snow_background_style: i32,
    pub jungle_background_style: i32,
    pub hell_background_style: i32,
    pub spawn_point_x: i32,
    pub spawn_point_y: i32,
    pub underground_level: f64,
    pub cavern_level: f64,
    pub current_time: f64,
    pub is_daytime: bool,
    pub moon_phase: u32,
    pub blood_moon: bool,
    pub eclipse: bool,
    pub dungeon_point_x: i32,
    pub dungeon_point_y: i32,
    pub world_evil_type: bool,
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
    pub saved_goblin_tinkerer: bool,
    pub saved_wizard: bool,
    pub saved_mechanic: bool,
    pub defeated_goblin_army: bool,
    pub defeated_clown: bool,
    pub defeated_frost_moon: bool,
    pub defeated_pirate_invasion: bool,
    pub shadow_orbs_smashed_at_least_once: bool,
    pub shadow_orbs_spawn_meteorite: bool,
    pub shadow_orbs_evil_boss_counter: u8,
    pub altars_smashed: i32,
    pub is_hardmode: bool,
    pub party_is_doomed: bool,
    pub invasion_delay: i32,
    pub invasion_size: i32,
    pub invasion_type: i32,
    pub invasion_position: f64,
    pub time_left_slime_rain: f64,
    pub sundial_cooldown: u8,
    pub is_rain_active: bool,
    pub rain_time_left: i32,
    pub max_rain: f32,
    pub hardmode_ore_1: i32,
    pub hardmode_ore_2: i32,
    pub hardmode_ore_3: i32,
    pub forest_background: i8,
    pub corruption_background: i8,
    pub jungle_background: i8,
    pub snow_background: i8,
    pub hallow_background: i8,
    pub crimson_background: i8,
    pub desert_background: i8,
    pub ocean_background: i8,
    pub cloud_background: i32,
    pub cloud_number: i16,
    pub wind_speed: f32,
    pub angler_today_quest_completed_by: Vec<String>,
    pub saved_angler: bool,
    pub angler_daily_quest_target: i32,
    pub saved_stylist: bool,
    pub saved_tax_collector: bool,
    pub saved_golfer: bool,
    pub invasion_size_start: i32,
    pub cultist_delay: i32,
    pub mob_kills: Vec<i32>,
    pub sundial_is_running: bool,
    pub defeated_duke_fishron: bool,
    pub defeated_martian_madness: bool,
    pub defeated_lunatic_cultist: bool,
    pub deteated_moon_lord: bool,
    pub defeated_pumpking: bool,
    pub defeated_mourning_wood: bool,
    pub defeated_ice_queen: bool,
    pub defeated_santa_nk1: bool,
    pub defeated_everscream: bool,
    pub defeated_solar_pillar: bool,
    pub defeated_vortex_pillar: bool,
    pub defeated_nebula_pillar: bool,
    pub defeated_stardust_pillar: bool,
    pub lunar_events_pillars_present_solar: bool, // TODO find a better name
    pub lunar_events_pillars_present_vortex: bool,
    pub lunar_events_pillars_present_nebula: bool,
    pub lunar_events_pillars_present_stardust: bool,
    pub lunar_events_are_active: bool,
    pub party_center_active: bool,
    pub party_natural_active: bool,
    pub party_cooldown: i32,
    pub partying_npcs: Vec<i32>,
    pub is_sandstorm_active: bool,
    pub sandstorm_time_left: i32,
    pub sandstorm_severity: f32,
    pub sandstorm_intended_severity: f32,
    pub saved_bartender: bool,
    pub old_ones_army_tier_1: bool,
    pub old_ones_army_tier_2: bool,
    pub old_ones_army_tier_3: bool,
    pub mushroom_background: i8,
    pub underworld_background: i8,
    pub forest_background_2: i8,
    pub forest_background_3: i8,
    pub forest_background_4: i8,
    pub combat_book_used: bool,
    pub lantern_nights_on_cooldown: i32,
    pub lantern_night_genuine: bool,
    pub lantern_night_manual: bool,
    pub next_night_is_lantern_night: bool,
    pub treetop_variants: Vec<i32>,
    pub halloween_today: bool,
    pub christmas_today: bool,
    pub ore_1: i32,
    pub ore_2: i32,
    pub ore_3: i32,
    pub ore_4: i32,
    pub has_cat: bool,
    pub has_dog: bool,
    pub has_bunny: bool,
    pub defeated_empress_of_light: bool,
    pub defeated_queen_slime: bool,
    pub defeated_deerclops: bool,
    pub saved_slime_nerdy: bool,
    pub saved_merchant: bool,
    pub saved_demolitionist: bool,
    pub saved_party_girl: bool,
    pub saved_dye_trader: bool,
    pub saved_truffle: bool,
    pub saved_arms_dealer: bool,
    pub saved_nurse: bool,
    pub saved_princess: bool,
    pub combat_book_2_used: bool,
    pub peddler_satchel_used: bool,
    pub saved_slime_cool: bool,
    pub saved_slime_elder: bool,
    pub saved_slime_clumsy: bool,
    pub saved_slime_diva: bool,
    pub saved_slime_surly: bool,
    pub saved_slime_mystic: bool,
    pub saved_slime_squire: bool,


}

impl World {
    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let bytes = std::fs::read(path)?;
        let mut r = ByteReader::new(&bytes);

        let version_integer = r.i32();
        let magic = String::from_utf8_lossy(r.bytes(7)).to_string();
        let savefile_type = r.u8();
        let revision = r.u32();
        let is_favorite = r.u64();

        let pointer_count = r.u16();
        let mut pointer_vector = vec![];
        for _ in 0..pointer_count {
            pointer_vector.push(r.u32());
        }
        let pointers = Pointers::from_vector(&pointer_vector); // create this only to use it during parsing

        let tile_frame_important_size = (r.i16() + 7) / 8;
        let mut tile_frame_important = vec![];
        for _ in 0..tile_frame_important_size {
            let current_bits = r.bits();
            tile_frame_important.extend(current_bits);
        }

        let unknown_file_format_data = r.read_until(pointers.world_header as usize);
        let world_name = r.string(None);
        let generator_seed = r.string(None);
        let generator_version = r.u64();
        let uuid = r.uuid();
        let id = r.i32();

        let bounds_vec = vec![
            r.i32(), // left
            r.i32(), // right
            r.i32(), // top
            r.i32(), // bottom
        ];
        let world_height = r.i32();
        let world_width = r.i32();
        let difficulty_value = r.i32();
        let is_drunk_world = r.bool();
        // println!("File offset after reading world data: {}", r.offset());
        let is_for_the_worthy = r.bool();
        let is_tenth_anniversary = r.bool();
        let is_the_constant = r.bool();
        let is_bee_world = r.bool();
        let is_upside_down = r.bool();
        let is_trap_world = r.bool();
        let is_zenith_world = r.bool();
        let created_on = r.datetime();
        let moon_style = r.u8();
        let tree_style_seperators = vec![r.i32(), r.i32(), r.i32()];
        let tree_style_properties = vec![r.i32(), r.i32(), r.i32(), r.i32()];
        let moss_style_seperators = vec![r.i32(), r.i32(), r.i32()];
        let moss_style_properties = vec![r.i32(), r.i32(), r.i32(), r.i32()];
        let snow_background_style = r.i32();
        let jungle_background_style = r.i32();
        let hell_background_style = r.i32();
        let spawn_point_x = r.i32();
        let spawn_point_y = r.i32();
        let underground_level = r.f64();
        let cavern_level = r.f64();
        let current_time = r.f64();
        let is_daytime = r.bool();
        let moon_phase = r.u32();
        let blood_moon = r.bool();
        let eclipse = r.bool();
        let dungeon_point_x = r.i32();
        let dungeon_point_y = r.i32();
        let world_evil_type = r.bool();
        let defeated_eye_of_cthulhu = r.bool();
        let defeated_eater_of_worlds = r.bool();
        let defeated_skeletron = r.bool();
        let defeated_queen_bee = r.bool();
        let defeated_the_twins = r.bool();
        let defeated_the_destroyer = r.bool();
        let defeated_skeletron_prime = r.bool();
        let defeated_any_mechanical_boss = r.bool();
        let defeated_plantera = r.bool();
        let defeated_golem = r.bool();
        let defeated_king_slime = r.bool();
        let saved_goblin_tinkerer = r.bool();
        let saved_wizard = r.bool();
        let saved_mechanic = r.bool();
        let defeated_goblin_army = r.bool();
        let defeated_clown = r.bool();
        let defeated_frost_moon = r.bool();
        let defeated_pirate_invasion = r.bool();
        let shadow_orbs_smashed_at_least_once = r.bool();
        let shadow_orbs_spawn_meteorite = r.bool();
        let shadow_orbs_evil_boss_counter = r.u8();
        let altars_smashed = r.i32();
        let is_hardmode = r.bool();
        let party_is_doomed = !r.bool(); // ???
        let invasion_delay = r.i32();
        let invasion_size = r.i32();
        let invasion_type = r.i32();
        let invasion_position = r.f64();
        let time_left_slime_rain = r.f64();
        let sundial_cooldown = r.u8();
        let is_rain_active = r.bool();
        let rain_time_left = r.i32();
        let max_rain = r.f32();
        let hardmode_ore_1 = r.i32();
        let hardmode_ore_2 = r.i32();
        let hardmode_ore_3 = r.i32();
        let forest_background = r.i8();
        let corruption_background = r.i8();
        let jungle_background = r.i8();
        let snow_background = r.i8();
        let hallow_background = r.i8();
        let crimson_background = r.i8();
        let desert_background = r.i8();
        let ocean_background = r.i8();
        let cloud_background = r.i32();
        let cloud_number = r.i16();
        let wind_speed = r.f32();

        let angler_today_quest_completed_by_count = r.i32();
        let mut angler_today_quest_completed_by = vec![];
        for _ in 0..angler_today_quest_completed_by_count {
            let name = r.string(None);
            if !name.is_empty() {
                angler_today_quest_completed_by.push(name);
            }
        }

        let saved_angler = r.bool();
        let angler_daily_quest_target = r.i32();
        let saved_stylist = r.bool();
        let saved_tax_collector = r.bool();
        let saved_golfer = r.bool();
        let invasion_size_start = r.i32();
        let cultist_delay = r.i32();

        let mob_kills_count = r.i16();
        let mut mob_kills = vec![];
        for _ in 0..mob_kills_count {
            mob_kills.push(r.i32());
        }
        let sundial_is_running = r.bool();
        let defeated_duke_fishron = r.bool();
        let defeated_martian_madness = r.bool();
        let defeated_lunatic_cultist = r.bool();
        let deteated_moon_lord = r.bool();
        let defeated_pumpking = r.bool();
        let defeated_mourning_wood = r.bool();
        let defeated_ice_queen = r.bool();
        let defeated_santa_nk1 = r.bool();
        let defeated_everscream = r.bool();
        let defeated_solar_pillar = r.bool();
        let defeated_vortex_pillar = r.bool();
        let defeated_nebula_pillar = r.bool();
        let defeated_stardust_pillar = r.bool();
        let lunar_events_pillars_present_solar = r.bool();
        let lunar_events_pillars_present_vortex = r.bool();
        let lunar_events_pillars_present_nebula = r.bool();
        let lunar_events_pillars_present_stardust = r.bool();
        let lunar_events_are_active = r.bool();
        let party_center_active = r.bool();
        let party_natural_active = r.bool();
        let party_cooldown = r.i32();

        let partying_npcs_count = r.i32();
        let mut partying_npcs = vec![];
        for _ in 0..partying_npcs_count {
            partying_npcs.push(r.i32());
        }

        let is_sandstorm_active = r.bool();
        let sandstorm_time_left = r.i32();
        let sandstorm_severity = r.f32();
        let sandstorm_intended_severity = r.f32();
        let saved_bartender = r.bool();
        let old_ones_army_tier_1 = r.bool();
        let old_ones_army_tier_2 = r.bool();
        let old_ones_army_tier_3 = r.bool();
        let mushroom_background = r.i8();
        let underworld_background = r.i8();
        let forest_background_2 = r.i8();
        let forest_background_3 = r.i8();
        let forest_background_4 = r.i8();
        let combat_book_used = r.bool();
        let lantern_nights_on_cooldown = r.i32();
        let lantern_night_genuine = r.bool();
        let lantern_night_manual = r.bool();
        let next_night_is_lantern_night = r.bool();

        let treetop_variants_count = r.i32();
        let mut treetop_variants = vec![];
        for _ in 0..treetop_variants_count {
            treetop_variants.push(r.i32());
        }

        let halloween_today = r.bool();
        let christmas_today = r.bool();
        let ore_1 = r.i32();
        let ore_2 = r.i32();
        let ore_3 = r.i32();
        let ore_4 = r.i32();
        let has_cat = r.bool();
        let has_dog = r.bool();
        let has_bunny = r.bool();
        let defeated_empress_of_light = r.bool();
        let defeated_queen_slime = r.bool();
        let defeated_deerclops = r.bool();
        let saved_slime_nerdy = r.bool();
        let saved_merchant = r.bool();
        let saved_demolitionist = r.bool();
        let saved_party_girl = r.bool();
        let saved_dye_trader = r.bool();
        let saved_truffle = r.bool();
        let saved_arms_dealer = r.bool();
        let saved_nurse = r.bool();
        let saved_princess = r.bool();
        let combat_book_2_used = r.bool();
        let peddler_satchel_used = r.bool();
        let saved_slime_cool = r.bool();
        let saved_slime_elder = r.bool();
        let saved_slime_clumsy = r.bool();
        let saved_slime_diva = r.bool();
        let saved_slime_surly = r.bool();
        let saved_slime_mystic = r.bool();
        let saved_slime_squire = r.bool();


        Ok(Self {
            version_integer,
            magic,
            savefile_type,
            revision,
            is_favorite,
            pointer_count,
            pointer_vector,
            tile_frame_important,
            unknown_file_format_data,
            world_name,
            generator_seed,
            generator_version,
            uuid,
            id,
            bounds_vec,
            world_height,
            world_width,
            difficulty_value,
            is_drunk_world,
            is_for_the_worthy,
            is_tenth_anniversary,
            is_the_constant,
            is_bee_world,
            is_upside_down,
            is_trap_world,
            is_zenith_world,
            created_on,
            moon_style,
            tree_style_seperators,
            tree_style_properties,
            moss_style_seperators,
            moss_style_properties,
            snow_background_style,
            jungle_background_style,
            hell_background_style,
            spawn_point_x,
            spawn_point_y,
            underground_level,
            cavern_level,
            current_time,
            is_daytime,
            moon_phase,
            blood_moon,
            eclipse,
            dungeon_point_x,
            dungeon_point_y,
            world_evil_type,
            defeated_eye_of_cthulhu,
            defeated_eater_of_worlds,
            defeated_skeletron,
            defeated_queen_bee,
            defeated_the_twins,
            defeated_the_destroyer,
            defeated_skeletron_prime,
            defeated_any_mechanical_boss,
            defeated_plantera,
            defeated_golem,
            defeated_king_slime,
            saved_goblin_tinkerer,
            saved_wizard,
            saved_mechanic,
            defeated_goblin_army,
            defeated_clown,
            defeated_frost_moon,
            defeated_pirate_invasion,
            shadow_orbs_smashed_at_least_once,
            shadow_orbs_spawn_meteorite,
            shadow_orbs_evil_boss_counter,
            altars_smashed,
            is_hardmode,
            party_is_doomed,
            invasion_delay,
            invasion_size,
            invasion_type,
            invasion_position,
            time_left_slime_rain,
            sundial_cooldown,
            is_rain_active,
            rain_time_left,
            max_rain,
            hardmode_ore_1,
            hardmode_ore_2,
            hardmode_ore_3,
            forest_background,
            corruption_background,
            jungle_background,
            snow_background,
            hallow_background,
            crimson_background,
            desert_background,
            ocean_background,
            cloud_background,
            cloud_number,
            wind_speed,
            angler_today_quest_completed_by,
            saved_angler,
            angler_daily_quest_target,
            saved_stylist,
            saved_tax_collector,
            saved_golfer,
            invasion_size_start,
            cultist_delay,
            mob_kills,
            sundial_is_running,
            defeated_duke_fishron,
            defeated_martian_madness,
            defeated_lunatic_cultist,
            deteated_moon_lord,
            defeated_pumpking,
            defeated_mourning_wood,
            defeated_ice_queen,
            defeated_santa_nk1,
            defeated_everscream,
            defeated_solar_pillar,
            defeated_vortex_pillar,
            defeated_nebula_pillar,
            defeated_stardust_pillar,
            lunar_events_pillars_present_solar,
            lunar_events_pillars_present_vortex,
            lunar_events_pillars_present_nebula,
            lunar_events_pillars_present_stardust,
            lunar_events_are_active,
            party_center_active,
            party_natural_active,
            party_cooldown,
            partying_npcs,
            is_sandstorm_active,
            sandstorm_time_left,
            sandstorm_severity,
            sandstorm_intended_severity,
            saved_bartender,
            old_ones_army_tier_1,
            old_ones_army_tier_2,
            old_ones_army_tier_3,
            mushroom_background,
            underworld_background,
            forest_background_2,
            forest_background_3,
            forest_background_4,
            combat_book_used,
            lantern_nights_on_cooldown,
            lantern_night_genuine,
            lantern_night_manual,
            next_night_is_lantern_night,
            treetop_variants,
            halloween_today,
            christmas_today,
            ore_1,
            ore_2,
            ore_3,
            ore_4,
            has_cat,
            has_dog,
            has_bunny,
            defeated_empress_of_light,
            defeated_queen_slime,
            defeated_deerclops,
            saved_slime_nerdy,
            saved_merchant,
            saved_demolitionist,
            saved_party_girl,
            saved_dye_trader,
            saved_truffle,
            saved_arms_dealer,
            saved_nurse,
            saved_princess,
            combat_book_2_used,
            peddler_satchel_used,
            saved_slime_cool,
            saved_slime_elder,
            saved_slime_clumsy,
            saved_slime_diva,
            saved_slime_surly,
            saved_slime_mystic,
            saved_slime_squire,

        })
    }

    pub fn version(&self) -> &str {
        if self.version_integer != 279 {
            eprintln!("⚠️ Warning: This parser was tested only on version 279 (1.4.4.9). Parsed version is {}", self.version_integer);
        }

        return match self.version_integer {
            12 => "1.0.5",          // unconfirmed
            20 => "1.0.6",          // unconfirmed
            22 => "1.0.6.1",        // unconfirmed
            37 => "1.1.1",          // unconfirmed
            39 => "1.1.2",          // unconfirmed
            67 => "1.2",            // unconfirmed
            71 => "1.2.0.3.1",      // unconfirmed
            72 => "1.2.1.1",        // unconfirmed
            73 => "1.2.1.2",        // unconfirmed
            77 => "1.2.2",          // unconfirmed
            94 => "1.2.3.1",        // unconfirmed
            101 => "1.2.4",         // unconfirmed
            102 => "1.2.4.1",       // unconfirmed
            140 | 146 => "1.3.0.1", // unconfirmed
            147 => "1.3.0.2",       // unconfirmed
            149 => "1.3.0.3",       // unconfirmed
            151 => "1.3.0.4",       // unconfirmed
            153 => "1.3.0.5",       // unconfirmed
            154 => "1.3.0.6",       // unconfirmed
            155 => "1.3.0.7",       // unconfirmed
            156 => "1.3.0.8",       // unconfirmed
            168 => "1.3.1",         // unconfirmed
            169 => "1.3.1.1",       // unconfirmed
            170 => "1.3.2",         // unconfirmed
            173 => "1.3.2.1",       // unconfirmed
            174 => "1.3.3",         // unconfirmed
            175 => "1.3.3.1",       // unconfirmed
            176 => "1.3.3.2",       // unconfirmed
            177 => "1.3.3.3",       // unconfirmed
            178 => "1.3.4",         // unconfirmed
            185 => "1.3.4.1",       // unconfirmed
            186 => "1.3.4.2",       // unconfirmed
            187 => "1.3.4.3",       // unconfirmed
            188 => "1.3.4.4",       // unconfirmed
            191 => "1.3.5",         // unconfirmed
            192 => "1.3.5.1",       // unconfirmed
            193 => "1.3.5.2",       // unconfirmed
            194 => "1.3.5.3",       // unconfirmed
            225 => "1.4.0.1",       // unconfirmed
            226 => "1.4.0.2",       // unconfirmed
            227 => "1.4.0.3",       // unconfirmed
            228 => "1.4.0.4",       // unconfirmed
            230 => "1.4.0.5",       // unconfirmed
            238 => "1.4.2.3",       // unconfirmed
            274 => "1.4.4.5",       // unconfirmed
            278 => "1.4.4.8",       // unconfirmed
            279 => "1.4.4.9",
            _ => "Unknon version",
        };
    }

    pub fn pointers(&self) -> Pointers {
        Pointers::from_vector(&self.pointer_vector)
    }

    pub fn difficulty(&self) -> &str {
        match self.difficulty_value {
            0 => "Classic",
            1 => "Expert",
            2 => "Master",
            3 => "Journey",
            _ => "Unknown",
        }
    }

    pub fn remove_corruption(self) -> Self {
        println!("Removing corruption...");
        // raise unimplemented error
        unimplemented!("Corruption removal is not implemented yet.");
    }

    pub fn read_from_json(path: &str) -> std::io::Result<Self> {
        println!("Reading from {path}...");
        unimplemented!("Reading functionality is not implemented yet.");
    }

    pub fn save_as_json(&self, path: &str) -> std::io::Result<()> {
        println!("Saving to {path}...");
        unimplemented!("Saving functionality is not implemented yet.");
    }

    pub fn save_as_wld(&self, path: &str) -> std::io::Result<()> {
        println!("Saving to {path}...");
        // test if this produces exactly the same file as the original
        unimplemented!("Saving functionality is not implemented yet.");
    }
}
