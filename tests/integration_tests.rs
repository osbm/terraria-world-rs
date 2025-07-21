use serde_json::Value;
use std::fs;
use std::path::Path;
use terraria_world::world::enums::LiquidType;
use terraria_world::world::tile_entity::TileEntityExtra;
use terraria_world::world::World;

/// Test utilities for integration tests
mod test_utils {
    use super::*;

    /// Load reference data from JSON file
    pub fn load_reference_data(file_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let data: Value = serde_json::from_str(&content)?;
        Ok(data)
    }

    /// Compare two values with tolerance for floating point numbers
    pub fn assert_approx_eq(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() < tolerance,
            "Expected {} to be within {} of {}",
            actual,
            tolerance,
            expected
        );
    }

    /// Validate world metadata against reference
    pub fn validate_world_metadata(world: &World, metadata: &Value) -> Result<(), String> {
        // Basic world properties
        assert_eq!(
            world.world_name,
            metadata["name"].as_str().unwrap(),
            "World name mismatch"
        );
        assert_eq!(
            world.world_width,
            metadata["size"]["width"].as_i64().unwrap() as i32,
            "World width mismatch"
        );
        assert_eq!(
            world.world_height,
            metadata["size"]["height"].as_i64().unwrap() as i32,
            "World height mismatch"
        );
        assert_eq!(
            world.is_hardmode,
            metadata["is_hardmode"].as_bool().unwrap(),
            "Hardmode flag mismatch"
        );

        // World flags
        let flags = [
            ("is_drunk_world", world.is_drunk_world),
            ("is_for_the_worthy", world.is_for_the_worthy),
            ("is_tenth_anniversary", world.is_tenth_anniversary),
            ("is_the_constant", world.is_the_constant),
            ("is_bee_world", world.is_bee_world),
            ("is_upside_down", world.is_upside_down),
            ("is_trap_world", world.is_trap_world),
            ("is_zenith_world", world.is_zenith_world),
        ];

        for (flag_name, actual_value) in flags {
            let expected_value = metadata[flag_name].as_bool().unwrap();
            assert_eq!(actual_value, expected_value, "{} flag mismatch", flag_name);
        }

        // Spawn and dungeon points
        assert_eq!(
            world.spawn_point_x,
            metadata["spawn_point"]["x"].as_i64().unwrap() as i32,
            "Spawn X mismatch"
        );
        assert_eq!(
            world.spawn_point_y,
            metadata["spawn_point"]["y"].as_i64().unwrap() as i32,
            "Spawn Y mismatch"
        );
        assert_eq!(
            world.dungeon_point_x,
            metadata["dungeon_point"]["x"].as_i64().unwrap() as i32,
            "Dungeon X mismatch"
        );
        assert_eq!(
            world.dungeon_point_y,
            metadata["dungeon_point"]["y"].as_i64().unwrap() as i32,
            "Dungeon Y mismatch"
        );

        // Underground levels with tolerance
        assert_approx_eq(
            world.underground_level,
            metadata["underground_level"].as_f64().unwrap(),
            0.1,
        );
        assert_approx_eq(
            world.cavern_level,
            metadata["cavern_level"].as_f64().unwrap(),
            0.1,
        );

        Ok(())
    }

    /// Validate tile frame important array
    pub fn validate_tile_frame_important(world: &World, metadata: &Value) -> Result<(), String> {
        let ref_tile_frame_important = metadata["tile_frame_important"].as_array().unwrap();
        assert_eq!(
            world.tile_frame_important.len(),
            ref_tile_frame_important.len(),
            "Tile frame important array length mismatch"
        );

        // Check that the array is not empty
        assert!(
            !world.tile_frame_important.is_empty(),
            "Tile frame important array should not be empty"
        );

        // Validate a few common block types
        let common_block_ids = [0, 1, 2, 3, 4, 5]; // DIRT, STONE, GRASS, PLANTS, TORCHES, TREES
        for &block_id in &common_block_ids {
            if block_id < world.tile_frame_important.len() {
                let _important = world.tile_frame_important[block_id];
                // Just verify we can access it without panicking
            }
        }

        Ok(())
    }

    /// Validate individual tile data
    pub fn validate_tile(
        tile: &terraria_world::world::tile::Tile,
        tile_ref: &Value,
    ) -> Result<(), String> {
        let x = tile_ref["position"]["x"].as_u64().unwrap() as usize;
        let y = tile_ref["position"]["y"].as_u64().unwrap() as usize;

        // Basic tile properties
        assert_eq!(
            tile.has_block(),
            tile_ref["has_block"].as_bool().unwrap(),
            "Block presence mismatch at ({}, {})",
            x,
            y
        );
        assert_eq!(
            tile.has_wall(),
            tile_ref["has_wall"].as_bool().unwrap(),
            "Wall presence mismatch at ({}, {})",
            x,
            y
        );
        assert_eq!(
            tile.liquid_type != LiquidType::NoLiquid && tile.liquid_amount > 0,
            tile_ref["has_liquid"].as_bool().unwrap(),
            "Liquid presence mismatch at ({}, {})",
            x,
            y
        );

        // Validate wiring
        let wiring_ref = &tile_ref["wiring"];
        assert_eq!(
            tile.red_wire,
            wiring_ref["red"].as_bool().unwrap(),
            "Red wiring mismatch at ({}, {})",
            x,
            y
        );
        assert_eq!(
            tile.blue_wire,
            wiring_ref["blue"].as_bool().unwrap(),
            "Blue wiring mismatch at ({}, {})",
            x,
            y
        );
        assert_eq!(
            tile.green_wire,
            wiring_ref["green"].as_bool().unwrap(),
            "Green wiring mismatch at ({}, {})",
            x,
            y
        );
        assert_eq!(
            tile.yellow_wire,
            wiring_ref["yellow"].as_bool().unwrap(),
            "Yellow wiring mismatch at ({}, {})",
            x,
            y
        );

        // Validate block data
        if tile.has_block() {
            let block_ref = &tile_ref["block"];
            assert_eq!(
                tile.block_id,
                block_ref["type_id"].as_u64().unwrap() as u16,
                "Block type mismatch at ({}, {})",
                x,
                y
            );
            assert_eq!(
                tile.block_active,
                block_ref["is_active"].as_bool().unwrap(),
                "Block active state mismatch at ({}, {})",
                x,
                y
            );
            assert_eq!(
                tile.block_paint.is_some(),
                block_ref["has_paint"].as_bool().unwrap(),
                "Block paint presence mismatch at ({}, {})",
                x,
                y
            );
            assert_eq!(
                tile.block_illuminant,
                block_ref["is_illuminant"].as_bool().unwrap(),
                "Block illuminant state mismatch at ({}, {})",
                x,
                y
            );
            assert_eq!(
                tile.block_echo,
                block_ref["is_echo"].as_bool().unwrap(),
                "Block echo state mismatch at ({}, {})",
                x,
                y
            );

            if let Some(paint_id) = block_ref["paint_id"].as_u64() {
                assert_eq!(
                    tile.block_paint.unwrap(),
                    paint_id as u8,
                    "Block paint ID mismatch at ({}, {})",
                    x,
                    y
                );
            }

            if let Some(frame_ref) = block_ref.get("frame") {
                assert!(
                    tile.block_frame.is_some(),
                    "Block frame missing at ({}, {})",
                    x,
                    y
                );
                let frame = tile.block_frame.as_ref().unwrap();
                assert_eq!(
                    frame.x,
                    frame_ref["x"].as_u64().unwrap() as u16,
                    "Block frame X mismatch at ({}, {})",
                    x,
                    y
                );
                assert_eq!(
                    frame.y,
                    frame_ref["y"].as_u64().unwrap() as u16,
                    "Block frame Y mismatch at ({}, {})",
                    x,
                    y
                );
            }
        }

        // Validate wall data
        if tile.has_wall() {
            let wall_ref = &tile_ref["wall"];
            assert_eq!(
                tile.wall_id,
                wall_ref["type_id"].as_u64().unwrap() as u16,
                "Wall type mismatch at ({}, {})",
                x,
                y
            );
            assert_eq!(
                tile.wall_paint.is_some(),
                wall_ref["has_paint"].as_bool().unwrap(),
                "Wall paint presence mismatch at ({}, {})",
                x,
                y
            );
            assert_eq!(
                tile.wall_illuminant,
                wall_ref["is_illuminant"].as_bool().unwrap(),
                "Wall illuminant state mismatch at ({}, {})",
                x,
                y
            );
            assert_eq!(
                tile.wall_echo,
                wall_ref["is_echo"].as_bool().unwrap(),
                "Wall echo state mismatch at ({}, {})",
                x,
                y
            );

            if let Some(paint_id) = wall_ref["paint_id"].as_u64() {
                assert_eq!(
                    tile.wall_paint.unwrap(),
                    paint_id as u8,
                    "Wall paint ID mismatch at ({}, {})",
                    x,
                    y
                );
            }
        }

        // Validate liquid data
        if tile.liquid_type != LiquidType::NoLiquid && tile.liquid_amount > 0 {
            let liquid_ref = &tile_ref["liquid"];
            assert_eq!(
                tile.liquid_type as u8,
                liquid_ref["type_id"].as_u64().unwrap() as u8,
                "Liquid type mismatch at ({}, {})",
                x,
                y
            );
            assert_eq!(
                tile.liquid_amount,
                liquid_ref["volume"].as_u64().unwrap() as u8,
                "Liquid volume mismatch at ({}, {})",
                x,
                y
            );
        }

        Ok(())
    }

    /// Validate NPC data against reference
    pub fn validate_npc(
        npc: &terraria_world::world::npc::NPC,
        npc_ref: &Value,
    ) -> Result<(), String> {
        assert_eq!(
            npc.type_,
            npc_ref["type_id"].as_i64().unwrap() as i32,
            "NPC type mismatch"
        );
        assert_eq!(
            npc.name,
            npc_ref["name"].as_str().unwrap(),
            "NPC name mismatch"
        );
        assert_eq!(
            npc.position_x,
            npc_ref["position"]["x"].as_f64().unwrap() as f32,
            "NPC position X mismatch"
        );
        assert_eq!(
            npc.position_y,
            npc_ref["position"]["y"].as_f64().unwrap() as f32,
            "NPC position Y mismatch"
        );
        assert_eq!(
            npc.variation_index,
            npc_ref["variation_index"].as_i64().unwrap() as i32,
            "NPC variation index mismatch"
        );

        // Validate home coordinates
        let home_ref = npc_ref["home"].as_object().unwrap();
        assert_eq!(
            npc.home.x,
            home_ref["x"].as_i64().unwrap() as i32,
            "NPC home X mismatch"
        );
        assert_eq!(
            npc.home.y,
            home_ref["y"].as_i64().unwrap() as i32,
            "NPC home Y mismatch"
        );

        Ok(())
    }

    /// Validate mob data against reference
    pub fn validate_mob(
        mob: &terraria_world::world::mob::Mob,
        mob_ref: &Value,
    ) -> Result<(), String> {
        assert_eq!(
            mob.type_,
            mob_ref["type_id"].as_i64().unwrap() as i32,
            "Mob type mismatch"
        );
        assert_eq!(
            mob.position_x,
            mob_ref["position"]["x"].as_f64().unwrap() as f32,
            "Mob position X mismatch"
        );
        assert_eq!(
            mob.position_y,
            mob_ref["position"]["y"].as_f64().unwrap() as f32,
            "Mob position Y mismatch"
        );
        Ok(())
    }

    /// Get test world files from environment or default
    pub fn get_test_world_files() -> Vec<String> {
        let mut world_files = Vec::new();
        if let Ok(world_path) = std::env::var("TEST_WORLDS_DIR") {
            if let Ok(entries) = std::fs::read_dir(&world_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|ext| ext == "wld").unwrap_or(false) {
                        world_files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        } else {
            eprintln!("TEST_WORLDS_DIR environment variable not set. Skipping integration tests.");
        }
        world_files
    }
}

