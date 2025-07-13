use terraria_world_parser::world::World;
use terraria_world_parser::world::enums::LiquidType;

fn main() -> std::io::Result<()> {
    let test_worlds_dir = std::env::var("TEST_WORLDS_DIR")
        .expect("TEST_WORLDS_DIR environment variable not set.");

    let world_path = format!("{}/almostemptyworld.wld", test_worlds_dir);
    println!("Loading world: {}", world_path);

    let mut world = World::from_file(&world_path)?;
    println!("World name: {}", world.world_name);
    println!("World size: {}x{}", world.world_width, world.world_height);

    // Print some tile information
    let mut non_empty_tiles = 0;
    for x in 0..world.world_width as usize {
        for y in 0..world.world_height as usize {
            if let Some(tile) = world.tiles.get_tile(x, y) {
                if tile.block_type.is_some() || tile.wall_type.is_some() || (tile.liquid_type != LiquidType::NoLiquid && tile.liquid_amount > 0) {
                    non_empty_tiles += 1;
                    if non_empty_tiles <= 10 {
                        println!("Tile at ({}, {}): {:?}", x, y, tile);
                    }
                }
            }
        }
    }


    Ok(())
}
