pub use terraria_world_parser::tile;
use terraria_world_parser::world::World;

fn main() {
    let path = "worlds/small_corruption.wld";
    let world = World::from_file(path).expect("Failed to read world file");
    // println!("World loaded: {:#?}", world);
    println!("Version: {}", world.version());
    println!("Pointers: {:#?}", world.pointers());
    println!("Difficulty: {}", world.difficulty());
}