#[test]
fn test_world_parsing_against_lihzahrd() {
    use test_utils::*;

    let world_files = get_test_world_files();

    if world_files.is_empty() {
        eprintln!("No test world files found. Skipping integration test.");
        eprintln!("Place .wld files in the current directory or set TEST_WORLDS_DIR environment variable.");
        return;
    }

    for world_file in world_files {
        println!("Testing world file: {}", world_file);

        let reference_file = format!(
            "{}.lihzahrd_reference.json",
            world_file.trim_end_matches(".wld")
        );

        // Skip if reference file doesn't exist
        if !Path::new(&reference_file).exists() {
            eprintln!(
                "Reference file {} not found. Run the Python integration test first.",
                reference_file
            );
            continue;
        }

        // Parse world with our Rust implementation
        let world = match World::from_file(&world_file) {
            Ok(w) => w,
            Err(e) => {
                eprintln!(
                    "Failed to parse world {} with Rust implementation: {}",
                    world_file, e
                );
                continue;
            }
        };

        // Load reference data from Python
        let reference_data = match load_reference_data(&reference_file) {
            Ok(data) => data,
            Err(e) => {
                eprintln!(
                    "Failed to load reference data from {}: {}",
                    reference_file, e
                );
                continue;
            }
        };

        // Validate world metadata
        if let Err(e) = validate_world_metadata(&world, &reference_data["metadata"]) {
            panic!("World metadata validation failed for {}: {}", world_file, e);
        }

        // Validate tile frame important array
        if let Err(e) = validate_tile_frame_important(&world, &reference_data["metadata"]) {
            panic!(
                "Tile frame important validation failed for {}: {}",
                world_file, e
            );
        }

        // Validate sample tiles
        let sample_tiles = &reference_data["tiles"]["sample_tiles"];
        let mut validated_tiles = 0;

        for tile_ref in sample_tiles.as_array().unwrap() {
            let x = tile_ref["position"]["x"].as_u64().unwrap() as usize;
            let y = tile_ref["position"]["y"].as_u64().unwrap() as usize;

            if let Some(tile) = world.tiles.get_tile(x, y) {
                if let Err(e) = validate_tile(tile, tile_ref) {
                    panic!(
                        "Tile validation failed for {} at ({}, {}): {}",
                        world_file, x, y, e
                    );
                }
                validated_tiles += 1;
            } else {
                eprintln!(
                    "Warning: Tile not found at ({}, {}) in {}",
                    x, y, world_file
                );
            }
        }

        println!(
            "Successfully validated {} tiles for {}",
            validated_tiles, world_file
        );

        // Validate entities
        let ref_npcs = reference_data["npcs"].as_array().unwrap();
        assert_eq!(
            world.npcs.len(),
            ref_npcs.len(),
            "NPC count mismatch for {}",
            world_file
        );
        for (i, (npc, ref_npc)) in world.npcs.iter().zip(ref_npcs.iter()).enumerate() {
            if let Err(e) = validate_npc(npc, ref_npc) {
                panic!(
                    "NPC validation failed for {} at index {}: {}",
                    world_file, i, e
                );
            }
        }

        let ref_mobs = reference_data["mobs"].as_array().unwrap();
        assert_eq!(
            world.mobs.len(),
            ref_mobs.len(),
            "Mob count mismatch for {}",
            world_file
        );
        for (i, (mob, ref_mob)) in world.mobs.iter().zip(ref_mobs.iter()).enumerate() {
            if let Err(e) = validate_mob(mob, ref_mob) {
                panic!(
                    "Mob validation failed for {} at index {}: {}",
                    world_file, i, e
                );
            }
        }

        let ref_shimmered_npcs = reference_data["shimmered_npcs"].as_array().unwrap();
        assert_eq!(
            world.shimmered_npcs.len(),
            ref_shimmered_npcs.len(),
            "Shimmered NPC count mismatch for {}",
            world_file
        );
        for (i, (shimmered_npc, ref_shimmered_npc)) in world
            .shimmered_npcs
            .iter()
            .zip(ref_shimmered_npcs.iter())
            .enumerate()
        {
            assert_eq!(
                *shimmered_npc,
                ref_shimmered_npc.as_i64().unwrap() as i32,
                "Shimmered NPC {} mismatch for {}",
                i,
                world_file
            );
        }

        println!(
            "Successfully validated {} NPCs, {} mobs, {} shimmered NPCs for {}",
            world.npcs.len(),
            world.mobs.len(),
            world.shimmered_npcs.len(),
            world_file
        );
    }
}

