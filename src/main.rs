use terraria_world_parser::world::World;

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
                if tile.block.is_some() || tile.wall.is_some() || tile.liquid.is_some() {
                    non_empty_tiles += 1;
                    if non_empty_tiles <= 10 {
                        println!("Tile at ({}, {}): {:?}", x, y, tile);
                    }
                }
            }
        }
    }
    println!("Total non-empty tiles: {}", non_empty_tiles);
    
    // Print column sizes
    println!("Column sizes:");
    for (i, size) in world.get_all_column_sizes().iter().enumerate() {
        if *size > 0 {
            println!("Column {}: {} bytes", i, size);
        }
    }
    
    Ok(())
}
