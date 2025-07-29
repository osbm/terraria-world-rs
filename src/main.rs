use std::env;

use terraria_world::world::World;

fn main() {
    // Get the path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_wld_file>", args[0]);
        std::process::exit(1);
    }
    let wld_path = &args[1];

    let world = World::from_file(wld_path).expect("Failed to load world file");

    // Print the world summary instead of full debug output
    println!("Tile frame: {:?}", world.tile_frame_important);
    println!("World created on: {}", world.created_on);
    println!("World NPCs: {:#?}", world.npcs);
    println!("Weather and Events: {:#?}", world.weather_events);
    println!("Invasion Data: {:#?}", world.invasions);
    // journey_powers
    println!("Journey Powers: {:#?}", world.journey_powers);
    // rooms
    println!("Rooms: {:#?}", world.rooms);
    // tile_entities
    println!("Tile Entities: {:#?}", world.tile_entities);
    // mobs
    println!("Mobs: {:#?}", world.mobs);
    // mob_kills
    println!("Mob Kills: {:#?}", world.mob_kills.len());
    // cavern_level
    println!("Cavern Level: {}", world.cavern_level);
    // underground_level
    println!("Underground Level: {}", world.underground_level);

    // use std::env;

    let mut world2 = World::new("example_world", "large", "classic");

    // set all the tiles walls to WOOD

    for x in 0..world2.world_width as usize {
        for y in 0..world2.world_height as usize {
            world2.tiles.tiles[x][y].wall_id = 4u16;
            if y == world2.world_height as usize / 2 {
                world2.tiles.tiles[x][y].block_id = 1u16;
            }
        }

    }


    world2
        .save_as_wld("example_world2.wld")
        .expect("Failed to save world");
}