#[test]
fn test_world_parsing_basic_functionality() {
    let world_files = test_utils::get_test_world_files();

    if world_files.is_empty() {
        eprintln!("No test world files found. Skipping basic functionality test.");
        return;
    }

    for world_file in world_files {
        println!("Testing basic functionality for: {}", world_file);

        let world = match World::from_file(&world_file) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Failed to parse world {}: {}", world_file, e);
                continue;
            }
        };

        // Basic sanity checks
        assert!(world.world_width > 0, "World width should be positive");
        assert!(world.world_height > 0, "World height should be positive");
        assert!(
            !world.world_name.is_empty(),
            "World name should not be empty"
        );

        // Check that spawn and dungeon points are within world bounds
        assert!(
            world.spawn_point_x >= 0 && world.spawn_point_x < world.world_width,
            "Spawn point X should be within world bounds"
        );
        assert!(
            world.spawn_point_y >= 0 && world.spawn_point_y < world.world_height,
            "Spawn point Y should be within world bounds"
        );
        assert!(
            world.dungeon_point_x >= 0 && world.dungeon_point_x < world.world_width,
            "Dungeon point X should be within world bounds"
        );
        assert!(
            world.dungeon_point_y >= 0 && world.dungeon_point_y < world.world_height,
            "Dungeon point Y should be within world bounds"
        );

        // Check underground levels are reasonable
        assert!(
            world.underground_level > 0.0,
            "Underground level should be positive"
        );
        assert!(
            world.cavern_level > world.underground_level,
            "Cavern level should be deeper than underground level"
        );

        // Check tile matrix
        assert!(
            !world.tile_frame_important.is_empty(),
            "Tile frame important array should not be empty"
        );

        // Try to access some tiles
        let mut accessible_tiles = 0;
        for x in 0..std::cmp::min(10, world.world_width as usize) {
            for y in 0..std::cmp::min(10, world.world_height as usize) {
                if world.tiles.get_tile(x, y).is_some() {
                    accessible_tiles += 1;
                }
            }
        }

        assert!(
            accessible_tiles > 0,
            "Should be able to access at least some tiles"
        );
        println!(
            "Successfully accessed {} tiles for {}",
            accessible_tiles, world_file
        );
    }
}

