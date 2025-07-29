// use std::env;

use terraria_world::world::World;

fn main() {
    // // Get the path from command line arguments
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     eprintln!("Usage: {} <path_to_wld_file>", args[0]);
    //     std::process::exit(1);
    // }
    // let wld_path = &args[1];

    // let world = World::from_file(wld_path).expect("Failed to load world file");

    // Print the world summary instead of full debug output
    // println!("Tile frame: {:?}", world.tile_frame_important.len());
    // sum is
    // let tile_frame_sum: usize = world.tile_frame_important.iter().map(|&x| if x { 1 } else { 0 }).sum();
    // println!("Tile frame sum: {}", tile_frame_sum);

    // // print 16 tile frames as 1 or 0 per line there are 800 or so
    // for (i, frame) in world.tile_frame_important.iter().enumerate() {
    //     if i % 16 == 0 && i != 0 {
    //         println!();
    //     }
    //     print!("{}", if *frame { "1" } else { "0" });
    //     print!(",");
    // }
    // println!();
    // println!("World created on: {}", world.created_on);
    // println!("World NPCs: {:#?}", world.npcs);
    // println!("Weather and Events: {:#?}", world.weather_events);
    // println!("Invasion Data: {:#?}", world.invasions);
    // // journey_powers
    // println!("Journey Powers: {:#?}", world.journey_powers);
    // // rooms
    // // println!("Rooms: {:#?}", world.rooms);
    // // tile_entities
    // // println!("Tile Entities: {:#?}", world.tile_entities);
    // // mobs
    // println!("Mobs: {:#?}", world.mobs);
    // // mob_kills
    // println!("Mob Kills: {:#?}", world.mob_kills.len());
    // cavern_level
    // println!("Cavern Level: {}", world.cavern_level);
    // // underground_level
    // println!("Underground Level: {}", world.underground_level);
    // println!("Generator version: {}", world.generator_version);
    // println!("Dungeon Point X: {}", world.dungeon_point_x);
    // println!("Dungeon Point Y: {}", world.dungeon_point_y);
    // // angler_daily_quest_target
    // println!("Angler Daily Quest Target: {}", world.angler_daily_quest_target);
    // println world world size
    // println!("World Name: {}", world.world_name);
    // println!("World Size: {}x{}", world.world_width, world.world_height);
    // println!("World tile size: {}x{}", world.tiles.tiles.len(), world.tiles.tiles[0].len());

    // use std::env;

    let mut world2 = World::new("example_world", "large", "classic", "corruption");

    for x in 0..world2.world_width as usize {
        for y in 0..world2.world_height as usize {
            world2.tiles.tiles[x][y].wall_id = 4u16;
            if y == world2.world_height as usize / 2 {
                world2.tiles.tiles[x][y].block_id = 1u16;
                world2.tiles.tiles[x][y].block_illuminant = true;
            }
        }

    }
    println!("Spawn Point: ({}, {})", world2.spawn_point_x, world2.spawn_point_y);
    world2.spawn_point_x = world2.world_width / 2;
    world2.spawn_point_y = world2.world_height / 2 - 1;
    // world2.bestiary = world.bestiary.clone();
    // world2.tile_frame_important = world.tile_frame_important.clone();
    // world2.weather_events = world.weather_events.clone();
    // world2.environment = world.environment.clone();
    // world2.invasions = world.invasions.clone();
    // world2.journey_powers = world.journey_powers.clone();
    // world2.tiles = world.tiles.clone();
    // wor

    println!("World2 Name: {}", world2.world_name);
    println!("World2 Size: {}x{}", world2.world_width, world2.world_height);
    println!("World2 tile size: {}x{}", world2.tiles.tiles.len(), world2.tiles.tiles[0].len());

    world2
        .save_as_wld("example_world2.wld")
        .expect("Failed to save world");
}
