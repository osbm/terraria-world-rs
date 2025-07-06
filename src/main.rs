mod reader;
mod world;

use world::World;

fn main() {
    let path = "worlds/small_corruption.wld";
    let world = World::from_file(path).expect("Failed to read world file");
    println!("World loaded: {:#?}", world);
    println!("Version: {}", world.version());
    println!("Pointers: {:#?}", world.pointers());
    println!("Difficulty: {}", world.difficulty());
}