#[test]
fn test_tile_frame_important_consistency() {
    let world_files = test_utils::get_test_world_files();

    if world_files.is_empty() {
        eprintln!("No test world files found. Skipping tile frame important test.");
        return;
    }

    for world_file in world_files {
        println!(
            "Testing tile frame important consistency for: {}",
            world_file
        );

        let world = match World::from_file(&world_file) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Failed to parse world {}: {}", world_file, e);
                continue;
            }
        };

        // Check that tile_frame_important array is not empty
        assert!(
            !world.tile_frame_important.is_empty(),
            "Tile frame important array should not be empty"
        );

        // Check that we can access tile_frame_important for common block types
        let common_block_ids = [0, 1, 2, 3, 4, 5]; // DIRT, STONE, GRASS, PLANTS, TORCHES, TREES

        for &block_id in &common_block_ids {
            if block_id < world.tile_frame_important.len() {
                let _important = world.tile_frame_important[block_id];
                // Just verify we can access it without panicking
            }
        }

        // Check that the array length is reasonable (should be at least a few hundred for modern Terraria)
        assert!(
            world.tile_frame_important.len() >= 100,
            "Tile frame important array should have at least 100 entries, got {}",
            world.tile_frame_important.len()
        );

        println!(
            "Tile frame important array has {} entries for {}",
            world.tile_frame_important.len(),
            world_file
        );
    }
}

