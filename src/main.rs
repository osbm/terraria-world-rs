use terraria_world_parser::world::World;

fn main() {
    let world_name = "empty-world.wld";
    let test_worlds_dir = std::env::var("TEST_WORLDS_DIR")
        .expect("TEST_WORLDS_DIR environment variable not set. Please provide the test worlds directory as a flake input.");
    println!("Using test worlds directory: {}", test_worlds_dir);
    let world = World::from_file(&format!("{}/{}", test_worlds_dir, world_name)).expect("Failed to read world file");
    // println!("World loaded: {:#?}", world);
    println!("Beastiary: {:#?}", world.bestiary);
    println!("Beastiary kills: {:#?}", world.bestiary.kills);
    println!("Journey Powers: {:#?}", world.journey_powers);
    println!("Chests: {:#?}", world.chests);

    
}
