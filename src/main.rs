use terraria_world::world::World;

fn main() {
    use rand::seq::SliceRandom;

    fn carve_maze(
        world: &mut World,
        gx: usize,
        gy: usize,
        visited: &mut Vec<Vec<bool>>,
        depth: usize,
        margin: usize,
        blocks_carved: &mut usize,
    ) {
        if depth > 5000 {
            // Increased depth limit for better coverage
            return;
        }

        let directions = &[(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut rng = rand::rng(); // Use the new rng function
        let mut dirs = directions.to_vec();
        dirs.shuffle(&mut rng);

        visited[gx][gy] = true;

        let cell_size = 8; // Grid cell size (wall + hall)
        let hall_size = 5; // Size of carved hall in each cell
        let wall_size = 3; // Remaining wall thickness

        let base_x = margin + gx * cell_size;
        let base_y = margin + gy * cell_size;

        if base_x + hall_size >= world.world_width as usize
            || base_y + hall_size >= world.world_height as usize
        {
            return;
        }

        // Carve current cell (hall_size x hall_size square) - set to air
        for dx in 0..hall_size {
            for dy in 0..hall_size {
                let wx = base_x + dx;
                let wy = base_y + dy;
                if world.tiles.tiles[wx][wy].block_id != u16::MAX {
                    *blocks_carved += 1;
                }
                world.tiles.tiles[wx][wy].block_id = u16::MAX; // Air
                world.tiles.tiles[wx][wy].block_active = false;
            }
        }

        for &(dx, dy) in &dirs {
            let nx = gx as isize + dx;
            let ny = gy as isize + dy;

            if nx < 0 || ny < 0 || nx as usize >= visited.len() || ny as usize >= visited[0].len() {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if visited[nx][ny] {
                continue;
            }

            // Carve passage between cells - set to air
            // Calculate the passage area between current cell and next cell
            let passage_start_x = if dx > 0 {
                base_x + hall_size
            } else if dx < 0 {
                base_x - wall_size
            } else {
                base_x
            };
            let passage_start_y = if dy > 0 {
                base_y + hall_size
            } else if dy < 0 {
                base_y - wall_size
            } else {
                base_y
            };

            let passage_width = if dx != 0 { wall_size } else { hall_size };
            let passage_height = if dy != 0 { wall_size } else { hall_size };

            for i in 0..passage_width {
                for j in 0..passage_height {
                    let wx = passage_start_x as isize + i as isize;
                    let wy = passage_start_y as isize + j as isize;

                    if wx >= 0
                        && wy >= 0
                        && (wx as usize) < world.world_width as usize
                        && (wy as usize) < world.world_height as usize
                    {
                        if world.tiles.tiles[wx as usize][wy as usize].block_id != u16::MAX {
                            *blocks_carved += 1;
                        }
                        world.tiles.tiles[wx as usize][wy as usize].block_id = u16::MAX; // Air
                        world.tiles.tiles[wx as usize][wy as usize].block_active = false;
                    }
                }
            }

            carve_maze(world, nx, ny, visited, depth + 1, margin, blocks_carved);
        }
    }
    // Usage
    let mut world = World::new("maze_world", "small", "classic", "corruption");

    println!(
        "World dimensions: {}x{}",
        world.world_width, world.world_height
    );

    // Counters for tracking block placement and carving
    let mut stone_blocks_placed = 0;
    let mut blocks_carved = 0;

    // First, fill the world with solid blocks (stone)
    println!("Filling world with stone blocks...");
    for x in 0..world.world_width as usize {
        for y in 0..world.world_height as usize {
            world.tiles.tiles[x][y].block_id = 1; // 1 = STONE
            world.tiles.tiles[x][y].block_active = true;
            stone_blocks_placed += 1;
        }
    }

    let margin = 50;

    // Create visited grid with correct dimensions for grid coordinates
    let grid_width = (world.world_width as usize - 2 * margin) / 8;
    let grid_height = (world.world_height as usize - 2 * margin) / 8;
    let mut visited = vec![vec![false; grid_height]; grid_width];

    println!("Grid dimensions: {}x{} cells", grid_width, grid_height);

    // Start in grid-aligned position within the margin
    let start_x = 1; // Grid coordinate, not world coordinate
    let start_y = 1; // Grid coordinate, not world coordinate

    println!("Starting maze carving...");
    carve_maze(
        &mut world,
        start_x,
        start_y,
        &mut visited,
        0,
        margin,
        &mut blocks_carved,
    );

    // Count how many cells were actually visited
    let mut visited_count = 0;
    for row in &visited {
        for &cell in row {
            if cell {
                visited_count += 1;
            }
        }
    }

    println!(
        "Visited {} out of {} total grid cells ({:.1}%)",
        visited_count,
        grid_width * grid_height,
        (visited_count as f64 / (grid_width * grid_height) as f64) * 100.0
    );

    world
        .save_as_wld("maze_world.wld")
        .expect("Failed to save maze world");

    // Calculate and display statistics
    let total_blocks = (world.world_width * world.world_height) as usize;
    let remaining_stone_blocks = stone_blocks_placed - blocks_carved;
    let air_blocks = blocks_carved;

    let stone_percentage = (remaining_stone_blocks as f64 / total_blocks as f64) * 100.0;
    let air_percentage = (air_blocks as f64 / total_blocks as f64) * 100.0;

    println!("Maze carving statistics:");
    println!("  Total blocks in world: {}", total_blocks);
    println!("  Stone blocks placed initially: {}", stone_blocks_placed);
    println!("  Blocks carved (turned to air): {}", blocks_carved);
    println!("  Remaining stone blocks: {}", remaining_stone_blocks);
    println!("  Stone percentage: {:.2}%", stone_percentage);
    println!("  Air percentage: {:.2}%", air_percentage);

    println!("Maze generated! World saved to maze_world.wld");
}