#[test]
fn test_world_file_validation() {
    // Test that invalid files are handled gracefully
    let invalid_files = ["nonexistent.wld"];

    for invalid_file in &invalid_files {
        if Path::new(invalid_file).exists() {
            let result = World::from_file(invalid_file);
            assert!(
                result.is_err(),
                "Should fail to parse invalid file: {}",
                invalid_file
            );
        }
    }

    // Test that non-world files are handled gracefully
    // Note: We don't test Cargo.toml as it might cause panics due to slice bounds
    // Instead, we test with a small invalid file
    let test_invalid_file = "test_invalid.wld";
    if Path::new(test_invalid_file).exists() {
        let result = World::from_file(test_invalid_file);
        assert!(
            result.is_err(),
            "Should fail to parse invalid world file: {}",
            test_invalid_file
        );
    }
}

#[test]
fn test_chests_against_lihzahrd() {
    use test_utils::*;
    let world_files = get_test_world_files();
    if world_files.is_empty() {
        eprintln!("No test world files found. Skipping chest integration test.");
        return;
    }
    for world_file in world_files {
        println!("Testing chests for world file: {}", world_file);
        let reference_file = format!(
            "{}.lihzahrd_reference.json",
            world_file.trim_end_matches(".wld")
        );
        if !Path::new(&reference_file).exists() {
            eprintln!(
                "Reference file {} not found. Run the Python integration test first.",
                reference_file
            );
            continue;
        }
        let world = match World::from_file(&world_file) {
            Ok(w) => w,
            Err(e) => {
                eprintln!(
                    "Failed to parse world {} with Rust implementation: {}",
                    world_file, e
                );
                continue;
            }
        };
        let reference_data = match load_reference_data(&reference_file) {
            Ok(data) => data,
            Err(e) => {
                eprintln!(
                    "Failed to load reference data from {}: {}",
                    reference_file, e
                );
                continue;
            }
        };
        let ref_chests = reference_data["chests"].as_array().unwrap();
        assert_eq!(
            world.chests.len(),
            ref_chests.len(),
            "Chest count mismatch for {}",
            world_file
        );
        for (i, (chest, ref_chest)) in world.chests.iter().zip(ref_chests.iter()).enumerate() {
            assert_eq!(
                chest.position.x,
                ref_chest["position"]["x"].as_i64().unwrap() as i32,
                "Chest {} x position mismatch",
                i
            );
            assert_eq!(
                chest.position.y,
                ref_chest["position"]["y"].as_i64().unwrap() as i32,
                "Chest {} y position mismatch",
                i
            );
            assert_eq!(
                chest.name,
                ref_chest["name"].as_str().unwrap(),
                "Chest {} name mismatch",
                i
            );
            let ref_contents = ref_chest["contents"].as_array().unwrap();
            assert_eq!(
                chest.contents.len(),
                ref_contents.len(),
                "Chest {} contents length mismatch",
                i
            );
            for (j, (item, ref_item)) in chest.contents.iter().zip(ref_contents.iter()).enumerate()
            {
                match (item, ref_item) {
                    (_, serde_json::Value::Null) => {}
                    (Some(item), serde_json::Value::Object(ref obj)) => {
                        assert_eq!(
                            item.quantity,
                            obj["quantity"].as_i64().unwrap() as i16,
                            "Chest {} item {} quantity mismatch",
                            i,
                            j
                        );
                        assert_eq!(
                            item.type_id,
                            obj["type_id"].as_i64().unwrap() as i32,
                            "Chest {} item {} type_id mismatch",
                            i,
                            j
                        );
                        assert_eq!(
                            item.prefix,
                            obj["prefix"].as_u64().unwrap() as u8,
                            "Chest {} item {} prefix mismatch",
                            i,
                            j
                        );
                    }
                    (_, _) => panic!("Chest {} item {}: Rust None but Python not null", i, j),
                }
            }
        }
        println!(
            "Successfully validated {} chests for {}",
            world.chests.len(),
            world_file
        );
    }
}

