use std::ptr::NonNull;

use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct MazeGrid {
    grid: Vec<Vec<char>>,
    path: char,
    wall: char,
}

impl MazeGrid {
    fn fill_wall_new(h: usize, w: usize, path: char, wall: char) -> MazeGrid {
        MazeGrid {
            grid: (0..h).map(|_| vec![wall; w]).collect(),
            path: path,
            wall: wall,
        }
    }

    fn set_path(&mut self, y: usize, x: usize, path: char) {
        self.grid[y][x] = path;
    }
}

#[derive(Debug)]
pub(crate) struct DiggerMethod {
    width: usize,
    height: usize,
    maze: MazeGrid,
}

impl DiggerMethod {
    pub fn new(width: usize, height: usize, path: char, wall: char) -> DiggerMethod {
        DiggerMethod {
            width,
            height,
            maze: MazeGrid::fill_wall_new(height, width, path, wall),
        }
    }

    pub fn generate(&mut self) {
        let y = get_random_position(&self.height);
        let x = get_random_position(&self.width);
        self.digger(y, x);
    }

    fn digger(&mut self, y: usize, x: usize) {
        let mut directions: Vec<Directions> = vec![
            Directions::Up,
            Directions::Down,
            Directions::Left,
            Directions::Right,
        ];

        self.maze.set_path(y, x, self.maze.path);

        // get random directions
        let mut rng = thread_rng();
        directions.shuffle(&mut rng);

        // let mut updated_maze: Vec<Vec<char>> = maze.to_vec();
        for direction in directions {
            match direction {
                Directions::Up => {
                    // y-1, y-2
                    // println!("Up");
                    if let Some(steped_y) = y.checked_sub(2) {
                        if self.can_dig(steped_y, x) {
                            self.maze.set_path(y - 1, x, self.maze.path);
                            self.digger(steped_y, x);
                        }
                    }
                }
                Directions::Down => {
                    // y+1, y+2
                    // println!("Down");
                    if self.can_dig(y + 2, x) {
                        self.maze.set_path(y + 1, x, self.maze.path);
                        self.digger(y + 2, x);
                    }
                }
                Directions::Left => {
                    // x-1, x-2
                    // println!("Left");
                    if let Some(steped_x) = x.checked_sub(2) {
                        if self.can_dig(y, steped_x) {
                            self.maze.set_path(y, x - 1, self.maze.path);
                            self.digger(y, steped_x);
                        }
                    }
                }
                Directions::Right => {
                    // x+1, x+2
                    // println!("Right");
                    if self.can_dig(y, x + 2) {
                        self.maze.set_path(y, x + 1, self.maze.path);
                        self.digger(y, x + 2);
                    }
                }
            }
            // self.inspect_maze();
        }
    }

    fn can_dig(&self, dy: usize, dx: usize) -> bool {
        if dy >= self.height {
            return false;
        }

        if dx >= self.width {
            return false;
        }

        if self.maze.grid[dy][dx] == self.maze.path {
            return false;
        }
        true
    }

    pub fn get_maze_grid(self) -> Vec<Vec<char>> {
        self.maze.grid
    }

    pub fn inspect_maze(&self) {
        for i in &self.maze.grid {
            println!("{}", i.into_iter().collect::<String>());
        }
    }
}

fn get_random_position(length: &usize) -> usize {
    let mut rng = thread_rng();
    let pos: usize = {
        if length % 2 == 0 {
            loop {
                let n: usize = rng.gen_range(0..*length);
                if n % 2 == 0 {
                    break n;
                }
            }
        } else {
            loop {
                let n = rng.gen_range(0..*length) as usize;
                if n % 2 != 0 {
                    break n;
                }
            }
        }
    };
    pos
}
