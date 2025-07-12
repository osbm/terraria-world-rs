use crate::reader::ByteReader;
use crate::writer::ByteWriter;

// Module declarations
pub mod tile;
mod pointers;
pub mod coordinates;
pub mod item;
pub mod chest;
pub mod sign;
pub mod entity;
pub mod npc;
pub mod mob;
pub mod tile_entity;
pub mod pressure_plate;
pub mod room;
pub mod bestiary;
pub mod journey_powers;
pub mod error;

use self::tile::{
    Block, BlockType, FrameImportantData, Liquid, LiquidType, RLEEncoding, Tile, TileMatrix, Wall,
    WallType, Wiring,
};
use self::pointers::Pointers;
use serde::{Deserialize, Serialize};

// Import all the moved types from their submodules
use crate::world::coordinates::Coordinates;
use crate::world::item::ItemStack;
use crate::world::chest::Chest;
use crate::world::sign::Sign;
use crate::world::entity::EntityType;
use crate::world::npc::NPC;
use crate::world::mob::Mob;
use crate::world::tile_entity::{TileEntity, TileEntityExtra};
use crate::world::pressure_plate::WeighedPressurePlate;
use crate::world::room::Room;
use crate::world::bestiary::Bestiary;
use crate::world::journey_powers::JourneyPowers;
use crate::world::error::InvalidFooterError;