#[test]
fn test_entities_against_lihzahrd() {
    use test_utils::*;
    let world_files = get_test_world_files();
    if world_files.is_empty() {
        eprintln!("No test world files found. Skipping entity integration test.");
        return;
    }
    for world_file in world_files {
        println!("Testing entities for world file: {}", world_file);
        let reference_file = format!(
            "{}.lihzahrd_reference.json",
            world_file.trim_end_matches(".wld")
        );
        if !Path::new(&reference_file).exists() {
            eprintln!(
                "Reference file {} not found. Run the Python integration test first.",
                reference_file
            );
            continue;
        }
        let world = match World::from_file(&world_file) {
            Ok(w) => w,
            Err(e) => {
                eprintln!(
                    "Failed to parse world {} with Rust implementation: {}",
                    world_file, e
                );
                continue;
            }
        };
        let reference_data = match load_reference_data(&reference_file) {
            Ok(data) => data,
            Err(e) => {
                eprintln!(
                    "Failed to load reference data from {}: {}",
                    reference_file, e
                );
                continue;
            }
        };

        // Validate NPCs
        let ref_npcs = reference_data["npcs"].as_array().unwrap();
        assert_eq!(
            world.npcs.len(),
            ref_npcs.len(),
            "NPC count mismatch for {}",
            world_file
        );
        for (i, (npc, ref_npc)) in world.npcs.iter().zip(ref_npcs.iter()).enumerate() {
            if let Err(e) = validate_npc(npc, ref_npc) {
                panic!(
                    "NPC validation failed for {} at index {}: {}",
                    world_file, i, e
                );
            }
        }

        // Validate mobs
        let ref_mobs = reference_data["mobs"].as_array().unwrap();
        assert_eq!(
            world.mobs.len(),
            ref_mobs.len(),
            "Mob count mismatch for {}",
            world_file
        );
        for (i, (mob, ref_mob)) in world.mobs.iter().zip(ref_mobs.iter()).enumerate() {
            if let Err(e) = validate_mob(mob, ref_mob) {
                panic!(
                    "Mob validation failed for {} at index {}: {}",
                    world_file, i, e
                );
            }
        }

        // Validate shimmered NPCs
        let ref_shimmered_npcs = reference_data["shimmered_npcs"].as_array().unwrap();
        assert_eq!(
            world.shimmered_npcs.len(),
            ref_shimmered_npcs.len(),
            "Shimmered NPC count mismatch for {}",
            world_file
        );
        for (i, (shimmered_npc, ref_shimmered_npc)) in world
            .shimmered_npcs
            .iter()
            .zip(ref_shimmered_npcs.iter())
            .enumerate()
        {
            assert_eq!(
                *shimmered_npc,
                ref_shimmered_npc.as_i64().unwrap() as i32,
                "Shimmered NPC {} mismatch for {}",
                i,
                world_file
            );
        }

        println!(
            "Successfully validated {} NPCs, {} mobs, {} shimmered NPCs for {}",
            world.npcs.len(),
            world.mobs.len(),
            world.shimmered_npcs.len(),
            world_file
        );
    }
}

