use rand::Rng;
use uuid::Uuid;

use crate::reader::ByteReader;
use crate::writer::ByteWriter;

// Module declarations
pub mod bestiary;
pub mod chest;
pub mod coordinates;
pub mod enums;
pub mod environment;
pub mod error;
pub mod game_progression;
pub mod invasions;
pub mod item;
pub mod journey_powers;
pub mod mob;
pub mod npc;
pub mod pressure_plate;
pub mod room;
pub mod saved_npcs;
pub mod sign;
pub mod tile;
pub mod tile_entity;
pub mod weather_events;

use self::enums::LiquidType;
use self::tile::{FrameImportantData, Tile, TileMatrix};
use serde::{Deserialize, Serialize};

// Import all the moved types from their submodules
use crate::world::bestiary::Bestiary;
use crate::world::chest::Chest;
use crate::world::coordinates::Coordinates;
use crate::world::environment::WorldEnvironment;
use crate::world::error::InvalidFooterError;
use crate::world::game_progression::GameProgression;
use crate::world::invasions::InvasionData;
use crate::world::item::ItemStack;
use crate::world::journey_powers::JourneyPowers;
use crate::world::mob::Mob;
use crate::world::npc::NPC;
use crate::world::pressure_plate::WeighedPressurePlate;
use crate::world::room::Room;
use crate::world::saved_npcs::SavedNPCs;
use crate::world::sign::Sign;
use crate::world::tile_entity::{TileEntity, TileEntityExtra};
use crate::world::weather_events::WeatherAndEvents;

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    // Core world info
    pub version_integer: i32,
    pub savefile_type: u8,
    pub revision: u32,
    pub is_favorite: u64,
    pub tile_frame_important: Vec<bool>,
    pub world_name: String,
    pub generator_seed: String,
    pub generator_version: u64,
    pub uuid: String,
    pub id: i32,
    pub bounds_vec: Vec<i32>,
    pub world_height: i32,
    pub world_width: i32,
    pub difficulty_value: i32,

    // World modifiers
    pub is_drunk_world: bool,
    pub is_for_the_worthy: bool,
    pub is_tenth_anniversary: bool,
    pub is_the_constant: bool,
    pub is_bee_world: bool,
    pub is_upside_down: bool,
    pub is_trap_world: bool,
    pub is_zenith_world: bool,
    pub created_on: String,

    // Grouped data
    pub game_progression: GameProgression,
    pub saved_npcs: SavedNPCs,
    pub environment: WorldEnvironment,
    pub weather_events: WeatherAndEvents,
    pub invasions: InvasionData,

    // Spawn and level data
    pub spawn_point_x: i32,
    pub spawn_point_y: i32,
    pub underground_level: f64,
    pub cavern_level: f64,
    pub dungeon_point_x: i32,
    pub dungeon_point_y: i32,
    pub world_evil_type: bool,

    // Hardmode ores and misc
    pub hardmode_ore_1: i32,
    pub hardmode_ore_2: i32,
    pub hardmode_ore_3: i32,
    pub ore_1: i32,
    pub ore_2: i32,
    pub ore_3: i32,
    pub ore_4: i32,

    // Pets and items
    pub has_cat: bool,
    pub has_dog: bool,
    pub has_bunny: bool,
    pub combat_book_used: bool,
    pub combat_book_2_used: bool,
    pub peddler_satchel_used: bool,

    // Angler quest data
    pub angler_today_quest_completed_by: Vec<String>,
    pub angler_daily_quest_target: i32,

    // Mob data
    pub mob_kills: Vec<i32>,

    // Sundial and moondial
    pub sundial_cooldown: u8,
    pub sundial_is_running: bool,
    pub moondial_is_running: bool,
    pub moondial_cooldown: u8,

    // World content
    pub tiles: TileMatrix,
    pub chests_max_items: i16,
    pub chests: Vec<Chest>,
    pub signs: Vec<Sign>,
    pub npcs: Vec<NPC>,
    pub mobs: Vec<Mob>,
    pub shimmered_npcs: Vec<i32>,
    pub tile_entities: Vec<TileEntity>,
    pub weighed_pressure_plates: Vec<WeighedPressurePlate>,
    pub rooms: Vec<Room>,
    pub bestiary: Bestiary,
    pub journey_powers: JourneyPowers,
}

impl World {
    pub fn new(world_name: &str, world_size: &str, difficulty: &str) -> Self {
        let (world_width, world_height) = match world_size {
            "small" => (4200, 1200),
            "medium" => (6400, 1800),
            "large" => (8400, 2400),
            _ => panic!("Invalid world size. Options are: small, medium, large"),
        };
        let cavern_level: f64 = match world_size {
            "small" => 451.0,
            "medium" => 733.0,
            "large" => 847.0,
            _ => panic!("Invalid world size. Options are: small, medium, large"),
        };
        let underground_level: f64 = match world_size {
            "small" => 337.0,
            "medium" => 493.0,
            "large" => 649.0,
            _ => panic!("Invalid world size. Options are: small, medium, large"),
        };
        let difficulty_value = match difficulty {
            "journey" => 3,
            "classic" => 0,
            "expert" => 1,
            "master" => 2,
            _ => panic!("Invalid difficulty. Options are: journey, classic, expert, master"),
        };
        let guide_npc = NPC::new(
            22,
            "Jacob".to_string(),
            world_width as f32 * 16.0 / 2.0,
            world_height as f32 * 16.0 / 2.0 + (16.0 * 4.0),
            true,
            Coordinates {
                x: 4196,
                y: 572
            },
            0
        );
        Self {
            version_integer: 279,
            savefile_type: 2,
            revision: 1,
            is_favorite: 0,
            tile_frame_important: Vec::new(),
            world_name: world_name.to_string(),
            generator_seed: "osbm/terraria-world-rs".to_string(),
            generator_version: 0,
            uuid: Uuid::new_v4().to_string(),
            id: rand::rng().random_range(1..=i32::MAX),
            bounds_vec: vec![0, world_width * 16, 0, world_height * 16],
            world_height,
            world_width,
            difficulty_value,
            is_drunk_world: false,
            is_for_the_worthy: false,
            is_tenth_anniversary: false,
            is_the_constant: false,
            is_bee_world: false,
            is_upside_down: false,
            is_trap_world: false,
            is_zenith_world: false,
            created_on: chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S%.f")
                .to_string(),
            game_progression: GameProgression::default(),
            saved_npcs: SavedNPCs::default(),
            environment: WorldEnvironment::default(),
            weather_events: WeatherAndEvents::default(),
            invasions: InvasionData::default(),
            spawn_point_x: 0,
            spawn_point_y: 0,
            underground_level,
            cavern_level,
            dungeon_point_x: 0,
            dungeon_point_y: 0,
            world_evil_type: false,
            hardmode_ore_1: -1,
            hardmode_ore_2: -1,
            hardmode_ore_3: -1,
            ore_1: 7,
            ore_2: 6,
            ore_3: 9,
            ore_4: 169,
            has_cat: false,
            has_dog: false,
            has_bunny: false,
            combat_book_used: false,
            combat_book_2_used: false,
            peddler_satchel_used: false,
            angler_today_quest_completed_by: Vec::new(),
            angler_daily_quest_target: 0,
            // mob_kills is 688 zeros
            mob_kills: vec![0; 688],
            sundial_cooldown: 0,
            sundial_is_running: false,
            moondial_is_running: false,
            moondial_cooldown: 0,
            tiles: TileMatrix::new((world_width as usize, world_height as usize)),
            chests_max_items: 0,
            chests: Vec::new(),
            signs: Vec::new(),
            npcs: vec![guide_npc],
            mobs: Vec::new(),
            shimmered_npcs: Vec::new(),
            tile_entities: Vec::new(),
            weighed_pressure_plates: Vec::new(),
            rooms: Vec::new(),
            bestiary: Bestiary::new(Vec::new(), Vec::new(), Vec::new()),
            journey_powers: JourneyPowers::new(),
        }
    }

    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let bytes = std::fs::read(path)?;
        let mut r = ByteReader::new(&bytes);

        let version_integer = r.i32();

