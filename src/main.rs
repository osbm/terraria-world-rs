use terraria_world::world::World;

fn main() {

    use rand::seq::SliceRandom;

    fn carve_maze(world: &mut World, x: usize, y: usize, visited: &mut Vec<Vec<bool>>, depth: usize, margin: usize) {
        // Limit recursion depth to prevent stack overflow
        if depth > 1000 {
            return;
        }

        let directions = &[(3, 0), (-3, 0), (0, 3), (0, -3)];
        let mut rng = rand::rng();
        let mut dirs = directions.to_vec();
        dirs.shuffle(&mut rng);

        visited[x][y] = true;

        for (dx, dy) in dirs {
            let nx_isize = x as isize + dx;
            let ny_isize = y as isize + dy;

            // Check bounds before casting to usize
            if nx_isize >= 0 && ny_isize >= 0 {
                let nx = nx_isize as usize;
                let ny = ny_isize as usize;

                // Check that we stay within the margin boundaries
                if nx >= margin
                    && ny >= margin
                    && nx < (world.world_width as usize - margin)
                    && ny < (world.world_height as usize - margin)
                    && !visited[nx][ny]
                {
                // Carve path between current and next
                for i in 0..3 {
                    for j in 0..3 {
                        let cx_isize = x as isize + dx / 2 - 1 + i;
                        let cy_isize = y as isize + dy / 2 - 1 + j;

                        // Check bounds before casting to usize
                        if cx_isize >= 0 && cy_isize >= 0 {
                            let cx = cx_isize as usize;
                            let cy = cy_isize as usize;

                            // Ensure carved path stays within margin boundaries
                            if cx >= margin
                                && cy >= margin
                                && cx < (world.world_width as usize - margin)
                                && cy < (world.world_height as usize - margin)
                            {
                                world.tiles.tiles[cx][cy].block_id = 1;
                            }
                        }
                    }
                }
                carve_maze(world, nx, ny, visited, depth + 1, margin);
                }
            }
        }
    }

    // Usage
    let mut world = World::new("maze_world", "small", "classic", "corruption");

    println!("World dimensions: {}x{}", world.world_width, world.world_height);

    let mut visited = vec![vec![false; world.world_height as usize]; world.world_width as usize];

    let margin = 41;

    // Fill the playable area with air (leaving margins as default blocks)
    for x in margin..(world.world_width as usize - margin) {
        for y in margin..(world.world_height as usize - margin) {
            world.tiles.tiles[x][y].block_id = u16::MAX;
        }
    }

    // Start in grid-aligned position within the margin
    let start_x = margin + 3;
    let start_y = margin + 3;
    carve_maze(&mut world, start_x, start_y, &mut visited, 0, margin);

    world
        .save_as_wld("maze_world.wld")
        .expect("Failed to save maze world");



    println!("Maze generated! Saving world...");
}