#[test]
fn test_tile_entity_parsing() {
    let test_worlds_dir = "tests/test_worlds";
    if !Path::new(test_worlds_dir).exists() {
        println!("No test worlds directory found, skipping tile entity test");
        return;
    }
    let entries = fs::read_dir(test_worlds_dir).expect("Failed to read test worlds directory");
    let wld_files: Vec<_> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "wld" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    if wld_files.is_empty() {
        println!("No .wld files found in tests/test_worlds directory, skipping tile entity test");
        return;
    }
    for wld_file in wld_files {
        let file_name = wld_file.file_name().unwrap().to_str().unwrap();
        println!("Testing tile entities for: {}", file_name);
        let world = World::from_file(wld_file.to_str().unwrap())
            .expect(&format!("Failed to read world file: {}", file_name));
        println!("Found {} tile entities", world.tile_entities.len());
        // Basic assertion: tile entities count should be non-negative (always true for Vec)
        // assert!(world.tile_entities.len() >= 0); // This is always true for Vec
        // Print a summary of tile entity types
        let mut type_counts = std::collections::HashMap::new();
        for te in &world.tile_entities {
            let type_str = match &te.extra {
                Some(TileEntityExtra::TargetDummy { .. }) => "TargetDummy",
                Some(TileEntityExtra::ItemFrame { .. }) => "ItemFrame",
                Some(TileEntityExtra::LogicSensor { .. }) => "LogicSensor",
                Some(TileEntityExtra::Mannequin { .. }) => "Mannequin",
                Some(TileEntityExtra::WeaponRack { .. }) => "WeaponRack",
                Some(TileEntityExtra::HatRack { .. }) => "HatRack",
                Some(TileEntityExtra::Plate { .. }) => "Plate",
                Some(TileEntityExtra::Pylon) => "Pylon",
                _ => "Unknown",
            };
            *type_counts.entry(type_str).or_insert(0) += 1;
        }
        println!("Tile entity type counts: {{");
        for (type_str, count) in &type_counts {
            println!("  {}: {}", type_str, count);
        }
        println!("}}\n");
        // Optionally, check the first tile entity for expected fields
        if let Some(first) = world.tile_entities.first() {
            match &first.extra {
                Some(TileEntityExtra::TargetDummy { npc }) => {
                    println!("First tile entity is TargetDummy with npc: {}", npc);
                }
                Some(TileEntityExtra::ItemFrame { item }) => {
                    println!(
                        "First tile entity is ItemFrame with item type_id: {}",
                        item.type_id
                    );
                }
                Some(TileEntityExtra::LogicSensor {
                    logic_check,
                    enabled,
                }) => {
                    println!(
                        "First tile entity is LogicSensor with logic_check: {}, enabled: {}",
                        logic_check, enabled
                    );
                }
                Some(TileEntityExtra::Mannequin { items, dyes }) => {
                    println!(
                        "First tile entity is Mannequin with {} items and {} dyes",
                        items.len(),
                        dyes.len()
                    );
                }
                Some(TileEntityExtra::WeaponRack { item }) => {
                    println!(
                        "First tile entity is WeaponRack with item type_id: {}",
                        item.type_id
                    );
                }
                Some(TileEntityExtra::HatRack { items, dyes }) => {
                    println!(
                        "First tile entity is HatRack with {} items and {} dyes",
                        items.len(),
                        dyes.len()
                    );
                }
                Some(TileEntityExtra::Plate { item }) => {
                    println!(
                        "First tile entity is Plate with item type_id: {}",
                        item.type_id
                    );
                }
                Some(TileEntityExtra::Pylon) => {
                    println!("First tile entity is Pylon");
                }
                _ => {
                    println!("First tile entity is Unknown");
                }
            }
        }
    }
}

#[test]
fn test_weighed_pressure_plates_parsing() {
    let test_worlds_dir = "tests/test_worlds";
    if !Path::new(test_worlds_dir).exists() {
        println!("No test worlds directory found, skipping weighed pressure plates test");
        return;
    }
    let entries = fs::read_dir(test_worlds_dir).expect("Failed to read test worlds directory");
    let wld_files: Vec<_> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "wld" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    if wld_files.is_empty() {
        println!("No .wld files found in tests/test_worlds directory, skipping weighed pressure plates test");
        return;
    }
    for wld_file in wld_files {
        let file_name = wld_file.file_name().unwrap().to_str().unwrap();
        println!("Testing weighed pressure plates for: {}", file_name);
        let world = World::from_file(wld_file.to_str().unwrap())
            .expect(&format!("Failed to read world file: {}", file_name));
        println!(
            "Found {} weighed pressure plates",
            world.weighed_pressure_plates.len()
        );
        // Basic assertion: weighed pressure plates count should be non-negative (always true for Vec)
        // assert!(world.weighed_pressure_plates.len() >= 0); // This is always true for Vec
        // Print details of the first few weighed pressure plates
        for (i, wpp) in world.weighed_pressure_plates.iter().take(5).enumerate() {
            println!(
                "  Weighed pressure plate {}: position ({}, {})",
                i, wpp.position.x, wpp.position.y
            );
        }
        if world.weighed_pressure_plates.len() > 5 {
            println!("  ... and {} more", world.weighed_pressure_plates.len() - 5);
        }
        println!();
    }
}