        let magic = String::from_utf8_lossy(r.bytes(7)).to_string();
        if magic != "relogic" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid magic string in the file header. Please open an issue at https://github.com/osbm/terraria-world-rs/issues"));
        }

        let savefile_type = r.u8();
        let revision = r.u32();
        let is_favorite = r.u64();

        let pointer_count = r.u16();
        let mut pointer_vector = vec![];
        for _ in 0..pointer_count {
            pointer_vector.push(r.u32());
        }

        let tile_frame_important_count = r.i16();
        let tile_frame_important_size = (tile_frame_important_count + 7) / 8;
        let mut tile_frame_important = vec![];
        for _ in 0..tile_frame_important_size {
            let current_bits = r.bits();
            tile_frame_important.extend(current_bits);
        }
        tile_frame_important.truncate(tile_frame_important_count as usize);

        if r.offset() as u32 != pointer_vector[0] {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Pointer mismatch after file header. Please open an issue at https://github.com/osbm/terraria-world-rs/issues"));
        }

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
        let is_for_the_worthy = r.bool();
        let is_tenth_anniversary = r.bool();
        let is_the_constant = r.bool();
        let is_bee_world = r.bool();
        let is_upside_down = r.bool();
        let is_trap_world = r.bool();
        let is_zenith_world = r.bool();
        let created_on = r.datetime();
        let moon_style = r.u8();
        let tree_style_separators = vec![r.i32(), r.i32(), r.i32()];
        let tree_style_properties = vec![r.i32(), r.i32(), r.i32(), r.i32()];
        let moss_style_separators = vec![r.i32(), r.i32(), r.i32()];
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
        let defeated_moon_lord = r.bool();
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
        let moondial_is_running = r.bool();
        let moondial_cooldown = r.u8();

        if r.offset() as u32 != pointer_vector[1] {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Pointer mismatch after world header section. Please open an issue at https://github.com/osbm/terraria-world-rs/issues"));
        }

        // tiles
        let (width, height) = (world_width as usize, world_height as usize);
        let tiles = Self::create_tile_matrix(&mut r, (width, height), &tile_frame_important);

        if r.offset() as u32 != pointer_vector[2] {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Pointer mismatch after tiles section. Please open an issue at https://github.com/osbm/terraria-world-rs/issues"));
        }

        // --- CHEST PARSING ---
        let chests_count = r.i16();
        let chests_max_items = r.i16();
        let mut chests = Vec::with_capacity(chests_count as usize);
        for _ in 0..chests_count {
            let chest_x = r.i32();
            let chest_y = r.i32();
            let chest_name = r.string(None);
            let mut chest_contents = Vec::with_capacity(chests_max_items as usize);
            for _ in 0..chests_max_items {
                let item_quantity = r.i16();
                if item_quantity > 0 {
                    let item_type = r.i32();
                    let item_prefix = r.u8();
                    chest_contents.push(Some(ItemStack {
                        quantity: item_quantity,
                        type_id: item_type,
                        prefix: item_prefix,
                    }));
                } else {
                    chest_contents.push(None);
                }
            }
            chests.push(Chest {
                position: Coordinates {
                    x: chest_x,
                    y: chest_y,
                },
                name: chest_name,
                contents: chest_contents,
            });
        }

        if r.offset() as u32 != pointer_vector[3] {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Pointer mismatch after chests section. Please open an issue at https://github.com/osbm/terraria-world-rs/issues"));
        }

        // --- SIGN PARSING ---
        let signs_count = r.i16();
        let mut signs = Vec::with_capacity(signs_count as usize);
        for _ in 0..signs_count {
            let sign_text = r.string(None);
            let sign_x = r.i32();
            let sign_y = r.i32();
            signs.push(Sign {
                text: sign_text,
                position: Coordinates {
                    x: sign_x,
                    y: sign_y,
                },
            });
        }

        if r.offset() as u32 != pointer_vector[4] {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Pointer mismatch after signs section. Please open an issue at https://github.com/osbm/terraria-world-rs/issues"));
        }

        // Parse entities
        let mut npcs = Vec::new();
        let mut mobs = Vec::new();

        // Parse shimmered NPCs
        let shimmered_npcs_count = r.i32();
        let mut shimmered_npcs = Vec::with_capacity(shimmered_npcs_count as usize);
        for _i in 0..shimmered_npcs_count {
            let npc_id = r.i32();
            shimmered_npcs.push(npc_id);
        }

        // Parse NPCs
        let mut _npc_index = 0;
        while r.bool() {
            let npc_type = r.i32();
            let npc_name = r.string(None);
            let npc_position_x = r.f32();
            let npc_position_y = r.f32();
            let is_homeless = r.bool();
            let npc_home = Coordinates {
                x: r.i32(),
                y: r.i32(),
            };
            let npc_flags = r.bits();
            let npc_variation_index = r.i32();
            if !npc_flags[0] {
                let _npc_variation_index = 0i32;
            }
            let npc = NPC::new(
                npc_type,
                npc_name,
                npc_position_x,
                npc_position_y,
                is_homeless,
                npc_home,
                npc_variation_index,
            );
            npcs.push(npc);
            _npc_index += 1;
        }

        // Parse mobs
        while r.bool() {
            let mob_type = r.i32();
            let mob_position_x = r.f32();
            let mob_position_y = r.f32();
            let mob = Mob::new(mob_type, mob_position_x, mob_position_y);
            mobs.push(mob);
        }

        if r.offset() as u32 != pointer_vector[5] {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Pointer mismatch after NPCs and mobs section. Please open an issue at https://github.com/osbm/terraria-world-rs/issues"));
        }

        // Parse tile entities
        let tile_entities_count = r.i32();
        let mut tile_entities = Vec::with_capacity(tile_entities_count as usize);
        for _ in 0..tile_entities_count {
            let te_type = r.u8();
            let te_id = r.i32();
            let te_position = Coordinates {
                x: r.i16() as i32,
                y: r.i16() as i32,
            };

            let te_extra = match te_type {
                0 => {
                    // Target Dummy
                    let npc = r.i16();
                    Some(TileEntityExtra::TargetDummy { npc })
                }
                1 => {
                    // Item Frame
                    let item_type = r.i16();
                    let item_prefix = r.u8();
                    let item_quantity = r.i16();
                    let item = ItemStack {
                        quantity: item_quantity,
                        type_id: item_type as i32,
                        prefix: item_prefix,
                    };
                    Some(TileEntityExtra::ItemFrame { item })
                }
                2 => {
                    // Logic Sensor
                    let logic_check = r.u8();
                    let enabled = r.bool();
                    Some(TileEntityExtra::LogicSensor {
                        logic_check,
                        enabled,
                    })
                }
                3 => {
                    // Mannequin
                    let item_flags = r.bits();
                    let dye_flags = r.bits();
                    let mut mannequin_items = vec![None; item_flags.len()];
                    let mut mannequin_dyes = vec![None; dye_flags.len()];

                    for (index, &flag) in item_flags.iter().enumerate() {
                        if !flag {
                            continue;
                        }
                        let item_type = r.i16();
                        let item_prefix = r.u8();
                        let item_quantity = r.i16();
                        mannequin_items[index] = Some(ItemStack {
                            quantity: item_quantity,
                            type_id: item_type as i32,
                            prefix: item_prefix,
                        });
                    }

                    for (index, &flag) in dye_flags.iter().enumerate() {
                        if !flag {
                            continue;
                        }
                        let item_type = r.i16();
                        let item_prefix = r.u8();
                        let item_quantity = r.i16();
                        mannequin_dyes[index] = Some(ItemStack {
                            quantity: item_quantity,
                            type_id: item_type as i32,
                            prefix: item_prefix,
                        });
                    }

                    Some(TileEntityExtra::Mannequin {
                        items: mannequin_items,
                        dyes: mannequin_dyes,
                    })
                }
                4 => {
                    // Weapon Rack
                    let item_type = r.i16();
                    let item_prefix = r.u8();
                    let item_quantity = r.i16();
                    let item = ItemStack {
                        quantity: item_quantity,
                        type_id: item_type as i32,
                        prefix: item_prefix,
                    };
                    Some(TileEntityExtra::WeaponRack { item })
                }
                5 => {
                    // Hat Rack
                    let item_flags = r.bits();
                    let mut rack_items = vec![None; 2];
                    let mut rack_dyes = vec![None; 2];

                    for (index, &flag) in item_flags.iter().take(2).enumerate() {
                        if !flag {
                            continue;
                        }
                        let item_type = r.i16();
                        let item_prefix = r.u8();
                        let item_quantity = r.i16();
                        rack_items[index] = Some(ItemStack {
                            quantity: item_quantity,
                            type_id: item_type as i32,
                            prefix: item_prefix,
                        });
                    }

                    for (index, &flag) in item_flags.iter().skip(2).take(2).enumerate() {
                        if !flag {
                            continue;
                        }
                        let item_type = r.i16();
                        let item_prefix = r.u8();
                        let item_quantity = r.i16();
                        rack_dyes[index] = Some(ItemStack {
                            quantity: item_quantity,
                            type_id: item_type as i32,
                            prefix: item_prefix,
                        });
                    }

                    Some(TileEntityExtra::HatRack {
                        items: rack_items,
                        dyes: rack_dyes,
                    })
                }
                6 => {
                    // Food Plate
                    let item_type = r.i16();
                    let item_prefix = r.u8();
                    let item_quantity = r.i16();
                    let item = ItemStack {
                        quantity: item_quantity,
                        type_id: item_type as i32,
                        prefix: item_prefix,
                    };
                    Some(TileEntityExtra::Plate { item })
                }
                7 => {
                    // Teleport Pylon
                    Some(TileEntityExtra::Pylon)
                }
                _ => {
                    // println!("Unknown tile entity type: {}", te_type);
                    None
                }
            };

            let tile_entity = TileEntity::new(te_id, te_position, te_extra);
            tile_entities.push(tile_entity);
        }

        if r.offset() as u32 != pointer_vector[6] {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Pointer mismatch after tile entities section. Please open an issue at https://github.com/osbm/terraria-world-rs/issues"));
        }

        // Parse weighed pressure plates
        let weighed_pressure_plates_count = r.i32();
        let mut weighed_pressure_plates =
            Vec::with_capacity(weighed_pressure_plates_count as usize);
        for _ in 0..weighed_pressure_plates_count {
            let position = Coordinates {
                x: r.i32(),
                y: r.i32(),
            };
            weighed_pressure_plates.push(WeighedPressurePlate::new(position));
        }

        if r.offset() as u32 != pointer_vector[7] {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Pointer mismatch after weighed pressure plates section. Please open an issue at https://github.com/osbm/terraria-world-rs/issues"));
        }

        // Parse town manager (rooms)
        let rooms_count = r.i32();
        let mut rooms = Vec::with_capacity(rooms_count as usize);
        for _ in 0..rooms_count {
            let npc = r.i32();
            let position = Coordinates {
                x: r.i32(),
                y: r.i32(),
            };
            rooms.push(Room::new(npc, position));
        }

        if r.offset() as u32 != pointer_vector[8] {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Pointer mismatch after rooms section. Please open an issue at https://github.com/osbm/terraria-world-rs/issues"));
        }

        // Parse bestiary
        let bestiary_kills_count = r.i32();
        let mut bestiary_kills = Vec::with_capacity(bestiary_kills_count as usize);
        for _ in 0..bestiary_kills_count {
            let entity = r.string(None);
            let kills = r.i32();
            bestiary_kills.push((entity, kills));
        }

        let bestiary_sightings_count = r.i32();
        let mut bestiary_sightings = Vec::with_capacity(bestiary_sightings_count as usize);
        for _ in 0..bestiary_sightings_count {
            bestiary_sightings.push(r.string(None));
        }

        let bestiary_chats_count = r.i32();
        let mut bestiary_chats = Vec::with_capacity(bestiary_chats_count as usize);
        for _ in 0..bestiary_chats_count {
            bestiary_chats.push(r.string(None));
        }

        let bestiary = Bestiary::new(bestiary_kills, bestiary_sightings, bestiary_chats);

        if r.offset() as u32 != pointer_vector[9] {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Pointer mismatch after bestiary section. Please open an issue at https://github.com/osbm/terraria-world-rs/issues"));
        }

        // Parse journey powers
        let mut journey_powers = JourneyPowers::new();
        while r.bool() {
            let power_id = r.i16();
            match power_id {
                0 => journey_powers.freeze_time = r.bool(),
                8 => journey_powers.time_rate = r.f32(),
                9 => journey_powers.freeze_rain = r.bool(),
                10 => journey_powers.freeze_wind = r.bool(),
                12 => journey_powers.difficulty = r.f32(),
                13 => journey_powers.freeze_biome_spread = r.bool(),
                _ => {
                    println!("Unknown journey power ID: {power_id} please open a issue at github.com/osbm/terraria-world-rs");
                }
            }
        }

        if r.offset() as u32 != pointer_vector[10] {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Pointer mismatch after journey powers section. Please open an issue at https://github.com/osbm/terraria-world-rs/issues"));
        }

        // Parse footer
        if !r.bool() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                InvalidFooterError("Invalid footer".to_string()),
            ));
        }
        let footer_world_name = r.string(None);
        if footer_world_name != world_name {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                InvalidFooterError("Invalid footer - world name mismatch".to_string()),
            ));
        }
        let footer_world_id = r.i32();
        if footer_world_id != id {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                InvalidFooterError("Invalid footer - world ID mismatch".to_string()),
            ));
        }

        let game_progression = GameProgression {
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
            defeated_duke_fishron,
            defeated_martian_madness,
            defeated_lunatic_cultist,
            defeated_moon_lord,
            defeated_pumpking,
            defeated_mourning_wood,
            defeated_ice_queen,
            defeated_santa_nk1,
            defeated_everscream,
            defeated_empress_of_light,
            defeated_queen_slime,
            defeated_deerclops,
            defeated_solar_pillar,
            defeated_vortex_pillar,
            defeated_nebula_pillar,
            defeated_stardust_pillar,
            lunar_events_pillars_present_solar,
            lunar_events_pillars_present_vortex,
            lunar_events_pillars_present_nebula,
            lunar_events_pillars_present_stardust,
            lunar_events_are_active,
            defeated_goblin_army,
            defeated_clown,
            defeated_frost_moon,
            defeated_pirate_invasion,
            is_hardmode,
            shadow_orbs_smashed_at_least_once,
            shadow_orbs_spawn_meteorite,
            shadow_orbs_evil_boss_counter,
            altars_smashed,
        };

        let saved_npcs = SavedNPCs {
            saved_goblin_tinkerer,
            saved_wizard,
            saved_mechanic,
            saved_angler,
            saved_stylist,
            saved_tax_collector,
            saved_golfer,
            saved_bartender,
            saved_slime_nerdy,
            saved_merchant,
            saved_demolitionist,
            saved_party_girl,
            saved_dye_trader,
            saved_truffle,
            saved_arms_dealer,
            saved_nurse,
            saved_princess,
            saved_slime_cool,
            saved_slime_elder,
            saved_slime_clumsy,
            saved_slime_diva,
            saved_slime_surly,
            saved_slime_mystic,
            saved_slime_squire,
        };

        let environment = WorldEnvironment {
            moon_style,
            tree_style_separators,
            tree_style_properties,
            moss_style_separators,
            moss_style_properties,
            snow_background_style,
            jungle_background_style,
            hell_background_style,
            forest_background,
            corruption_background,
            jungle_background,
            snow_background,
            hallow_background,
            crimson_background,
            desert_background,
            ocean_background,
            mushroom_background,
            underworld_background,
            forest_background_2,
            forest_background_3,
            forest_background_4,
            cloud_background,
            cloud_number,
            wind_speed,
            treetop_variants,
        };

        let weather_events = WeatherAndEvents {
            current_time,
            is_daytime,
            moon_phase,
            blood_moon,
            eclipse,
            is_rain_active,
            rain_time_left,
            max_rain,
            is_sandstorm_active,
            sandstorm_time_left,
            sandstorm_severity,
            sandstorm_intended_severity,
            halloween_today,
            christmas_today,
            party_center_active,
            party_natural_active,
            party_cooldown,
            partying_npcs,
            party_is_doomed,
            lantern_nights_on_cooldown,
            lantern_night_genuine,
            lantern_night_manual,
            next_night_is_lantern_night,
        };

        let invasions = InvasionData {
            invasion_delay,
            invasion_size,
            invasion_type,
            invasion_position,
            invasion_size_start,
            cultist_delay,
            time_left_slime_rain,
            old_ones_army_tier_1,
            old_ones_army_tier_2,
            old_ones_army_tier_3,
        };

        let world = World {
            version_integer,
            savefile_type,
            revision,
            is_favorite,
            tile_frame_important,
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
            game_progression,
            saved_npcs,
            environment,
            weather_events,
            invasions,
            spawn_point_x,
            spawn_point_y,
            underground_level,
            cavern_level,
            dungeon_point_x,
            dungeon_point_y,
            world_evil_type,
            hardmode_ore_1,
            hardmode_ore_2,
            hardmode_ore_3,
            ore_1,
            ore_2,
            ore_3,
            ore_4,
            has_cat,
            has_dog,
            has_bunny,
            combat_book_used,
            combat_book_2_used,
            peddler_satchel_used,
            angler_today_quest_completed_by,
            angler_daily_quest_target,
            mob_kills,
            sundial_cooldown,
            sundial_is_running,
            moondial_is_running,
            moondial_cooldown,
            tiles,
            chests_max_items,
            chests,
            signs,
            npcs,
            mobs,
            shimmered_npcs,
            tile_entities,
            weighed_pressure_plates,
            rooms,
            bestiary,
            journey_powers,
        };

        Ok(world)
    }

    pub fn version(&self) -> &str {
        if self.version_integer != 279 {
            eprintln!("⚠️ Warning: This parser was tested only on version 279 (1.4.4.9). Parsed version is {}", self.version_integer);
        }

        match self.version_integer {
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
        }
    }

    pub fn difficulty(&self) -> &str {
        match self.difficulty_value {
            0 => "Classic",
            1 => "Expert",
            2 => "Master",
            3 => "Journey",
            _ => "Invalid difficulty",
        }
    }

    // set block by name
    pub fn set_block(&mut self, x: usize, y: usize, block_name: &str) {
        if x < self.world_width as usize && y < self.world_height as usize {
            let tile = &mut self.tiles.tiles[x][y];
            tile.set_block_name(block_name);
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < self.world_width as usize && y < self.world_height as usize {
            Some(&self.tiles.tiles[x][y])
        } else {
            None
        }
    }

    pub fn get_all_possible_block_names(&self) -> Vec<String> {
        // TODO return from the enums
        Vec::new()
    }

    pub fn set_wall(&mut self, x: usize, y: usize, wall_name: &str) {
        if x < self.world_width as usize && y < self.world_height as usize {
            let tile = &mut self.tiles.tiles[x][y];
            tile.set_wall_name(wall_name);
        } else {
            eprintln!(
                "Coordinates ({}, {}) are out of bounds for the world size {}x{}.",
                x, y, self.world_width, self.world_height
            );
        }
    }

    pub fn place_tile(&mut self, x: usize, y: usize, tile: Tile) {
        if x < self.world_width as usize && y < self.world_height as usize {
            self.tiles.tiles[x][y] = tile;
        } else {
            eprintln!(
                "Coordinates ({}, {}) are out of bounds for the world size {}x{}.",
                x, y, self.world_width, self.world_height
            );
        }
    }

    // there is 3 types of corruption: TODO make it 100% match the game
    // 0 - Corruption
    // 1 - Crimson
    // 2 - Hallow
    // and game tells us how many of each block is the percentage of the world
    // like your world is 4% corrupted, 2% crimson, and 1% hallowed
    pub fn get_corruption_stats(&self) -> (f32, f32, f32) {
        // https://terraria.wiki.gg/wiki/Dryad#World_status
        let mut corruption_count = 0;
        let mut crimson_count = 0;
        let mut hallow_count = 0;
        let mut total_nonempty_tiles = 0;

        let corruption_blocks = [
            "CORRUPT_ICE",
            "CORRUPT_HARDENED_SAND",
            "CORRUPT_SANDSTONE",
            "CORRUPT_VINES",
            "CRIMSON_JUNGLE_GRASS",
            "CORRUPT_GRASS",
            "EBONSTONE",
            "EBONSAND",
            "EBONWOOD",
        ];

        let crimson_blocks = [
            "CRIMSON_HARDENED_SAND",
            "CRIMSON_HARDENED_SANDSTONE",
            "CRIMSON_VINES",
            "CRIMSTONE",
            "CRIMSAND",
            "CRIMTANE_THORNS",
            "CRIMSON_JUNGLE_GRASS",
            "FLESH_ICE",
            "FLESH_WEEDS",
            "FLESH_GRASS",
        ];

        let hallow_blocks = [
            "HALLOWED_ICE",
            "HALLOW_HARDENED_SAND",
            "HALLOW_SANDSTONE",
            "PEARLSAND",
            "PEARLSTONE",
            "GOLF_GRASS_HALLOWED",
            "HALLOWED_GRASS",
            "HALLOWED_PLANTS",
            "HALLOWED_PLANTS2",
            "HALLOWED_VINES",
        ];

        let solid_blocks = [
            "STONE",
            "GRASS",
            // "DIRT",
            "ICE",
            "JUNGLE_GRASS",
            "MOWED_GOLF_GRASS",
        ];

        for x in 0..self.world_width as usize {
            for y in 0..self.world_height as usize {
                let tile = &self.tiles.tiles[x][y];
                let block_name = tile.get_block_name();

                if corruption_blocks.contains(&block_name) {
                    corruption_count += 1;
                } else if crimson_blocks.contains(&block_name) {
                    crimson_count += 1;
                } else if hallow_blocks.contains(&block_name) {
                    hallow_count += 1;
                }

                if solid_blocks.contains(&block_name) {
                    total_nonempty_tiles += 1;
                }
            }
        }
        total_nonempty_tiles += corruption_count + hallow_count + crimson_count;

        (
            corruption_count as f32 * 100.0 / total_nonempty_tiles as f32,
            crimson_count as f32 * 100.0 / total_nonempty_tiles as f32,
            hallow_count as f32 * 100.0 / total_nonempty_tiles as f32,
        )
    }

    pub fn read_from_json(path: &str) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let world = serde_json::from_reader(reader)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(world)
    }

    pub fn save_as_wld(&self, path: &str) -> std::io::Result<()> {
        // Create all section writers
        let world_header_writer = self.write_world_header_section();
        let tiles_writer = self.write_tiles_section();
        let chests_writer = self.write_chests_section();
        let signs_writer = self.write_signs_section();
        let npcs_writer = self.write_npcs_section();
        let tile_entities_writer = self.write_tile_entities_section();
        let pressure_plates_writer = self.write_pressure_plates_section();
        let town_manager_writer = self.write_town_manager_section();
        let bestiary_writer = self.write_bestiary_section();
        let journey_powers_writer = self.write_journey_powers_section();
        let footer_writer = self.write_footer_section();

        // Create header writer with placeholders
        let mut header_writer = ByteWriter::new();
        header_writer.i32(self.version_integer);
        header_writer.bytes("relogic".as_bytes());
        header_writer.u8(self.savefile_type);
        header_writer.u32(self.revision);
        header_writer.u64(self.is_favorite);
        header_writer.u16(11); // 11 sections

        // Write placeholder pointers (will be updated later)
        for _ in 0..11 {
            header_writer.u32(0);
        }

        // Write tile_frame_important count and bits in the file header
        let original_count = self.tile_frame_important.len() as i16;
        header_writer.i16(original_count);
        for chunk in self.tile_frame_important.chunks(8) {
            header_writer.bits(chunk);
        }

        // Calculate section lengths and update pointers
        let mut current_offset = header_writer.offset() as u32;

        // Update pointer vector with actual offsets
        let mut pointer_vector = Vec::new();

        // Section 0: World Header (starts after the fixed header)
        pointer_vector.push(current_offset);
        current_offset += world_header_writer.offset() as u32;

        // Section 1: Tiles
        pointer_vector.push(current_offset);
        current_offset += tiles_writer.offset() as u32;

        // Section 2: Chests
        pointer_vector.push(current_offset);
        current_offset += chests_writer.offset() as u32;

        // Section 3: Signs
        pointer_vector.push(current_offset);
        current_offset += signs_writer.offset() as u32;

        // Section 4: NPCs
        pointer_vector.push(current_offset);
        current_offset += npcs_writer.offset() as u32;

        // Section 5: Tile Entities
        pointer_vector.push(current_offset);
        current_offset += tile_entities_writer.offset() as u32;

        // Section 6: Pressure Plates
        pointer_vector.push(current_offset);
        current_offset += pressure_plates_writer.offset() as u32;

        // Section 7: Town Manager
        pointer_vector.push(current_offset);
        current_offset += town_manager_writer.offset() as u32;

        // Section 8: Bestiary
        pointer_vector.push(current_offset);
        current_offset += bestiary_writer.offset() as u32;

        // Section 9: Journey Powers
        pointer_vector.push(current_offset);
        current_offset += journey_powers_writer.offset() as u32;

        // Section 10: Footer
        pointer_vector.push(current_offset);
        let _unused_offset = current_offset + footer_writer.offset() as u32;

        // Write the complete file
        let mut final_writer = ByteWriter::new();

        // Write header with updated pointers
        final_writer.i32(self.version_integer);
        final_writer.bytes("relogic".as_bytes());
        // print the current offset for debugging
        final_writer.u8(self.savefile_type);
        final_writer.u32(self.revision);
        final_writer.u64(self.is_favorite);
        final_writer.u16(pointer_vector.len() as u16);

        // Write actual pointer values from world object for debugging section sizes
        for pointer in pointer_vector {
            final_writer.u32(pointer);
        }

        // Write tile_frame_important count and bits in the file header
        final_writer.i16(original_count);
        for chunk in self.tile_frame_important.chunks(8) {
            final_writer.bits(chunk);
        }

        // Write all section buffers
        final_writer.bytes(&world_header_writer.into_inner());
        final_writer.bytes(&tiles_writer.into_inner());
        final_writer.bytes(&chests_writer.into_inner());
        final_writer.bytes(&signs_writer.into_inner());
        final_writer.bytes(&npcs_writer.into_inner());
        final_writer.bytes(&tile_entities_writer.into_inner());
        final_writer.bytes(&pressure_plates_writer.into_inner());
        final_writer.bytes(&town_manager_writer.into_inner());
        final_writer.bytes(&bestiary_writer.into_inner());
        final_writer.bytes(&journey_powers_writer.into_inner());
        final_writer.bytes(&footer_writer.into_inner());

        // Write buffer to file
        let buffer = final_writer.into_inner();
        std::fs::write(path, buffer)?;

        Ok(())
    }

    fn write_world_header_section(&self) -> ByteWriter {
        let mut writer = ByteWriter::new();

        writer.string(&self.world_name);
        writer.string(&self.generator_seed);
        writer.u64(self.generator_version);
        writer.uuid(&self.uuid);
        writer.i32(self.id);

        // Write bounds_vec (left, right, top, bottom)
        for v in &self.bounds_vec {
            writer.i32(*v);
        }

        // Write world_height, world_width, difficulty_value, flags, created_on, moon_style
        writer.i32(self.world_height);
        writer.i32(self.world_width);
        writer.i32(self.difficulty_value);
        writer.bool(self.is_drunk_world);
        writer.bool(self.is_for_the_worthy);
        writer.bool(self.is_tenth_anniversary);
        writer.bool(self.is_the_constant);
        writer.bool(self.is_bee_world);
        writer.bool(self.is_upside_down);
        writer.bool(self.is_trap_world);
        writer.bool(self.is_zenith_world);
        writer.datetime(&self.created_on);
        writer.u8(self.environment.moon_style);

        // Write tree_style_separators, tree_style_properties, moss_style_separators, moss_style_properties
        for v in &self.environment.tree_style_separators {
            writer.i32(*v);
        }
        for v in &self.environment.tree_style_properties {
            writer.i32(*v);
        }
        for v in &self.environment.moss_style_separators {
            writer.i32(*v);
        }
        for v in &self.environment.moss_style_properties {
            writer.i32(*v);
        }

        // Write background styles
        writer.i32(self.environment.snow_background_style);
        writer.i32(self.environment.jungle_background_style);
        writer.i32(self.environment.hell_background_style);

        // Write spawn point, underground/cavern levels, time, day, moon, events, dungeon, world evil, boss flags, etc.
        writer.i32(self.spawn_point_x);
        writer.i32(self.spawn_point_y);
        writer.f64(self.underground_level);
        writer.f64(self.cavern_level);
        writer.f64(self.weather_events.current_time);
        writer.bool(self.weather_events.is_daytime);
        writer.u32(self.weather_events.moon_phase);
        writer.bool(self.weather_events.blood_moon);
        writer.bool(self.weather_events.eclipse);
        writer.i32(self.dungeon_point_x);
        writer.i32(self.dungeon_point_y);
        writer.bool(self.world_evil_type);
        writer.bool(self.game_progression.defeated_eye_of_cthulhu);
        writer.bool(self.game_progression.defeated_eater_of_worlds);
        writer.bool(self.game_progression.defeated_skeletron);
        writer.bool(self.game_progression.defeated_queen_bee);
        writer.bool(self.game_progression.defeated_the_twins);
        writer.bool(self.game_progression.defeated_the_destroyer);
        writer.bool(self.game_progression.defeated_skeletron_prime);
        writer.bool(self.game_progression.defeated_any_mechanical_boss);
        writer.bool(self.game_progression.defeated_plantera);
        writer.bool(self.game_progression.defeated_golem);
        writer.bool(self.game_progression.defeated_king_slime);
        writer.bool(self.saved_npcs.saved_goblin_tinkerer);
        writer.bool(self.saved_npcs.saved_wizard);
        writer.bool(self.saved_npcs.saved_mechanic);
        writer.bool(self.game_progression.defeated_goblin_army);
        writer.bool(self.game_progression.defeated_clown);
        writer.bool(self.game_progression.defeated_frost_moon);
        writer.bool(self.game_progression.defeated_pirate_invasion);
        writer.bool(self.game_progression.shadow_orbs_smashed_at_least_once);
        writer.bool(self.game_progression.shadow_orbs_spawn_meteorite);
        writer.u8(self.game_progression.shadow_orbs_evil_boss_counter);
        writer.i32(self.game_progression.altars_smashed);
        writer.bool(self.game_progression.is_hardmode);
        writer.bool(!self.weather_events.party_is_doomed); // party_is_doomed is inverted
        writer.i32(self.invasions.invasion_delay);
        writer.i32(self.invasions.invasion_size);
        writer.i32(self.invasions.invasion_type);
        writer.f64(self.invasions.invasion_position);
        writer.f64(self.invasions.time_left_slime_rain);
        writer.u8(self.sundial_cooldown);
        writer.bool(self.weather_events.is_rain_active);
        writer.i32(self.weather_events.rain_time_left);
        writer.f32(self.weather_events.max_rain);
        writer.i32(self.hardmode_ore_1);
        writer.i32(self.hardmode_ore_2);
        writer.i32(self.hardmode_ore_3);
        writer.i8(self.environment.forest_background);
        writer.i8(self.environment.corruption_background);
        writer.i8(self.environment.jungle_background);
        writer.i8(self.environment.snow_background);
        writer.i8(self.environment.hallow_background);
        writer.i8(self.environment.crimson_background);
        writer.i8(self.environment.desert_background);
        writer.i8(self.environment.ocean_background);
        writer.i32(self.environment.cloud_background);
        writer.i16(self.environment.cloud_number);
        writer.f32(self.environment.wind_speed);

        // Angler quest completed by
        writer.i32(self.angler_today_quest_completed_by.len() as i32);
        for name in &self.angler_today_quest_completed_by {
            writer.string(name);
        }

        // Angler and other NPCs
        writer.bool(self.saved_npcs.saved_angler);
        writer.i32(self.angler_daily_quest_target);
        writer.bool(self.saved_npcs.saved_stylist);
        writer.bool(self.saved_npcs.saved_tax_collector);
        writer.bool(self.saved_npcs.saved_golfer);
        writer.i32(self.invasions.invasion_size_start);
        writer.i32(self.invasions.cultist_delay);

        // Mob kills
        writer.i16(self.mob_kills.len() as i16);
        for v in &self.mob_kills {
            writer.i32(*v);
        }
        writer.bool(self.sundial_is_running);
        writer.bool(self.game_progression.defeated_duke_fishron);
        writer.bool(self.game_progression.defeated_martian_madness);
        writer.bool(self.game_progression.defeated_lunatic_cultist);
        writer.bool(self.game_progression.defeated_moon_lord);
        writer.bool(self.game_progression.defeated_pumpking);
        writer.bool(self.game_progression.defeated_mourning_wood);
        writer.bool(self.game_progression.defeated_ice_queen);
        writer.bool(self.game_progression.defeated_santa_nk1);
        writer.bool(self.game_progression.defeated_everscream);
        writer.bool(self.game_progression.defeated_solar_pillar);
        writer.bool(self.game_progression.defeated_vortex_pillar);
        writer.bool(self.game_progression.defeated_nebula_pillar);
        writer.bool(self.game_progression.defeated_stardust_pillar);
        writer.bool(self.game_progression.lunar_events_pillars_present_solar);
        writer.bool(self.game_progression.lunar_events_pillars_present_vortex);
        writer.bool(self.game_progression.lunar_events_pillars_present_nebula);
        writer.bool(self.game_progression.lunar_events_pillars_present_stardust);
        writer.bool(self.game_progression.lunar_events_are_active);
        writer.bool(self.weather_events.party_center_active);
        writer.bool(self.weather_events.party_natural_active);
        writer.i32(self.weather_events.party_cooldown);
        writer.i32(self.weather_events.partying_npcs.len() as i32);
        for v in &self.weather_events.partying_npcs {
            writer.i32(*v);
        }
        writer.bool(self.weather_events.is_sandstorm_active);
        writer.i32(self.weather_events.sandstorm_time_left);
        writer.f32(self.weather_events.sandstorm_severity);
        writer.f32(self.weather_events.sandstorm_intended_severity);
        writer.bool(self.saved_npcs.saved_bartender);
        writer.bool(self.invasions.old_ones_army_tier_1);
        writer.bool(self.invasions.old_ones_army_tier_2);
        writer.bool(self.invasions.old_ones_army_tier_3);
        writer.i8(self.environment.mushroom_background);
        writer.i8(self.environment.underworld_background);
        writer.i8(self.environment.forest_background_2);
        writer.i8(self.environment.forest_background_3);
        writer.i8(self.environment.forest_background_4);
        writer.bool(self.combat_book_used);
        writer.i32(self.weather_events.lantern_nights_on_cooldown);
        writer.bool(self.weather_events.lantern_night_genuine);
        writer.bool(self.weather_events.lantern_night_manual);
        writer.bool(self.weather_events.next_night_is_lantern_night);
        writer.i32(self.environment.treetop_variants.len() as i32);
        for v in &self.environment.treetop_variants {
            writer.i32(*v);
        }
        writer.bool(self.weather_events.halloween_today);
        writer.bool(self.weather_events.christmas_today);
        writer.i32(self.ore_1);
        writer.i32(self.ore_2);
        writer.i32(self.ore_3);
        writer.i32(self.ore_4);
        writer.bool(self.has_cat);
        writer.bool(self.has_dog);
        writer.bool(self.has_bunny);
        writer.bool(self.game_progression.defeated_empress_of_light);
        writer.bool(self.game_progression.defeated_queen_slime);
        writer.bool(self.game_progression.defeated_deerclops);
        writer.bool(self.saved_npcs.saved_slime_nerdy);
        writer.bool(self.saved_npcs.saved_merchant);
        writer.bool(self.saved_npcs.saved_demolitionist);
        writer.bool(self.saved_npcs.saved_party_girl);
        writer.bool(self.saved_npcs.saved_dye_trader);
        writer.bool(self.saved_npcs.saved_truffle);
        writer.bool(self.saved_npcs.saved_arms_dealer);
        writer.bool(self.saved_npcs.saved_nurse);
        writer.bool(self.saved_npcs.saved_princess);
        writer.bool(self.combat_book_2_used);
        writer.bool(self.peddler_satchel_used);
        writer.bool(self.saved_npcs.saved_slime_cool);
        writer.bool(self.saved_npcs.saved_slime_elder);
        writer.bool(self.saved_npcs.saved_slime_clumsy);
        writer.bool(self.saved_npcs.saved_slime_diva);
        writer.bool(self.saved_npcs.saved_slime_surly);
        writer.bool(self.saved_npcs.saved_slime_mystic);
        writer.bool(self.saved_npcs.saved_slime_squire);
        writer.bool(self.moondial_is_running);
        writer.u8(self.moondial_cooldown);

        writer
    }

    fn write_tiles_section(&self) -> ByteWriter {
        let mut writer = ByteWriter::new();
        let tiles = &self.tiles.tiles;

        for column in tiles {
            let mut y: usize = 0;
            while y < self.world_height as usize {
                // Find run length for RLE
                let mut run_length = 1;
                while y + run_length < self.world_height as usize
                    && column[y].tiles_equal(&column[y + run_length])
                    && run_length < 0x10000
                {
                    run_length += 1;
                }
                let tile_bytes = Self::serialize_tile(&column[y], run_length);
                writer.bytes(&tile_bytes.into_inner());
                y += run_length;
            }
        }
        writer
    }

    fn serialize_tile(tile: &Tile, repetition_count: usize) -> ByteWriter {
        let mut tile_bytes = ByteWriter::new();
        // --- Flag Byte 1 ---
        let mut flags1 = 0u8;
        let mut flags2 = 0u8;
        let mut flags3 = 0u8;
        let mut flags4 = 0u8;
        let mut has_flags2 = false;
        let mut has_flags3 = false;
        let mut has_flags4 = false;

        // Block
        let has_block = tile.has_block();
        if has_block {
            flags1 |= 1 << 1;
        }
        // Wall
        let has_wall = tile.has_wall();
        if has_wall {
            flags1 |= 1 << 2;
        }
        // Liquid
        let has_water = tile.liquid_type == LiquidType::Water && tile.liquid_amount > 0;
        let has_lava = tile.liquid_type == LiquidType::Lava && tile.liquid_amount > 0;
        let has_honey = tile.liquid_type == LiquidType::Honey && tile.liquid_amount > 0;
        let has_shimmer = tile.liquid_type == LiquidType::Shimmer && tile.liquid_amount > 0;
        if has_water || has_honey || has_shimmer {
            flags1 |= 1 << 3;
        } // Set water bit for shimmer too
        if has_lava || has_honey {
            flags1 |= 1 << 4;
        }
        // Extended block id
        let has_extended_block_id = has_block && tile.block_id > 255;
        if has_extended_block_id {
            flags1 |= 1 << 5;
        }
        // RLE
        let rle_val = if repetition_count - 1 > 0xFF {
            2
        } else if repetition_count > 1 {
            1
        } else {
            0
        };
        flags1 |= (rle_val & 0x03) << 6;

        // --- Flag Byte 2 ---
        // Wires
        if tile.red_wire {
            flags2 |= 1 << 1;
            has_flags2 = true;
        }
        if tile.blue_wire {
            flags2 |= 1 << 2;
            has_flags2 = true;
        }
        if tile.green_wire {
            flags2 |= 1 << 3;
            has_flags2 = true;
        }
        // Block shape (bits 4,5,6)
        let shape = tile.block_shape & 0x07;
        if (shape & 0b001) != 0 {
            flags2 |= 1 << 4;
            has_flags2 = true;
        }
        if (shape & 0b010) != 0 {
            flags2 |= 1 << 5;
            has_flags2 = true;
        }
        if (shape & 0b100) != 0 {
            flags2 |= 1 << 6;
            has_flags2 = true;
        }

        // --- Flag Byte 3 ---
        // Yellow wire
        if tile.yellow_wire {
            flags3 |= 1 << 1;
            has_flags3 = true;
            has_flags2 = true;
            flags2 |= 1 << 0;
        }
        // Block inactive (active = !inactive)
        if !tile.block_active {
            flags3 |= 1 << 2;
            has_flags3 = true;
            has_flags2 = true;
            flags2 |= 1 << 0;
        }
        // Block painted
        if tile.block_paint.is_some() {
            flags3 |= 1 << 3;
            has_flags3 = true;
            has_flags2 = true;
            flags2 |= 1 << 0;
        }
        // Wall painted
        if tile.wall_paint.is_some() {
            flags3 |= 1 << 4;
            has_flags3 = true;
            has_flags2 = true;
            flags2 |= 1 << 0;
        }
        // Actuator
        if tile.activator_wire {
            flags3 |= 1 << 5;
            has_flags3 = true;
            has_flags2 = true;
            flags2 |= 1 << 0;
        }
        // Extended wall id
        let has_extended_wall_id = has_wall && tile.wall_id > 255;
        if has_extended_wall_id {
            flags3 |= 1 << 6;
            has_flags3 = true;
            has_flags2 = true;
            flags2 |= 1 << 0;
        }
        // Shimmer liquid
        if has_shimmer {
            flags3 |= 1 << 7;
            has_flags3 = true;
            has_flags2 = true;
            flags2 |= 1 << 0;
        }
        // If any flag3 bits set, set flag2.0
        if has_flags3 {
            flags2 |= 1 << 0;
        }

        // --- Flag Byte 4 ---
        // Block echo
        if tile.block_echo {
            flags4 |= 1 << 1;
            has_flags4 = true;
            has_flags3 = true;
            has_flags2 = true;
            flags3 |= 1 << 0;
        }
        // Wall echo
        if tile.wall_echo {
            flags4 |= 1 << 2;
            has_flags4 = true;
            has_flags3 = true;
            has_flags2 = true;
            flags3 |= 1 << 0;
        }
        // Block illuminant
        if tile.block_illuminant {
            flags4 |= 1 << 3;
            has_flags4 = true;
            has_flags3 = true;
            has_flags2 = true;
            flags3 |= 1 << 0;
        }
        // Wall illuminant
        if tile.wall_illuminant {
            flags4 |= 1 << 4;
            has_flags4 = true;
            has_flags3 = true;
            has_flags2 = true;
            flags3 |= 1 << 0;
        }
        // If any flag4 bits set, set flag3.0
        if has_flags4 {
            flags3 |= 1 << 0;
        }

        // If any flag3 bits set, set flag2.0
        if has_flags3 {
            flags2 |= 1 << 0;
        }

        // Now set Flag 1.0 (has Flag Byte 2) after all Flag Byte 2 logic is complete
        if has_flags2 {
            flags1 |= 1 << 0;
        }
        // Write flag bytes
        tile_bytes.u8(flags1);
        if has_flags2 {
            tile_bytes.u8(flags2);
        }
        if has_flags3 {
            tile_bytes.u8(flags3);
        }
        if has_flags4 {
            tile_bytes.u8(flags4);
        }

        // Block
        if has_block {
            if has_extended_block_id {
                tile_bytes.u16(tile.block_id);
            } else {
                tile_bytes.u8(tile.block_id as u8);
            }
            // Frame important
            if tile.block_frame.is_some() {
                let frame = tile.block_frame.as_ref().unwrap();
                tile_bytes.u16(frame.x);
                tile_bytes.u16(frame.y);
            }
            // Block paint
            if let Some(paint) = tile.block_paint {
                tile_bytes.u8(paint);
            }
        }
        // Wall
        if has_wall {
            tile_bytes.u8((tile.wall_id & 0xFF) as u8);
            if has_extended_wall_id {
                tile_bytes.u8((tile.wall_id >> 8) as u8);
            }
            // Wall paint
            if let Some(paint) = tile.wall_paint {
                tile_bytes.u8(paint);
            }
        }
        // Liquid
        if has_water || has_lava || has_honey || has_shimmer {
            tile_bytes.u8(tile.liquid_amount);
        }
        // RLE
        match rle_val {
            2 => tile_bytes.u16((repetition_count - 1) as u16),
            1 => tile_bytes.u8((repetition_count - 1) as u8),
            _ => {}
        }
        tile_bytes
    }

    fn write_chests_section(&self) -> ByteWriter {
        let mut writer = ByteWriter::new();

        writer.i16(self.chests.len() as i16);
        writer.i16(self.chests_max_items);
        for chest in &self.chests {
            writer.i32(chest.position.x);
            writer.i32(chest.position.y);
            writer.string(&chest.name);
            for item in &chest.contents {
                if let Some(item) = item {
                    writer.i16(item.quantity);
                    writer.i32(item.type_id);
                    writer.u8(item.prefix);
                } else {
                    writer.i16(0);
                }
            }
        }

        writer
    }

    fn write_signs_section(&self) -> ByteWriter {
        let mut writer = ByteWriter::new();

        writer.i16(self.signs.len() as i16);
        for sign in &self.signs {
            writer.string(&sign.text);
            writer.i32(sign.position.x);
            writer.i32(sign.position.y);
        }

        writer
    }

    fn write_npcs_section(&self) -> ByteWriter {
        let mut writer = ByteWriter::new();

        writer.i32(self.shimmered_npcs.len() as i32);
        for id in &self.shimmered_npcs {
            writer.i32(*id);
        }
        // Write npcs
        for npc in &self.npcs {
            writer.bool(true); // presence flag
            writer.i32(npc.type_);
            writer.string(&npc.name);
            writer.f32(npc.position_x);
            writer.f32(npc.position_y);
            writer.bool(npc.is_homeless);
            writer.i32(npc.home.x);
            writer.i32(npc.home.y);
            writer.bits(&[true, false, false, false, false, false, false, false]); // npc_flags (placeholder)
            writer.i32(npc.variation_index);
        }
        writer.bool(false); // end of npcs
                            // Write mobs
        for mob in &self.mobs {
            writer.bool(true);
            writer.i32(mob.type_);
            writer.f32(mob.position_x);
            writer.f32(mob.position_y);
        }
        writer.bool(false); // end of mobs
        writer
    }

    fn write_tile_entities_section(&self) -> ByteWriter {
        let mut writer = ByteWriter::new();

        writer.i32(self.tile_entities.len() as i32);
        for te in &self.tile_entities {
            let (te_type, extra) = match &te.extra {
                Some(crate::world::TileEntityExtra::TargetDummy { .. }) => (0u8, &te.extra),
                Some(crate::world::TileEntityExtra::ItemFrame { .. }) => (1u8, &te.extra),
                Some(crate::world::TileEntityExtra::LogicSensor { .. }) => (2u8, &te.extra),
                Some(crate::world::TileEntityExtra::Mannequin { .. }) => (3u8, &te.extra),
                Some(crate::world::TileEntityExtra::WeaponRack { .. }) => (4u8, &te.extra),
                Some(crate::world::TileEntityExtra::HatRack { .. }) => (5u8, &te.extra),
                Some(crate::world::TileEntityExtra::Plate { .. }) => (6u8, &te.extra),
                Some(crate::world::TileEntityExtra::Pylon) => (7u8, &te.extra),
                _ => (255u8, &te.extra),
            };
            writer.u8(te_type);
            writer.i32(te.id);
            writer.i16(te.position.x as i16);
            writer.i16(te.position.y as i16);
            match extra {
                Some(crate::world::TileEntityExtra::TargetDummy { npc }) => {
                    writer.i16(*npc);
                }
                Some(crate::world::TileEntityExtra::ItemFrame { item }) => {
                    writer.i16(item.type_id as i16);
                    writer.u8(item.prefix);
                    writer.i16(item.quantity);
                }
                Some(crate::world::TileEntityExtra::LogicSensor {
                    logic_check,
                    enabled,
                }) => {
                    writer.u8(*logic_check);
                    writer.bool(*enabled);
                }
                Some(crate::world::TileEntityExtra::Mannequin { items, dyes }) => {
                    let item_flags: Vec<bool> = items.iter().map(|i| i.is_some()).collect();
                    let dye_flags: Vec<bool> = dyes.iter().map(|i| i.is_some()).collect();
                    writer.bits(&item_flags);
                    writer.bits(&dye_flags);
                    for item in items.iter().flatten() {
                        writer.i16(item.type_id as i16);
                        writer.u8(item.prefix);
                        writer.i16(item.quantity);
                    }
                    for dye in dyes.iter().flatten() {
                        writer.i16(dye.type_id as i16);
                        writer.u8(dye.prefix);
                        writer.i16(dye.quantity);
                    }
                }
                Some(crate::world::TileEntityExtra::WeaponRack { item }) => {
                    writer.i16(item.type_id as i16);
                    writer.u8(item.prefix);
                    writer.i16(item.quantity);
                }
                Some(crate::world::TileEntityExtra::HatRack { items, dyes }) => {
                    let item_flags: Vec<bool> = items
                        .iter()
                        .chain(dyes.iter())
                        .map(|i| i.is_some())
                        .collect();
                    writer.bits(&item_flags);
                    for item in items.iter().chain(dyes.iter()).flatten() {
                        writer.i16(item.type_id as i16);
                        writer.u8(item.prefix);
                        writer.i16(item.quantity);
                    }
                }
                Some(crate::world::TileEntityExtra::Plate { item }) => {
                    writer.i16(item.type_id as i16);
                    writer.u8(item.prefix);
                    writer.i16(item.quantity);
                }
                Some(crate::world::TileEntityExtra::Pylon) => {}
                _ => {}
            }
        }

        writer
    }

    fn write_pressure_plates_section(&self) -> ByteWriter {
        let mut writer = ByteWriter::new();

        writer.i32(self.weighed_pressure_plates.len() as i32);
        for plate in &self.weighed_pressure_plates {
            writer.i32(plate.position.x);
            writer.i32(plate.position.y);
        }
        writer
    }

    fn write_town_manager_section(&self) -> ByteWriter {
        let mut writer = ByteWriter::new();

        writer.i32(self.rooms.len() as i32);
        for room in &self.rooms {
            writer.i32(room.npc);
            writer.i32(room.position.x);
            writer.i32(room.position.y);
        }

        writer
    }

    fn write_bestiary_section(&self) -> ByteWriter {
        let mut writer = ByteWriter::new();

        writer.i32(self.bestiary.kills.len() as i32);
        for (entity, kills) in &self.bestiary.kills {
            writer.string(entity);
            writer.i32(*kills);
        }
        writer.i32(self.bestiary.sightings.len() as i32);
        for s in &self.bestiary.sightings {
            writer.string(s);
        }
        writer.i32(self.bestiary.chats.len() as i32);
        for c in &self.bestiary.chats {
            writer.string(c);
        }

        writer
    }

    fn write_journey_powers_section(&self) -> ByteWriter {
        let mut writer = ByteWriter::new();

        // Write powers collect power IDs from the journey_powers
        let power_ids = [
            0, 8, 9, 10, 12, 13, // Known powers
            // TODO fix collect these ids from journey_powers attribute
        ];

        for &power_id in &power_ids {
            writer.bool(true);
            writer.i16(power_id);
            match power_id {
                0 => writer.bool(self.journey_powers.freeze_time),
                8 => writer.f32(self.journey_powers.time_rate),
                9 => writer.bool(self.journey_powers.freeze_rain),
                10 => writer.bool(self.journey_powers.freeze_wind),
                12 => writer.f32(self.journey_powers.difficulty),
                13 => writer.bool(self.journey_powers.freeze_biome_spread),
                _ => {
                    // For unknown power IDs, we need to skip the value
                    // This shouldn't happen in normal cases, but we need to handle it
                    println!("Warning: Unknown journey power ID {power_id} during writing");
                }
            }
        }
        writer.bool(false); // end of journey powers
        writer
    }

    fn write_footer_section(&self) -> ByteWriter {
        let mut writer = ByteWriter::new();
        writer.bool(true);
        writer.string(&self.world_name);
        writer.i32(self.id);
        writer
    }

    fn read_tile_block(r: &mut ByteReader, tile_frame_important: &[bool]) -> (Tile, usize) {
        let flags1 = r.bits();
        let has_flags2 = flags1[0];
        let flags2 = if has_flags2 { r.bits() } else { vec![false; 8] };
        let has_flags3 = flags2[0];
        let flags3 = if has_flags3 { r.bits() } else { vec![false; 8] };
        let has_flags4 = flags3[0];
        let flags4 = if has_flags4 { r.bits() } else { vec![false; 8] };

        let has_block = flags1[1];
        let has_extended_block_id = flags1[5];
        let is_block_painted = flags3[3];
        let is_block_active = !flags3[2];
        let is_block_echo = flags4[1];
        let is_block_illuminant = flags4[3];

        let has_wall = flags1[2];
        let has_extended_wall_id = flags3[6];
        let is_wall_painted = flags3[4];
        let is_wall_echo = flags4[2];
        let is_wall_illuminant = flags4[4];

        let liquid_type = Self::liquid_type_from_flags(&flags1, &flags3);
        let block_shape = (flags2[6] as u8) * 4 + (flags2[5] as u8) * 2 + (flags2[4] as u8);
        let red_wire = flags2[1];
        let blue_wire = flags2[2];
        let green_wire = flags2[3];
        let yellow_wire = flags3[1];
        let activator_wire = flags3[5];

        // Create tile with default values
        let mut tile = Tile::new();

        // Always set shape and block flags, even for empty tiles
        tile.block_shape = block_shape;
        tile.block_active = is_block_active;
        tile.block_illuminant = is_block_illuminant;
        tile.block_echo = is_block_echo;

        // Parse block
        if has_block {
            let block_id = if has_extended_block_id {
                r.u16()
            } else {
                r.u8() as u16
            };

            let frame = if tile_frame_important
                .get(block_id as usize)
                .copied()
                .unwrap_or(false)
            {
                Some(FrameImportantData::new(r.u16(), r.u16()))
            } else {
                None
            };

            let block_paint = if is_block_painted { Some(r.u8()) } else { None };

            tile.block_id = block_id;
            tile.block_frame = frame;
            tile.block_paint = block_paint;
        }

        // Parse wall
        let wall_id_l = if has_wall { r.u8() } else { 0 };
        let wall_paint = if has_wall && is_wall_painted {
            Some(r.u8())
        } else {
            None
        };

        // Parse liquid
        if liquid_type != LiquidType::NoLiquid {
            tile.liquid_type = liquid_type;
            tile.liquid_amount = r.u8();
        }

        // Parse wall, again
        let wall_id_g = if has_extended_wall_id { r.u8() } else { 0 };

        if has_wall {
            let wall_id = (wall_id_g as u16) * 256 + (wall_id_l as u16);
            tile.wall_id = wall_id;
            tile.wall_paint = wall_paint;
            tile.wall_illuminant = is_wall_illuminant;
            tile.wall_echo = is_wall_echo;
        }

        // Set wiring
        tile.red_wire = red_wire;
        tile.blue_wire = blue_wire;
        tile.green_wire = green_wire;
        tile.yellow_wire = yellow_wire;
        tile.activator_wire = activator_wire;

        // Find RLE Compression multiplier
        let rle_value = (flags1[7] as u8) * 2 + (flags1[6] as u8);
        let multiply_by = match rle_value {
            2 => r.u16() as usize + 1,
            1 => r.u8() as usize + 1,
            0 => 1,
            _ => 1, // i am not sure if it can be anything else
        };

        (tile, multiply_by)
    }

    fn liquid_type_from_flags(flags1: &[bool], flags3: &[bool]) -> LiquidType {
        let flags13 = flags1.get(3).unwrap_or(&false);
        let flags14 = flags1.get(4).unwrap_or(&false);
        let flags37 = flags3.get(7).unwrap_or(&false);

        if *flags37 {
            LiquidType::Shimmer
        } else if *flags13 && *flags14 {
            LiquidType::Honey
        } else if *flags14 {
            LiquidType::Lava
        } else if *flags13 {
            LiquidType::Water
        } else {
            LiquidType::NoLiquid
        }
    }

    fn create_tile_matrix(
        r: &mut ByteReader,
        world_size: (usize, usize),
        tile_frame_important: &[bool],
    ) -> TileMatrix {
        let mut tm = TileMatrix::new((0, 0));
        let (width, height) = world_size;

        for _ in 0..width {
            let mut column = Vec::new();
            let mut column_bytes = Vec::new();
            let start_offset = r.offset();

            while column.len() < height {
                let (tile, multiply_by) = Self::read_tile_block(r, tile_frame_important);
                for _ in 0..multiply_by {
                    column.push(tile.clone());
                }
            }

            let end_offset = r.offset();
            let column_data = r.slice_bytes(start_offset, end_offset);
            column_bytes.extend_from_slice(&column_data);

            tm.add_column(column);
        }
        tm
    }
}
