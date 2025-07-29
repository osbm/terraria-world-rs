use rand::Rng;
use terraria_world::world::World;

#[derive(Clone, Copy, PartialEq)]
enum CellType {
    Wall,
    Path,
}

struct MazeGenerator {
    width: usize,
    height: usize,
    maze: Vec<Vec<CellType>>,
}

impl MazeGenerator {
    fn new(width: usize, height: usize) -> Self {
        // Initialize maze with all walls
        let maze = vec![vec![CellType::Wall; height]; width];
        Self { width, height, maze }
    }

    fn generate(&mut self) {
        let mut rng = rand::rng();
        let mut stack = Vec::new();

        // Start at top-left corner (make it odd coordinates for proper maze generation)
        let start_x = 1;
        let start_y = 1;

        self.maze[start_x][start_y] = CellType::Path;
        stack.push((start_x, start_y));

        while let Some((current_x, current_y)) = stack.last().copied() {
            let neighbors = self.get_unvisited_neighbors(current_x, current_y);

            if neighbors.is_empty() {
                stack.pop();
            } else {
                // Choose random neighbor
                let &(next_x, next_y) = neighbors.choose(&mut rng).unwrap();

                // Remove wall between current and next cell
                let wall_x = (current_x + next_x) / 2;
                let wall_y = (current_y + next_y) / 2;

                self.maze[wall_x][wall_y] = CellType::Path;
                self.maze[next_x][next_y] = CellType::Path;

                stack.push((next_x, next_y));
            }
        }

        // Ensure entrance and exit are clear
        self.maze[1][1] = CellType::Path;
        if self.width >= 3 && self.height >= 3 {
            self.maze[self.width - 2][self.height - 2] = CellType::Path;
        }
    }

    fn get_unvisited_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let directions = [(0, 2), (2, 0), (0, -2i32), (-2i32, 0)];

        for (dx, dy) in directions.iter() {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x > 0 && new_y > 0 &&
               (new_x as usize) < self.width - 1 &&
               (new_y as usize) < self.height - 1 &&
               self.maze[new_x as usize][new_y as usize] == CellType::Wall {
                neighbors.push((new_x as usize, new_y as usize));
            }
        }

        neighbors
    }
}

trait SliceRandom<T> {
    fn choose<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<&T>;
}

impl<T> SliceRandom<T> for [T] {
    fn choose<R: Rng + ?Sized>(&self, rng: &mut R) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(&self[rng.random_range(0..self.len())])
        }
    }
}

fn main() {
    let mut world = World::new("maze_world", "large", "classic", "corruption");

    // Calculate maze dimensions based on world size
    // We want cells to be 8 blocks wide (5 path + 3 wall)
    let cell_size = 8;
    let maze_width = (world.world_width as usize / cell_size) | 1; // Make odd
    let maze_height = (world.world_height as usize / cell_size) | 1; // Make odd

    println!("Generating maze of size {}x{}", maze_width, maze_height);

    // Generate the maze
    let mut maze_gen = MazeGenerator::new(maze_width, maze_height);
    maze_gen.generate();

    // Apply maze to world
    for world_x in 0..world.world_width as usize {
        for world_y in 0..world.world_height as usize {
            // Convert world coordinates to maze coordinates
            let maze_x = world_x / cell_size;
            let maze_y = world_y / cell_size;

            // Default to wall
            let mut is_wall = true;

            if maze_x < maze_width && maze_y < maze_height {
                match maze_gen.maze[maze_x][maze_y] {
                    CellType::Path => {
                        // For path cells, create 5-block wide passages
                        let offset_x = world_x % cell_size;
                        let offset_y = world_y % cell_size;

                        // Center 5 blocks are path, outer 3 are walls
                        if offset_x >= 1 && offset_x <= 6 && offset_y >= 1 && offset_y <= 6 {
                            is_wall = false;
                        }
                    },
                    CellType::Wall => {
                        is_wall = true;
                    }
                }
            }

            if is_wall {
                // Wall blocks - use stone
                world.tiles.tiles[world_x][world_y].block_id = 1u16; // Stone
                world.tiles.tiles[world_x][world_y].wall_id = 4u16;  // Stone wall
            } else {
                // Path blocks - clear air
                world.tiles.tiles[world_x][world_y].block_id = u16::MAX; // Air
                world.tiles.tiles[world_x][world_y].wall_id = 0u16;  // No wall
            }
        }
    }

    // Add some lighting torches periodically
    for world_x in (10..world.world_width as usize).step_by(20) {
        for world_y in (10..world.world_height as usize).step_by(20) {
            if world.tiles.tiles[world_x][world_y].block_id == 0 {
                world.tiles.tiles[world_x][world_y].block_id = 4u16; // Torch
                world.tiles.tiles[world_x][world_y].block_illuminant = true;
            }
        }
    }

    println!("Maze generated! Saving world...");
    world
        .save_as_wld("maze_world.wld")
        .expect("Failed to save maze world");

    println!("Maze world saved as 'maze_world.wld'");
}