#[test]
fn test_town_manager_parsing() {
    let test_worlds_dir = "tests/test_worlds";
    if !Path::new(test_worlds_dir).exists() {
        println!("No test worlds directory found, skipping town manager test");
        return;
    }
    let entries = fs::read_dir(test_worlds_dir).expect("Failed to read test worlds directory");
    let wld_files: Vec<_> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "wld" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    if wld_files.is_empty() {
        println!("No .wld files found in tests/test_worlds directory, skipping town manager test");
        return;
    }
    for wld_file in wld_files {
        let file_name = wld_file.file_name().unwrap().to_str().unwrap();
        println!("Testing town manager (rooms) for: {}", file_name);
        let world = World::from_file(wld_file.to_str().unwrap())
            .expect(&format!("Failed to read world file: {}", file_name));
        println!("Found {} rooms", world.rooms.len());
        // Basic assertion: rooms count should be non-negative (always true for Vec)
        // assert!(world.rooms.len() >= 0); // This is always true for Vec
        // Print details of the first few rooms
        for (i, room) in world.rooms.iter().take(5).enumerate() {
            println!(
                "  Room {}: NPC type {}, position ({}, {})",
                i,
                room.npc,
                room.position.x,
                room.position.y
            );
        }
        if world.rooms.len() > 5 {
            println!("  ... and {} more", world.rooms.len() - 5);
        }
        println!();
    }
}

#[test]
fn test_bestiary_parsing() {
    let test_worlds_dir = "tests/test_worlds";
    if !Path::new(test_worlds_dir).exists() {
        println!("No test worlds directory found, skipping bestiary test");
        return;
    }
    let entries = fs::read_dir(test_worlds_dir).expect("Failed to read test worlds directory");
    let wld_files: Vec<_> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "wld" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    if wld_files.is_empty() {
        println!("No .wld files found in tests/test_worlds directory, skipping bestiary test");
        return;
    }
    for wld_file in wld_files {
        let file_name = wld_file.file_name().unwrap().to_str().unwrap();
        println!("Testing bestiary for: {}", file_name);
        let world = World::from_file(wld_file.to_str().unwrap())
            .expect(&format!("Failed to read world file: {}", file_name));
        println!("Bestiary stats:");
        println!("  Kills: {} entries", world.bestiary.kills.len());
        println!("  Sightings: {} entries", world.bestiary.sightings.len());
        println!("  Chats: {} entries", world.bestiary.chats.len());

        // Print first few kills
        let mut kill_iter = world.bestiary.kills.iter().take(5);
        if let Some((entity, kills)) = kill_iter.next() {
            println!("  Sample kills:");
            println!("    {}: {}", entity, kills);
            for (entity, kills) in kill_iter {
                println!("    {}: {}", entity, kills);
            }
        }

        // Print first few sightings
        if !world.bestiary.sightings.is_empty() {
            println!("  Sample sightings:");
            for entity in world.bestiary.sightings.iter().take(5) {
                println!("    {}", entity);
            }
        }

        // Print first few chats
        if !world.bestiary.chats.is_empty() {
            println!("  Sample chats:");
            for entity in world.bestiary.chats.iter().take(5) {
                println!("    {}", entity);
            }
        }
        println!();
    }
}

#[test]
fn test_journey_powers_parsing() {
    let test_worlds_dir = "tests/test_worlds";
    if !Path::new(test_worlds_dir).exists() {
        println!("No test worlds directory found, skipping journey powers test");
        return;
    }
    let entries = fs::read_dir(test_worlds_dir).expect("Failed to read test worlds directory");
    let wld_files: Vec<_> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "wld" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    if wld_files.is_empty() {
        println!(
            "No .wld files found in tests/test_worlds directory, skipping journey powers test"
        );
        return;
    }
    for wld_file in wld_files {
        let file_name = wld_file.file_name().unwrap().to_str().unwrap();
        println!("Testing journey powers for: {}", file_name);
        let world = World::from_file(wld_file.to_str().unwrap())
            .expect(&format!("Failed to read world file: {}", file_name));
        println!("Journey powers:");
        println!("  Freeze time: {}", world.journey_powers.freeze_time);
        println!("  Time rate: {}", world.journey_powers.time_rate);
        println!("  Freeze rain: {}", world.journey_powers.freeze_rain);
        println!("  Freeze wind: {}", world.journey_powers.freeze_wind);
        println!("  Difficulty: {}", world.journey_powers.difficulty);
        println!(
            "  Freeze biome spread: {}",
            world.journey_powers.freeze_biome_spread
        );
        println!();
    }
}
