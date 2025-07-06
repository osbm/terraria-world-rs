mod world;
mod reader;

use world::World;


fn main() {
    let path = "worlds/small_corruption.wld";
    let world = World::from_file(path);
    println!("World loaded: {:#?}", world);
}