#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    pub version_integer: i32,
    pub magic: String,
    pub savefile_type: u8,
    pub revision: u32,
    pub is_favorite: u64,
    pub pointer_count: u16,
    pub pointer_vector: Vec<u32>,
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
    pub moondial_is_running: bool,
    pub moondial_cooldown: u8,
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
    pub tile_bytes: Vec<Vec<u8>>, // Each Vec<u8> represents the entire column data
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
        // println!("Pointer count: {}", pointer_count);
        // println!("File offset after reading pointer count: {}", r.offset());
        let mut pointer_vector = vec![];
        for _ in 0..pointer_count {
            pointer_vector.push(r.u32());
        }
        let pointers = Pointers::from_vector(&pointer_vector); // create this only to use it during parsing

        // Print section sizes from pointer table
        println!("=== Section sizes from pointer table ===");
        println!("Section 1 (File Header): {} bytes", pointers.world_header);
        println!(
            "Section 2 (World Header): {} bytes, starting at {}, ending at {}",
            pointers.world_tiles - pointers.world_header,
            pointers.world_header,
            pointers.world_tiles

        );
        println!(
            "Section 3 (Tiles): {} bytes, starting at {}, ending at {}",
            pointers.chests - pointers.world_tiles,
            pointers.world_tiles,
            pointers.chests
        );
        println!(
            "Section 4 (Chests): {} bytes, starting at {}, ending at {}",
            pointers.signs - pointers.chests,
            pointers.chests,
            pointers.signs
        );
        println!(
            "Section 5 (Signs): {} bytes, starting at {}, ending at {}",
            pointers.npcs - pointers.signs,
            pointers.signs,
            pointers.npcs
        );
        println!(
            "Section 6 (NPCs): {} bytes, starting at {}, ending at {}",
            pointers.tile_entities - pointers.npcs,
            pointers.npcs,
            pointers.tile_entities
        );
        println!(
            "Section 7 (Tile Entities): {} bytes, starting at {}, ending at {}",
            pointers.pressure_plates - pointers.tile_entities,
            pointers.tile_entities,
            pointers.pressure_plates
        );
        println!(
            "Section 8 (Pressure Plates): {} bytes, starting at {}, ending at {}",
            pointers.town_manager - pointers.pressure_plates,
            pointers.pressure_plates,
            pointers.town_manager
        );
        println!(
            "Section 9 (Town Manager): {} bytes, starting at {}, ending at {}",
            pointers.bestiary - pointers.town_manager,
            pointers.town_manager,
            pointers.bestiary
        );
        println!(
            "Section 10 (Beastiary): {} bytes, starting at {}, ending at {}",
            pointers.journey_powers - pointers.bestiary,
            pointers.bestiary,
            pointers.journey_powers
        );
        println!(
            "Section 11 (Journey Powers): {} bytes, starting at {}, ending at {}",
            pointers.footer - pointers.journey_powers,
            pointers.journey_powers,
            pointers.footer
        );
        println!("========================================");

        // println!("File offset after reading pointers: {}", r.offset());

        let tile_frame_important_count = r.i16();
        // println!("Reading tile_frame_important: count={}", tile_frame_important_count);
        let tile_frame_important_size = (tile_frame_important_count + 7) / 8;
        let mut tile_frame_important = vec![];
        for _ in 0..tile_frame_important_size {
            let current_bits = r.bits();
            tile_frame_important.extend(current_bits);
        }
        // Truncate to the exact count since we read full bytes but only need specific number of bits
        tile_frame_important.truncate(tile_frame_important_count as usize);

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
        // println!("File offset before date: {}", r.offset());
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
        let moondial_is_running = r.bool();
        let moondial_cooldown = r.u8();
        // tiles
        let (width, height) = (world_width as usize, world_height as usize);
        let mut tile_bytes: Vec<Vec<u8>> = vec![Vec::new(); width]; // Each column will store its entire data
        let tiles = Self::create_tile_matrix(
            &mut r,
            (width, height),
            &tile_frame_important,
            &mut tile_bytes,
        );

        // // Print last 10 bytes of tile data for empty world
        // if world_name == "Blank World - Journey" {
        //     println!("=== Last 10 bytes of tile data (read) ===");
        //     let tile_data_end = r.offset();
        //     println!("tile_data_end: {}", tile_data_end);
        //     println!("pointers.chests: {}", pointers.chests);
        //     if tile_data_end >= 10 {
        //         // Get the last 10 bytes by slicing from the end
        //         let last_10_bytes = r.slice_bytes(tile_data_end - 10, tile_data_end);
        //         println!("{:02X?}", last_10_bytes);
        //     } else {
        //         // If less than 10 bytes, get all available bytes
        //         let all_bytes = r.slice_bytes(0, tile_data_end);
        //         println!("{:02X?}", all_bytes);
        //     }
        //     println!("=== End last 10 bytes of tile data (read) ===");
        // }

        let debug_chest_offset_before  = r.offset();

        // --- CHEST PARSING ---
        let chests_count = r.i16();
        let chests_max_items = r.i16();
        println!("chests_max_items: {}", chests_max_items);
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


        if world_name == "Blank World - Journey" {
            let debug_chest_offset_after = r.offset();
            // println!("File offset after chests: {}", r.offset());
            println!("=== Chests section as hex ===");
            // just the read bytes for chests
            let chests_bytes = r.slice_bytes(debug_chest_offset_before, debug_chest_offset_after);
            for (i, byte) in chests_bytes.iter().enumerate() {
                print!("{:02X} ", byte);
                if (i + 1) % 16 == 0 {
                    println!();
                }
            }
            println!();
            println!("=== End chests section ===");
        }
        println!("File offset after chests: {}", r.offset());


        // --- SIGN PARSING ---
        let debug_signs_offset_before = r.offset();
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
        let debug_signs_offset_after = r.offset();
        if world_name == "Blank World - Journey" {
            println!("=== Signs section as hex ===");
            let signs_bytes = r.slice_bytes(debug_signs_offset_before, debug_signs_offset_after);
            for (i, byte) in signs_bytes.iter().enumerate() {
                print!("{:02X} ", byte);
                if (i + 1) % 16 == 0 {
                    println!();
                }
            }
            println!();
            println!("=== End signs section ===");
        }

        // Parse entities
        let mut npcs = Vec::new();
        let mut mobs = Vec::new();

        // Parse shimmered NPCs
        let shimmered_npcs_count = r.i32();
        // println!("shimmered_npcs_count: {} at offset {}", shimmered_npcs_count, r.offset());
        let mut shimmered_npcs = Vec::with_capacity(shimmered_npcs_count as usize);
        for _i in 0..shimmered_npcs_count {
            let npc_id = r.i32();
            // println!("shimmered_npcs[{}]: {} at offset {}", _i, npc_id, r.offset());
            shimmered_npcs.push(npc_id);
        }

        // Parse NPCs
        let mut _npc_index = 0;
        while r.bool() {
            // println!("NPC {}: start at offset {}", _npc_index, r.offset());
            let npc_type = EntityType::from(r.i32());
            // println!("NPC {}: type = {:?} at offset {}", _npc_index, npc_type, r.offset());
            let npc_name = r.string(None);
            // println!("NPC {}: name = '{}' at offset {}", _npc_index, npc_name, r.offset());
            let npc_position = Coordinates {
                x: r.f32() as i32,
                y: r.f32() as i32,
            };
            // println!("NPC {}: position = {:?} at offset {}", _npc_index, npc_position, r.offset());
            let _is_homeless = r.bool();
            // println!("NPC {}: is_homeless = {} at offset {}", _npc_index, _is_homeless, r.offset());
            let npc_home = Coordinates {
                x: r.i32(),
                y: r.i32(),
            };

            // println!("NPC {}: home = {:?} at offset {}", _npc_index, npc_home, r.offset());

            let npc_flags = r.bits();
            // println!("NPC {}: flags = {:?} at offset {}", _npc_index, npc_flags, r.offset());
            let npc_variation_index = r.i32();
            if !npc_flags[0] {
                let _npc_variation_index = 0i32;
            }
            // println!("NPC {}: variation_index = {} at offset {}", _npc_index, npc_variation_index, r.offset());

            let npc = NPC::new(
                npc_type,
                npc_name,
                npc_position,
                npc_home,
                npc_variation_index,
            );
            npcs.push(npc);
            // println!("NPC {}: end at offset {}", _npc_index, r.offset());
            _npc_index += 1;
        }

        // Parse mobs
        let mut _mob_index = 0;
        while r.bool() {
            // println!("Mob {}: start at offset {}", _mob_index, r.offset());
            let mob_type = EntityType::from(r.i32());
            // println!("Mob {}: type = {:?} at offset {}", _mob_index, mob_type, r.offset());
            let mob_position = Coordinates {
                x: r.f32() as i32,
                y: r.f32() as i32,
            };
            // println!("Mob {}: position = {:?} at offset {}", _mob_index, mob_position, r.offset());
            let mob = Mob::new(mob_type, mob_position);
            mobs.push(mob);
            // println!("Mob {}: end at offset {}", _mob_index, r.offset());
            _mob_index += 1;
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

        // Parse town manager (rooms)
        let rooms_count = r.i32();
        let mut rooms = Vec::with_capacity(rooms_count as usize);
        for _ in 0..rooms_count {
            let npc = EntityType::from(r.i32());
            let position = Coordinates {
                x: r.i32(),
                y: r.i32(),
            };
            rooms.push(Room::new(npc, position));
        }

        // Parse bestiary
        let debug_bestiary_offset_before = r.offset();
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
        let debug_bestiary_offset_after = r.offset();
        if world_name == "Blank World - Journey" {
            println!("=== Bestiary section as hex ===");
            let bestiary_bytes = r.slice_bytes(debug_bestiary_offset_before, debug_bestiary_offset_after);
            for (i, byte) in bestiary_bytes.iter().enumerate() {
                print!("{:02X} ", byte);
                if (i + 1) % 16 == 0 {
                    println!();
                }
            }
            println!();
            println!("=== End Bestiary section ===");
        }

        // Parse journey powers
        let mut journey_powers = JourneyPowers::new();
        while r.bool() {
            let power_id = r.i16();
            // Record the order of power IDs
            journey_powers.power_order.push(power_id);
            match power_id {
                0 => journey_powers.freeze_time = r.bool(),
                8 => journey_powers.time_rate = r.f32(),
                9 => journey_powers.freeze_rain = r.bool(),
                10 => journey_powers.freeze_wind = r.bool(),
                12 => journey_powers.difficulty = r.f32(),
                13 => journey_powers.freeze_biome_spread = r.bool(),
                _ => {
                    println!("Unknown journey power ID: {} please open a issue at github.com/osbm/terraria-world-rs", power_id);
                }
            }
        }

        if world_name == "Blank World - Journey" {
            println!("=== Journey Powers section as hex ===");
            let journey_powers_bytes = r.slice_bytes(
                pointers.journey_powers as usize,
                pointers.footer as usize,
            );
            for (i, byte) in journey_powers_bytes.iter().enumerate() {
                print!("{:02X} ", byte);
                if (i + 1) % 16 == 0 {
                    println!();
                }
            }
            println!();
            println!("=== End Journey Powers section ===");
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

        let world = World { // World vs Self?
            version_integer,
            magic,
            savefile_type,
            revision,
            is_favorite,
            pointer_count,
            pointer_vector,
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
            tile_bytes,
        };

        if world.world_name == "Blank World - Journey" {
            println!("CONSTRUCTOR: chests_max_items = {}", world.chests_max_items);
        }

        Ok(world)
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
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let world = serde_json::from_reader(reader)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(world)
    }

    pub fn save_as_wld(&mut self, path: &str) -> std::io::Result<()> {
        use crate::writer::ByteWriter;
        println!("Saving to {path}...");

        // Create 11 separate buffers for each section
        let mut section_buffers: Vec<ByteWriter> = vec![ByteWriter::new(); 11];

        // Section 1: File header
        let mut header_writer = ByteWriter::new();
        header_writer.i32(self.version_integer);
        header_writer.bytes(self.magic.as_bytes());
        header_writer.u8(self.savefile_type);
        header_writer.u32(self.revision);
        header_writer.u64(self.is_favorite);
        header_writer.u16(self.pointer_count);

        // Write placeholder pointers (will be updated later)
        for _ in 0..self.pointer_count {
            header_writer.u32(0);
        }

        // Section 2: World header
        let world_header_writer = &mut section_buffers[0];
        world_header_writer.string(&self.world_name);
        world_header_writer.string(&self.generator_seed);
        world_header_writer.u64(self.generator_version);
        world_header_writer.uuid(&self.uuid);
        world_header_writer.i32(self.id);

        // Write bounds_vec (left, right, top, bottom)
        for v in &self.bounds_vec {
            world_header_writer.i32(*v);
        }

        // Write world_height, world_width, difficulty_value, flags, created_on, moon_style
        world_header_writer.i32(self.world_height);
        world_header_writer.i32(self.world_width);
        world_header_writer.i32(self.difficulty_value);
        world_header_writer.bool(self.is_drunk_world);
        world_header_writer.bool(self.is_for_the_worthy);
        world_header_writer.bool(self.is_tenth_anniversary);
        world_header_writer.bool(self.is_the_constant);
        world_header_writer.bool(self.is_bee_world);
        world_header_writer.bool(self.is_upside_down);
        world_header_writer.bool(self.is_trap_world);
        world_header_writer.bool(self.is_zenith_world);
        world_header_writer.datetime(&self.created_on);
        world_header_writer.u8(self.moon_style);

        // Write tree_style_seperators, tree_style_properties, moss_style_seperators, moss_style_properties
        for v in &self.tree_style_seperators {
            world_header_writer.i32(*v);
        }
        for v in &self.tree_style_properties {
            world_header_writer.i32(*v);
        }
        for v in &self.moss_style_seperators {
            world_header_writer.i32(*v);
        }
        for v in &self.moss_style_properties {
            world_header_writer.i32(*v);
        }

        // Write background styles
        world_header_writer.i32(self.snow_background_style);
        world_header_writer.i32(self.jungle_background_style);
        world_header_writer.i32(self.hell_background_style);

        // Write spawn point, underground/cavern levels, time, day, moon, events, dungeon, world evil, boss flags, etc.
        world_header_writer.i32(self.spawn_point_x);
        world_header_writer.i32(self.spawn_point_y);
        world_header_writer.f64(self.underground_level);
        world_header_writer.f64(self.cavern_level);
        world_header_writer.f64(self.current_time);
        world_header_writer.bool(self.is_daytime);
        world_header_writer.u32(self.moon_phase);
        world_header_writer.bool(self.blood_moon);
        world_header_writer.bool(self.eclipse);
        world_header_writer.i32(self.dungeon_point_x);
        world_header_writer.i32(self.dungeon_point_y);
        world_header_writer.bool(self.world_evil_type);
        world_header_writer.bool(self.defeated_eye_of_cthulhu);
        world_header_writer.bool(self.defeated_eater_of_worlds);
        world_header_writer.bool(self.defeated_skeletron);
        world_header_writer.bool(self.defeated_queen_bee);
        world_header_writer.bool(self.defeated_the_twins);
        world_header_writer.bool(self.defeated_the_destroyer);
        world_header_writer.bool(self.defeated_skeletron_prime);
        world_header_writer.bool(self.defeated_any_mechanical_boss);
        world_header_writer.bool(self.defeated_plantera);
        world_header_writer.bool(self.defeated_golem);
        world_header_writer.bool(self.defeated_king_slime);
        world_header_writer.bool(self.saved_goblin_tinkerer);
        world_header_writer.bool(self.saved_wizard);
        world_header_writer.bool(self.saved_mechanic);
        world_header_writer.bool(self.defeated_goblin_army);
        world_header_writer.bool(self.defeated_clown);
        world_header_writer.bool(self.defeated_frost_moon);
        world_header_writer.bool(self.defeated_pirate_invasion);
        world_header_writer.bool(self.shadow_orbs_smashed_at_least_once);
        world_header_writer.bool(self.shadow_orbs_spawn_meteorite);
        world_header_writer.u8(self.shadow_orbs_evil_boss_counter);
        world_header_writer.i32(self.altars_smashed);
        world_header_writer.bool(self.is_hardmode);
        world_header_writer.bool(!self.party_is_doomed); // party_is_doomed is inverted
        world_header_writer.i32(self.invasion_delay);
        world_header_writer.i32(self.invasion_size);
        world_header_writer.i32(self.invasion_type);
        world_header_writer.f64(self.invasion_position);
        world_header_writer.f64(self.time_left_slime_rain);
        world_header_writer.u8(self.sundial_cooldown);
        world_header_writer.bool(self.is_rain_active);
        world_header_writer.i32(self.rain_time_left);
        world_header_writer.f32(self.max_rain);
        world_header_writer.i32(self.hardmode_ore_1);
        world_header_writer.i32(self.hardmode_ore_2);
        world_header_writer.i32(self.hardmode_ore_3);
        world_header_writer.i8(self.forest_background);
        world_header_writer.i8(self.corruption_background);
        world_header_writer.i8(self.jungle_background);
        world_header_writer.i8(self.snow_background);
        world_header_writer.i8(self.hallow_background);
        world_header_writer.i8(self.crimson_background);
        world_header_writer.i8(self.desert_background);
        world_header_writer.i8(self.ocean_background);
        world_header_writer.i32(self.cloud_background);
        world_header_writer.i16(self.cloud_number);
        world_header_writer.f32(self.wind_speed);

        // Angler quest completed by
        world_header_writer.i32(self.angler_today_quest_completed_by.len() as i32);
        for name in &self.angler_today_quest_completed_by {
            world_header_writer.string(name);
        }

        // Angler and other NPCs
        world_header_writer.bool(self.saved_angler);
        world_header_writer.i32(self.angler_daily_quest_target);
        world_header_writer.bool(self.saved_stylist);
        world_header_writer.bool(self.saved_tax_collector);
        world_header_writer.bool(self.saved_golfer);
        world_header_writer.i32(self.invasion_size_start);
        world_header_writer.i32(self.cultist_delay);

        // Mob kills
        world_header_writer.i16(self.mob_kills.len() as i16);
        for v in &self.mob_kills {
            world_header_writer.i32(*v);
        }
        world_header_writer.bool(self.sundial_is_running);
        world_header_writer.bool(self.defeated_duke_fishron);
        world_header_writer.bool(self.defeated_martian_madness);
        world_header_writer.bool(self.defeated_lunatic_cultist);
        world_header_writer.bool(self.deteated_moon_lord);
        world_header_writer.bool(self.defeated_pumpking);
        world_header_writer.bool(self.defeated_mourning_wood);
        world_header_writer.bool(self.defeated_ice_queen);
        world_header_writer.bool(self.defeated_santa_nk1);
        world_header_writer.bool(self.defeated_everscream);
        world_header_writer.bool(self.defeated_solar_pillar);
        world_header_writer.bool(self.defeated_vortex_pillar);
        world_header_writer.bool(self.defeated_nebula_pillar);
        world_header_writer.bool(self.defeated_stardust_pillar);
        world_header_writer.bool(self.lunar_events_pillars_present_solar);
        world_header_writer.bool(self.lunar_events_pillars_present_vortex);
        world_header_writer.bool(self.lunar_events_pillars_present_nebula);
        world_header_writer.bool(self.lunar_events_pillars_present_stardust);
        world_header_writer.bool(self.lunar_events_are_active);
        world_header_writer.bool(self.party_center_active);
        world_header_writer.bool(self.party_natural_active);
        world_header_writer.i32(self.party_cooldown);
        world_header_writer.i32(self.partying_npcs.len() as i32);
        for v in &self.partying_npcs {
            world_header_writer.i32(*v);
        }
        world_header_writer.bool(self.is_sandstorm_active);
        world_header_writer.i32(self.sandstorm_time_left);
        world_header_writer.f32(self.sandstorm_severity);
        world_header_writer.f32(self.sandstorm_intended_severity);
        world_header_writer.bool(self.saved_bartender);
        world_header_writer.bool(self.old_ones_army_tier_1);
        world_header_writer.bool(self.old_ones_army_tier_2);
        world_header_writer.bool(self.old_ones_army_tier_3);
        world_header_writer.i8(self.mushroom_background);
        world_header_writer.i8(self.underworld_background);
        world_header_writer.i8(self.forest_background_2);
        world_header_writer.i8(self.forest_background_3);
        world_header_writer.i8(self.forest_background_4);
        world_header_writer.bool(self.combat_book_used);
        world_header_writer.i32(self.lantern_nights_on_cooldown);
        world_header_writer.bool(self.lantern_night_genuine);
        world_header_writer.bool(self.lantern_night_manual);
        world_header_writer.bool(self.next_night_is_lantern_night);
        world_header_writer.i32(self.treetop_variants.len() as i32);
        for v in &self.treetop_variants {
            world_header_writer.i32(*v);
        }
        world_header_writer.bool(self.halloween_today);
        world_header_writer.bool(self.christmas_today);
        world_header_writer.i32(self.ore_1);
        world_header_writer.i32(self.ore_2);
        world_header_writer.i32(self.ore_3);
        world_header_writer.i32(self.ore_4);
        world_header_writer.bool(self.has_cat);
        world_header_writer.bool(self.has_dog);
        world_header_writer.bool(self.has_bunny);
        world_header_writer.bool(self.defeated_empress_of_light);
        world_header_writer.bool(self.defeated_queen_slime);
        world_header_writer.bool(self.defeated_deerclops);
        world_header_writer.bool(self.saved_slime_nerdy);
        world_header_writer.bool(self.saved_merchant);
        world_header_writer.bool(self.saved_demolitionist);
        world_header_writer.bool(self.saved_party_girl);
        world_header_writer.bool(self.saved_dye_trader);
        world_header_writer.bool(self.saved_truffle);
        world_header_writer.bool(self.saved_arms_dealer);
        world_header_writer.bool(self.saved_nurse);
        world_header_writer.bool(self.saved_princess);
        world_header_writer.bool(self.combat_book_2_used);
        world_header_writer.bool(self.peddler_satchel_used);
        world_header_writer.bool(self.saved_slime_cool);
        world_header_writer.bool(self.saved_slime_elder);
        world_header_writer.bool(self.saved_slime_clumsy);
        world_header_writer.bool(self.saved_slime_diva);
        world_header_writer.bool(self.saved_slime_surly);
        world_header_writer.bool(self.saved_slime_mystic);
        world_header_writer.bool(self.saved_slime_squire);
        world_header_writer.bool(self.moondial_is_running);
        world_header_writer.u8(self.moondial_cooldown);

        // Section 3: Tiles
        let tiles_writer = &mut section_buffers[1];

        // a method named write_tiles_section
        self.write_tiles_section(tiles_writer);

        // Print last 10 bytes of tile data for empty world
        if self.world_name == "Blank World - Journey" {
            println!("=== Last 10 bytes of tile data (write) ===");
            let tile_data = tiles_writer.as_slice();
            if tile_data.len() >= 10 {
                let last_10_bytes = &tile_data[tile_data.len() - 10..];
                println!("{:02X?}", last_10_bytes);
            } else {
                println!("{:02X?}", tile_data);
            }
            println!("=== End last 10 bytes of tile data (write) ===");
        }

        // Section 4: Chests
        let chests_writer = &mut section_buffers[2];
        chests_writer.i16(self.chests.len() as i16);
        println!("chests_max_items: {}", self.chests_max_items);
        chests_writer.i16(self.chests_max_items);
        for chest in &self.chests {
            chests_writer.i32(chest.position.x);
            chests_writer.i32(chest.position.y);
            chests_writer.string(&chest.name);
            for item in &chest.contents {
                if let Some(item) = item {
                    chests_writer.i16(item.quantity);
                    chests_writer.i32(item.type_id);
                    chests_writer.u8(item.prefix);
                } else {
                    chests_writer.i16(0);
                }
            }
        }

        // print hex values of chests section but only if the world name is
        if self.world_name == "Blank World - Journey" {
            println!("=== Chests section as hex ===");
            for (i, byte) in chests_writer.as_slice().iter().enumerate() {
                print!("{:02X?} ", byte);
                if (i + 1) % 16 == 0 {
                    println!();
                }
            }
            println!();
            println!("=== End chests section ===");

        }

        // Section 5: Signs
        let signs_writer = &mut section_buffers[3];
        signs_writer.i16(self.signs.len() as i16);
        for sign in &self.signs {
            signs_writer.string(&sign.text);
            signs_writer.i32(sign.position.x);
            signs_writer.i32(sign.position.y);
        }
        if self.world_name == "Blank World - Journey" {
            println!("=== Signs section as hex ===");
            for (i, byte) in signs_writer.as_slice().iter().enumerate() {
                print!("{:02X} ", byte);
                if (i + 1) % 16 == 0 {
                    println!();
                }
            }
            println!();
            println!("=== End signs section ===");
        }

        // Section 6: NPCs and Mobs
        let npcs_writer = &mut section_buffers[4];
        npcs_writer.i32(self.shimmered_npcs.len() as i32);
        for id in &self.shimmered_npcs {
            npcs_writer.i32(*id);
        }
        // Write npcs
        for npc in &self.npcs {
            npcs_writer.bool(true); // presence flag
            npcs_writer.i32(npc.type_.id());
            npcs_writer.string(&npc.name);
            npcs_writer.f32(npc.position.x as f32);
            npcs_writer.f32(npc.position.y as f32);
            npcs_writer.bool(false); // is_homeless (not tracked)
            npcs_writer.i32(npc.home.x);
            npcs_writer.i32(npc.home.y);
            npcs_writer.bits(&[true, false, false, false, false, false, false, false]); // npc_flags (placeholder)
            npcs_writer.i32(npc.variation_index);
        }
        npcs_writer.bool(false); // end of npcs
                                 // Write mobs
        for mob in &self.mobs {
            npcs_writer.bool(true);
            npcs_writer.i32(mob.type_.id());
            npcs_writer.f32(mob.position.x as f32);
            npcs_writer.f32(mob.position.y as f32);
        }
        npcs_writer.bool(false); // end of mobs

        // Section 7: Tile Entities
        let tile_entities_writer = &mut section_buffers[5];
        tile_entities_writer.i32(self.tile_entities.len() as i32);
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
                None => (255u8, &te.extra),
            };
            tile_entities_writer.u8(te_type);
            tile_entities_writer.i32(te.id);
            tile_entities_writer.i16(te.position.x as i16);
            tile_entities_writer.i16(te.position.y as i16);
            match extra {
                Some(crate::world::TileEntityExtra::TargetDummy { npc }) => {
                    tile_entities_writer.i16(*npc);
                }
                Some(crate::world::TileEntityExtra::ItemFrame { item }) => {
                    tile_entities_writer.i16(item.type_id as i16);
                    tile_entities_writer.u8(item.prefix);
                    tile_entities_writer.i16(item.quantity);
                }
                Some(crate::world::TileEntityExtra::LogicSensor {
                    logic_check,
                    enabled,
                }) => {
                    tile_entities_writer.u8(*logic_check);
                    tile_entities_writer.bool(*enabled);
                }
                Some(crate::world::TileEntityExtra::Mannequin { items, dyes }) => {
                    let item_flags: Vec<bool> = items.iter().map(|i| i.is_some()).collect();
                    let dye_flags: Vec<bool> = dyes.iter().map(|i| i.is_some()).collect();
                    tile_entities_writer.bits(&item_flags);
                    tile_entities_writer.bits(&dye_flags);
                    for (_i, item) in items.iter().enumerate() {
                        if let Some(item) = item {
                            tile_entities_writer.i16(item.type_id as i16);
                            tile_entities_writer.u8(item.prefix);
                            tile_entities_writer.i16(item.quantity);
                        }
                    }
                    for (_i, dye) in dyes.iter().enumerate() {
                        if let Some(dye) = dye {
                            tile_entities_writer.i16(dye.type_id as i16);
                            tile_entities_writer.u8(dye.prefix);
                            tile_entities_writer.i16(dye.quantity);
                        }
                    }
                }
                Some(crate::world::TileEntityExtra::WeaponRack { item }) => {
                    tile_entities_writer.i16(item.type_id as i16);
                    tile_entities_writer.u8(item.prefix);
                    tile_entities_writer.i16(item.quantity);
                }
                Some(crate::world::TileEntityExtra::HatRack { items, dyes }) => {
                    let item_flags: Vec<bool> = items
                        .iter()
                        .chain(dyes.iter())
                        .map(|i| i.is_some())
                        .collect();
                    tile_entities_writer.bits(&item_flags);
                    for item in items.iter().chain(dyes.iter()) {
                        if let Some(item) = item {
                            tile_entities_writer.i16(item.type_id as i16);
                            tile_entities_writer.u8(item.prefix);
                            tile_entities_writer.i16(item.quantity);
                        }
                    }
                }
                Some(crate::world::TileEntityExtra::Plate { item }) => {
                    tile_entities_writer.i16(item.type_id as i16);
                    tile_entities_writer.u8(item.prefix);
                    tile_entities_writer.i16(item.quantity);
                }
                Some(crate::world::TileEntityExtra::Pylon) => {}
                None => {}
            }
        }

        // Section 8: Pressure Plates
        let pressure_plates_writer = &mut section_buffers[6];
        pressure_plates_writer.i32(self.weighed_pressure_plates.len() as i32);
        for plate in &self.weighed_pressure_plates {
            pressure_plates_writer.i32(plate.position.x);
            pressure_plates_writer.i32(plate.position.y);
        }

        // Section 9: Town Manager
        let town_manager_writer = &mut section_buffers[7];
        town_manager_writer.i32(self.rooms.len() as i32);
        for room in &self.rooms {
            town_manager_writer.i32(room.npc.id());
            town_manager_writer.i32(room.position.x);
            town_manager_writer.i32(room.position.y);
        }

        // Section 10: Bestiary
        let bestiary_writer = &mut section_buffers[8];
        bestiary_writer.i32(self.bestiary.kills.len() as i32);
        for (entity, kills) in &self.bestiary.kills {
            bestiary_writer.string(entity);
            bestiary_writer.i32(*kills);
        }
        bestiary_writer.i32(self.bestiary.sightings.len() as i32);
        for s in &self.bestiary.sightings {
            bestiary_writer.string(s);
        }
        bestiary_writer.i32(self.bestiary.chats.len() as i32);
        for c in &self.bestiary.chats {
            bestiary_writer.string(c);
        }
        if self.world_name == "Blank World - Journey" {
            println!("=== Bestiary section as hex ===");
            for (i, byte) in bestiary_writer.as_slice().iter().enumerate() {
                print!("{:02X} ", byte);
                if (i + 1) % 16 == 0 {
                    println!();
                }
            }
            println!();
            println!("=== End Bestiary section ===");
        }

        // Section 11: Journey Powers
        let journey_powers_writer = &mut section_buffers[9];
        // Write each power as a pair (id, value) in the exact same order as read
        for &power_id in &self.journey_powers.power_order {
            journey_powers_writer.bool(true);
            journey_powers_writer.i16(power_id);
            match power_id {
                0 => journey_powers_writer.bool(self.journey_powers.freeze_time),
                8 => journey_powers_writer.f32(self.journey_powers.time_rate),
                9 => journey_powers_writer.bool(self.journey_powers.freeze_rain),
                10 => journey_powers_writer.bool(self.journey_powers.freeze_wind),
                12 => journey_powers_writer.f32(self.journey_powers.difficulty),
                13 => journey_powers_writer.bool(self.journey_powers.freeze_biome_spread),
                _ => {
                    // For unknown power IDs, we need to skip the value
                    // This shouldn't happen in normal cases, but we need to handle it
                    println!("Warning: Unknown journey power ID {} during writing", power_id);
                }
            }
        }
        journey_powers_writer.bool(false); // end of journey powers


        if self.world_name == "Blank World - Journey" {
            println!("=== Journey Powers section as hex ===");
            for (i, byte) in journey_powers_writer.as_slice().iter().enumerate() {
                print!("{:02X} ", byte);
                if (i + 1) % 16 == 0 {
                    println!();
                }
            }
            println!();
            println!("=== End Journey Powers section ===");
        }

        // Footer
        let footer_writer = &mut section_buffers[10];
        footer_writer.bool(true);
        footer_writer.string(&self.world_name);
        footer_writer.i32(self.id);

        // Calculate section lengths and update pointers
        let mut current_offset = header_writer.offset() as u32;

        // The header_writer already includes the tile_frame_important data, so we don't need to add it again

        // Update pointer vector with actual offsets
        let mut pointer_vector = Vec::new();

        // Section 0: World Header (starts after the fixed header)
        pointer_vector.push(current_offset);
        current_offset += section_buffers[0].offset() as u32;

        // Section 1: Tiles
        pointer_vector.push(current_offset);
        current_offset += section_buffers[1].offset() as u32;

        // Section 2: Chests
        pointer_vector.push(current_offset);
        current_offset += section_buffers[2].offset() as u32;

        // Section 3: Signs
        pointer_vector.push(current_offset);
        current_offset += section_buffers[3].offset() as u32;

        // Section 4: NPCs
        pointer_vector.push(current_offset);
        current_offset += section_buffers[4].offset() as u32;

        // Section 5: Tile Entities
        pointer_vector.push(current_offset);
        current_offset += section_buffers[5].offset() as u32;

        // Section 6: Pressure Plates
        pointer_vector.push(current_offset);
        current_offset += section_buffers[6].offset() as u32;

        // Section 7: Town Manager
        pointer_vector.push(current_offset);
        current_offset += section_buffers[7].offset() as u32;

        // Section 8: Bestiary
        pointer_vector.push(current_offset);
        current_offset += section_buffers[8].offset() as u32;

        // Section 9: Journey Powers
        pointer_vector.push(current_offset);
        current_offset += section_buffers[9].offset() as u32;

        // Section 10: Footer
        pointer_vector.push(current_offset);
        let _unused_offset = current_offset + section_buffers[10].offset() as u32;

        // Write the complete file
        let mut final_writer = ByteWriter::new();

        // Write header with updated pointers
        final_writer.i32(self.version_integer);
        final_writer.bytes(self.magic.as_bytes());
        final_writer.u8(self.savefile_type);
        final_writer.u32(self.revision);
        final_writer.u64(self.is_favorite);
        final_writer.u16(self.pointer_count);

        // Write updated pointer vector
        // TODO: Revert to placeholder pointers after confirming world serialization/deserialization works correctly
        // Write actual pointer values from world object for debugging section sizes
        for &pointer in &self.pointer_vector {
            final_writer.u32(pointer);
        }

        // Write tile_frame_important count and bits in the file header
        // We need to write the original count, not the actual array length
        // The original count is what determines how many bytes to read
        let original_count = self.tile_frame_important.len() as i16;
        // println!("Writing tile_frame_important: count={}, actual_bits={}",
        //  original_count, self.tile_frame_important.len());
        final_writer.i16(original_count);
        for chunk in self.tile_frame_important.chunks(8) {
            final_writer.bits(chunk);
        }

        // Print section sizes from buffer lengths
        println!("=== Section sizes from buffer lengths ===");
        println!("Section 1 (File Header): {} bytes", header_writer.offset());
        println!(
            "Section 2 (World Header): {} bytes",
            section_buffers[0].offset()
        );
        println!("Section 3 (Tiles): {} bytes", section_buffers[1].offset());
        println!("Section 4 (Chests): {} bytes", section_buffers[2].offset());
        println!("Section 5 (Signs): {} bytes", section_buffers[3].offset());
        println!("Section 6 (NPCs): {} bytes", section_buffers[4].offset());
        println!(
            "Section 7 (Tile Entities): {} bytes",
            section_buffers[5].offset()
        );
        println!(
            "Section 8 (Pressure Plates): {} bytes",
            section_buffers[6].offset()
        );
        println!(
            "Section 9 (Town Manager): {} bytes",
            section_buffers[7].offset()
        );
        println!(
            "Section 10 (Beastiary): {} bytes",
            section_buffers[8].offset()
        );
        println!(
            "Section 11 (Journey Powers): {} bytes",
            section_buffers[9].offset()
        );
        println!(
            "Section 12 (Footer): {} bytes",
            section_buffers[10].offset()
        );
        println!("=========================================");

        // Write all section buffers
        for section_buffer in section_buffers {
            final_writer.bytes(&section_buffer.into_inner());
        }

        // Write buffer to file
        let buffer = final_writer.into_inner();
        std::fs::write(path, buffer)?;


        println!("!");
        println!("!");
        println!("!");
        println!("!");
        println!("!");

        Ok(())
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
        let rle_compression = Self::rle_encoding_from_flags(&flags1);
        let block_shape = 0; // TODO: Implement proper shape parsing
        let wiring = Self::wiring_from_flags(&flags2, &flags3);

        // Parse block
        let block = if has_block {
            let block_type = if has_extended_block_id {
                BlockType::from(r.u16())
            } else {
                BlockType::from(r.u8() as u16)
            };

            let frame = if tile_frame_important
                .get(block_type.id() as usize)
                .copied()
                .unwrap_or(false)
            {
                Some(FrameImportantData::new(r.u16(), r.u16()))
            } else {
                None
            };

            let block_paint = if is_block_painted { Some(r.u8()) } else { None };

            Some(Block::new(
                block_type,
                frame,
                block_paint,
                is_block_active,
                block_shape,
                is_block_illuminant,
                is_block_echo,
            ))
        } else {
            None
        };

        // Parse wall
        let wall_type_l = if has_wall { r.u8() } else { 0 };
        let wall_paint = if has_wall && is_wall_painted {
            Some(r.u8())
        } else {
            None
        };

        // Parse liquid
        let liquid = if liquid_type != LiquidType::NoLiquid {
            Some(Liquid::new(liquid_type, r.u8()))
        } else {
            None
        };

        // Parse wall, again
        let wall_type_g = if has_extended_wall_id { r.u8() } else { 0 };

        let wall = if has_wall {
            let wall_type = WallType::from((wall_type_g as u16) * 256 + (wall_type_l as u16));
            Some(Wall::new(
                wall_type,
                wall_paint,
                is_wall_illuminant,
                is_wall_echo,
            ))
        } else {
            None
        };

        // Find RLE Compression multiplier
        let multiply_by = match rle_compression {
            RLEEncoding::DoubleByte => r.u16() as usize + 1,
            RLEEncoding::SingleByte => r.u8() as usize + 1,
            RLEEncoding::NoCompression => 1,
        };

        // Create tile
        let tile = Tile::new(block, wall, liquid, wiring);
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

    fn rle_encoding_from_flags(flags1: &[bool]) -> RLEEncoding {
        let flags16 = flags1.get(6).unwrap_or(&false);
        let flags17 = flags1.get(7).unwrap_or(&false);
        let value = (*flags17 as u8) * 2 + (*flags16 as u8);
        RLEEncoding::from(value)
    }

    fn wiring_from_flags(flags2: &[bool], flags3: &[bool]) -> Wiring {
        let red = flags2.get(1).unwrap_or(&false);
        let blue = flags2.get(2).unwrap_or(&false);
        let green = flags2.get(3).unwrap_or(&false);
        let yellow = flags3.get(1).unwrap_or(&false);

        Wiring::new(*red, *blue, *green, *yellow)
    }

    fn create_tile_matrix(
        r: &mut ByteReader,
        world_size: (usize, usize),
        tile_frame_important: &[bool],
        tile_bytes: &mut Vec<Vec<u8>>,
    ) -> TileMatrix {
        let mut tm = TileMatrix::new();
        let (width, height) = world_size;

        for x in 0..width {
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
            tile_bytes[x] = column_bytes;
        }
        tm
    }


    fn serialize_tile_data(&self, tile: &Tile) -> Vec<u8> {
        // Prepare headers
        let mut header1 = 0u8;
        let mut header2 = 0u8;
        let mut header3 = 0u8;
        let mut header4 = 0u8;
        let mut data = Vec::new();

        // ... existing logic to set header1, header2, header3, header4, and push tile data to data ...
        // (copy from your current function, but don't push headers yet)

        // tile data
        if let Some(block) = &tile.block {
            if block.is_active && block.type_.id() <= 520 && block.type_.id() != 423 {
                // activate bit[1]
                header1 |= 0b_0000_0010;

                // save tile type as byte or int16
                data.push(block.type_.id() as u8); // low byte
                if block.type_.id() > 255 {
                    // write high byte
                    data.push((block.type_.id() >> 8) as u8);
                    // set header1 bit[5] for int16 tile type
                    header1 |= 0b_0010_0000;
                }

                if let Some(frame) = &block.frame {
                    data.push((frame.x & 0xFF) as u8);
                    data.push(((frame.x & 0xFF00) >> 8) as u8);
                    data.push((frame.y & 0xFF) as u8);
                    data.push(((frame.y & 0xFF00) >> 8) as u8);
                } else if (block.type_.id() as usize) < self.tile_frame_important.len() && self.tile_frame_important[block.type_.id() as usize] {
                    data.push(0); data.push(0); data.push(0); data.push(0);
                }

                if self.version_integer < 269 {
                    if let Some(paint) = block.paint {
                        if paint != 0 || block.is_illuminant {
                            let mut color = paint;
                            if color == 0 && block.is_illuminant { color = 31; }
                            header3 |= 0b_0000_1000;
                            data.push(color);
                        }
                    }
                } else {
                    if let Some(paint) = block.paint {
                        if paint != 0 && paint != 31 {
                            header3 |= 0b_0000_1000;
                            data.push(paint);
                        }
                    }
                }
            }
        }
        if let Some(wall) = &tile.wall {
            if wall.type_.id() != 0 && wall.type_.id() <= 255 {
                header1 |= 0b_0000_0100;
                data.push(wall.type_.id() as u8);
                if self.version_integer < 269 {
                    if let Some(paint) = wall.paint {
                        if paint != 0 || wall.is_illuminant {
                            let mut color = paint;
                            if color == 0 && wall.is_illuminant { color = 31; }
                            header3 |= 0b_0001_0000;
                            data.push(color);
                        }
                    }
                } else {
                    if let Some(paint) = wall.paint {
                        if paint != 0 && paint != 31 {
                            header3 |= 0b_0001_0000;
                            data.push(paint);
                        }
                    }
                }
            }
        }
        if let Some(liquid) = &tile.liquid {
            if liquid.volume != 0 && liquid.type_ != LiquidType::NoLiquid {
                match liquid.type_ {
                    LiquidType::Shimmer if self.version_integer >= 269 => {
                        header3 |= 0b_1000_0000;
                        header1 |= 0b_0000_1000;
                    }
                    LiquidType::Lava => { header1 |= 0b_0001_0000; }
                    LiquidType::Honey => { header1 |= 0b_0001_1000; }
                    _ => { header1 |= 0b_0000_1000; }
                }
                data.push(liquid.volume);
            }
        }
        if tile.wiring.red { header2 |= 0b_0000_0010; }
        if tile.wiring.blue { header2 |= 0b_0000_0100; }
        if tile.wiring.green { header2 |= 0b_0000_1000; }
        if let Some(block) = &tile.block {
            let brick_style = (block.shape << 4) as u8;
            header2 |= brick_style;
        }
        if tile.wiring.yellow { header3 |= 0b_0010_0000; }
        if let Some(wall) = &tile.wall {
            if wall.type_.id() > 255 && self.version_integer >= 222 {
                header3 |= 0b_0100_0000;
                data.push((wall.type_.id() >> 8) as u8);
            }
        }
        if self.version_integer >= 269 {
            if let Some(block) = &tile.block { if block.is_echo { header4 |= 0b_0000_0010; } }
            if let Some(wall) = &tile.wall { if wall.is_echo { header4 |= 0b_0000_0100; } }
            if let Some(block) = &tile.block { if block.is_illuminant || block.paint == Some(31) { header4 |= 0b_0000_1000; } }
            if let Some(wall) = &tile.wall { if wall.is_illuminant || wall.paint == Some(31) { header4 |= 0b_0001_0000; } }
            if header4 != 0 { header3 |= 0b_0000_0001; }
        }
        if header3 != 0 { header2 |= 0b_0000_0001; }
        if header2 != 0 { header1 |= 0b_0000_0001; }

        // Now, push only as many headers as needed
        let mut out = Vec::new();
        out.push(header1);
        if header1 & 0b_0000_0001 != 0 {
            out.push(header2);
            if header2 & 0b_0000_0001 != 0 {
                out.push(header3);
                if self.version_integer >= 269 && header3 & 0b_0000_0001 != 0 {
                    out.push(header4);
                }
            }
        }
        out.extend(data);
        out
    }

    fn write_tiles_section(&mut self, writer: &mut ByteWriter) {
        // The thing is that while we have every Tile in the memory
        // The gamefile format Uses something called RLE Compression
        // Which essentially just means in a column repeat the same tile

        // But we will need to calculate If a tile will need to be repeated before getting it

        for column_idx in 0..self.world_width as usize {
            let mut column_data = Vec::new();
            let mut row_idx = 0;

            while row_idx < self.world_height as usize {
                let current_tile = &self.tiles.tiles[column_idx][row_idx];

                // Calculate RLE (Run Length Encoding) for this tile
                let mut rle = 0;
                let mut next_y = row_idx + 1;
                let mut remaining_y = self.world_height as usize - row_idx - 1;

                // Check how many consecutive identical tiles we have
                while remaining_y > 0 && next_y < self.world_height as usize {
                    let next_tile = &self.tiles.tiles[column_idx][next_y];

                    // Check if tiles are equal (excluding special cases)
                    if self.tiles_equal(current_tile, next_tile) {
                        rle += 1;
                        remaining_y -= 1;
                        next_y += 1;
                    } else {
                        break;
                    }
                }

                // Serialize the tile data first (without RLE)
                let mut tile_data = self.serialize_tile_data(current_tile);

                // Apply RLE compression if needed
                if rle > 0 {
                    // Set RLE encoding bits in header1 (bits 6-7)
                    let header_index = 0; // header1 is always at index 0

                    if rle <= 255 {
                        // set bit[6] of header1 for byte size rle
                        tile_data[header_index] |= 0b_0100_0000; // 64
                        // Append RLE value at the end
                        tile_data.push((rle & 0xFF) as u8);
                    } else {
                        // set bit[7] of header1 for int16 size rle
                        tile_data[header_index] |= 0b_1000_0000; // 128
                        // Append RLE value as u16 at the end
                        tile_data.push((rle & 0xFF) as u8); // low byte
                        tile_data.push(((rle & 0xFF00) >> 8) as u8); // high byte
                    }
                }

                // Add the tile data to the column
                column_data.extend_from_slice(&tile_data);

                // Skip the tiles we've already processed
                row_idx += rle + 1;
            }

            // Store the entire column data in tile_bytes
            if column_idx < self.tile_bytes.len() {
                self.tile_bytes[column_idx] = column_data.clone();
            }

            // Write the column data to the writer
            writer.bytes(&column_data);
        }
    }

    fn tiles_equal(&self, tile1: &Tile, tile2: &Tile) -> bool {
        // Check if two tiles are equal for RLE compression
        // This is a simplified comparison - you might need to adjust based on your needs

        // Compare blocks
        let block_equal = match (&tile1.block, &tile2.block) {
            (Some(b1), Some(b2)) => {
                b1.type_.id() == b2.type_.id() &&
                b1.is_active == b2.is_active &&
                b1.shape == b2.shape &&
                b1.paint == b2.paint &&
                b1.is_illuminant == b2.is_illuminant &&
                b1.is_echo == b2.is_echo &&
                b1.frame == b2.frame
            }
            (None, None) => true,
            _ => false,
        };

        // Compare walls
        let wall_equal = match (&tile1.wall, &tile2.wall) {
            (Some(w1), Some(w2)) => {
                w1.type_.id() == w2.type_.id() &&
                w1.paint == w2.paint &&
                w1.is_illuminant == w2.is_illuminant &&
                w1.is_echo == w2.is_echo
            }
            (None, None) => true,
            _ => false,
        };

        // Compare liquids
        let liquid_equal = match (&tile1.liquid, &tile2.liquid) {
            (Some(l1), Some(l2)) => {
                l1.type_ == l2.type_ && l1.volume == l2.volume
            }
            (None, None) => true,
            _ => false,
        };

        // Compare wiring
        let wiring_equal = tile1.wiring.red == tile2.wiring.red &&
                          tile1.wiring.blue == tile2.wiring.blue &&
                          tile1.wiring.green == tile2.wiring.green &&
                          tile1.wiring.yellow == tile2.wiring.yellow;

        block_equal && wall_equal && liquid_equal && wiring_equal
    }

    /// Get the raw byte data for a specific column (for debugging RLE compression)
    pub fn get_column_data(&self, column_idx: usize) -> Option<&[u8]> {
        self.tile_bytes.get(column_idx).map(|data| data.as_slice())
    }

    /// Get the size of a specific column's data (for debugging RLE compression)
    pub fn get_column_size(&self, column_idx: usize) -> Option<usize> {
        self.tile_bytes.get(column_idx).map(|data| data.len())
    }

    /// Get all column sizes (for debugging RLE compression)
    pub fn get_all_column_sizes(&self) -> Vec<usize> {
        self.tile_bytes.iter().map(|data| data.len()).collect()
    }
}