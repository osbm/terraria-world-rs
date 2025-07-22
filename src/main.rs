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

    // print the very first tile
    println!("{:?}", world.get_corruption_stats());
}
