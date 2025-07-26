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
    println!("World created on: {}", world.created_on);
    // println!("World Environment: {:#?}", world.environment);
    // println!("Weather and Events: {:#?}", world.weather_events);
    // println!("Invasion Data: {:#?}", world.invasions);
    // use std::env;

    let mut world2 = World::new("example_world", "large", "classic");

    world2
        .save_as_wld("example_world2.wld")
        .expect("Failed to save world");
}
