use terraria_world::world::World;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
struct MazeCell {
    is_carved: bool,
    connections: [bool; 4], // [right, left, down, up]
}

impl Default for MazeCell {
    fn default() -> Self {
        Self {
            is_carved: false,
            connections: [false; 4],
        }
    }
}

#[derive(Debug)]
struct MazeData {
    width: usize,
    height: usize,
    cells: Vec<Vec<MazeCell>>,
    cell_size: usize,
    hall_size: usize,
    wall_size: usize,
    margin: usize,
}

impl MazeData {
    fn new(world_width: usize, world_height: usize, margin: usize) -> Self {
        let cell_size = 8;
        let hall_size = 5;
        let wall_size = 3;

        let width = (world_width - 2 * margin) / cell_size;
        let height = (world_height - 2 * margin) / cell_size;

        println!("Maze grid will be {}x{} cells", width, height);
        println!("Each cell is {}x{} blocks ({} hall + {} wall)", cell_size, cell_size, hall_size, wall_size);

        Self {
            width,
            height,
            cells: vec![vec![MazeCell::default(); height]; width],
            cell_size,
            hall_size,
            wall_size,
            margin,
        }
    }

    fn generate_maze(&mut self) {
        let mut visited = vec![vec![false; self.height]; self.width];
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]; // right, left, down, up

        // Use iterative approach with a stack to avoid stack overflow
        let mut stack = Vec::new();
        let start_x = 1;
        let start_y = 1;

        // Start the maze generation
        visited[start_x][start_y] = true;
        self.cells[start_x][start_y].is_carved = true;
        stack.push((start_x, start_y));

        let mut rng = rand::rng();

        while let Some((x, y)) = stack.last().cloned() {
            let mut dirs = directions.to_vec();
            dirs.shuffle(&mut rng);

            let mut found_unvisited = false;

            for &(dx, dy) in &dirs {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0 || ny < 0 || nx as usize >= self.width || ny as usize >= self.height {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if visited[nx][ny] {
                    continue;
                }

                // Found an unvisited neighbor
                visited[nx][ny] = true;
                self.cells[nx][ny].is_carved = true;

                // Create connection between current cell and next cell
                let direction_index = if dx == 1 { 0 } else if dx == -1 { 1 } else if dy == 1 { 2 } else { 3 };
                let opposite_index = if dx == 1 { 1 } else if dx == -1 { 0 } else if dy == 1 { 3 } else { 2 };

                self.cells[x][y].connections[direction_index] = true;
                self.cells[nx][ny].connections[opposite_index] = true;

                stack.push((nx, ny));
                found_unvisited = true;
                break;
            }

            if !found_unvisited {
                stack.pop();
            }
        }

        // Count carved cells
        let mut carved_count = 0;
        let mut connection_count = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.cells[x][y].is_carved {
                    carved_count += 1;
                }
                for &connected in &self.cells[x][y].connections {
                    if connected {
                        connection_count += 1;
                    }
                }
            }
        }

        println!("Generated maze: {} carved cells, {} connections", carved_count, connection_count / 2);
    }

    fn apply_to_world(&self, world: &mut World) -> usize {
        let mut blocks_carved = 0;

        println!("Applying maze to world...");

        for gx in 0..self.width {
            for gy in 0..self.height {
                let cell = &self.cells[gx][gy];

                if cell.is_carved {
                    // Carve the main hall area
                    let base_x = self.margin + gx * self.cell_size;
                    let base_y = self.margin + gy * self.cell_size;

                    // Carve hall
                    for dx in 0..self.hall_size {
                        for dy in 0..self.hall_size {
                            let wx = base_x + dx;
                            let wy = base_y + dy;

                            if wx < world.world_width as usize && wy < world.world_height as usize {
                                if world.tiles.tiles[wx][wy].block_id != u16::MAX {
                                    blocks_carved += 1;
                                }
                                world.tiles.tiles[wx][wy].block_id = u16::MAX;
                                world.tiles.tiles[wx][wy].block_active = false;
                            }
                        }
                    }

                    // Carve connections to neighboring cells
                    // Right connection
                    if cell.connections[0] {
                        for dy in 0..self.hall_size {
                            for dx in self.hall_size..self.cell_size {
                                let wx = base_x + dx;
                                let wy = base_y + dy;

                                if wx < world.world_width as usize && wy < world.world_height as usize {
                                    if world.tiles.tiles[wx][wy].block_id != u16::MAX {
                                        blocks_carved += 1;
                                    }
                                    world.tiles.tiles[wx][wy].block_id = u16::MAX;
                                    world.tiles.tiles[wx][wy].block_active = false;
                                }
                            }
                        }
                    }

                    // Down connection
                    if cell.connections[2] {
                        for dx in 0..self.hall_size {
                            for dy in self.hall_size..self.cell_size {
                                let wx = base_x + dx;
                                let wy = base_y + dy;

                                if wx < world.world_width as usize && wy < world.world_height as usize {
                                    if world.tiles.tiles[wx][wy].block_id != u16::MAX {
                                        blocks_carved += 1;
                                    }
                                    world.tiles.tiles[wx][wy].block_id = u16::MAX;
                                    world.tiles.tiles[wx][wy].block_active = false;
                                }
                            }
                        }
                    }
                }
            }
        }

        blocks_carved
    }
}

fn main() {
    let mut world = World::new("maze_world", "small", "classic", "corruption");

    println!("World dimensions: {}x{}", world.world_width, world.world_height);

    // Create maze data structure
    let margin = 50;
    let mut maze = MazeData::new(world.world_width as usize, world.world_height as usize, margin);

    // Generate the maze structure
    println!("Generating maze structure...");
    maze.generate_maze();

    // Calculate expected carving statistics before applying
    let total_cells = maze.width * maze.height;
    let carved_cells = maze.cells.iter()
        .flat_map(|row| row.iter())
        .filter(|cell| cell.is_carved)
        .count();

    let blocks_per_carved_cell = maze.hall_size * maze.hall_size;
    let estimated_blocks_carved = carved_cells * blocks_per_carved_cell;

    println!("Maze generation complete:");
    println!("  Total grid cells: {}", total_cells);
    println!("  Carved cells: {} ({:.1}%)", carved_cells, (carved_cells as f64 / total_cells as f64) * 100.0);
    println!("  Estimated blocks to carve: {}", estimated_blocks_carved);

    // Fill the world with solid blocks (stone)
    println!("Filling world with stone blocks...");
    let mut stone_blocks_placed = 0;
    for x in 0..world.world_width as usize {
        for y in 0..world.world_height as usize {
            world.tiles.tiles[x][y].block_id = 1; // 1 = STONE
            world.tiles.tiles[x][y].block_active = true;
            stone_blocks_placed += 1;
        }
    }

    // Apply the maze to the world
    println!("Applying maze to world...");
    let blocks_carved = maze.apply_to_world(&mut world);

    world
        .save_as_wld("maze_world.wld")
        .expect("Failed to save maze world");

    // Calculate and display final statistics
    let total_blocks = (world.world_width * world.world_height) as usize;
    let remaining_stone_blocks = stone_blocks_placed - blocks_carved;
    let air_blocks = blocks_carved;

    let stone_percentage = (remaining_stone_blocks as f64 / total_blocks as f64) * 100.0;
    let air_percentage = (air_blocks as f64 / total_blocks as f64) * 100.0;

    println!("Final maze statistics:");
    println!("  Total blocks in world: {}", total_blocks);
    println!("  Stone blocks placed initially: {}", stone_blocks_placed);
    println!("  Blocks carved (turned to air): {}", blocks_carved);
    println!("  Remaining stone blocks: {}", remaining_stone_blocks);
    println!("  Stone percentage: {:.2}%", stone_percentage);
    println!("  Air percentage: {:.2}%", air_percentage);

    println!("Maze generated! World saved to maze_world.wld");
}
