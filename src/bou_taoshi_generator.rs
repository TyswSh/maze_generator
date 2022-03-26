use std::panic;

use rand::{thread_rng, Rng};

#[derive(Debug)]
struct MazeGrid {
    grid: Vec<Vec<char>>,
    path: char,
    wall: char,
}

impl MazeGrid {
    fn new(h: usize, w: usize, path: char, wall: char) -> MazeGrid {
        let mut init_grid: Vec<Vec<char>> = vec![vec![path; w]; h];
        for i in 0..h {
            for j in 0..w {
                if i == 0 || i == h - 1 || j == 0 || j == w - 1 || (i % 2 == 0 && j % 2 == 0) {
                    init_grid[i][j] = wall;
                }
            }
        }
        MazeGrid {
            grid: init_grid,
            path,
            wall,
        }
    }

    fn set_path(&mut self, y: usize, x: usize) {
        self.grid[y][x] = self.wall;
    }

    fn inspect_grid(&self) {
        for i in &self.grid {
            println!("{}", i.iter().collect::<String>());
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub(crate) struct BouTaoshiMethod {
    width: usize,
    height: usize,
    maze: MazeGrid,
}

impl BouTaoshiMethod {
    pub fn new(height: usize, width: usize, path: char, wall: char) -> BouTaoshiMethod {
        if height < 5 || width < 5 {
            panic!(
                "Invalid value: height = {} or width = {} are 5 or less!!",
                height, width
            );
        }
        BouTaoshiMethod {
            width,
            height,
            maze: MazeGrid::new(height, width, path, wall),
        }
    }

    pub fn generate(&mut self) {
        for i in (0..self.height).step_by(2) {
            for j in (0..self.width).step_by(2) {
                if i == 0 || i == self.height - 1 || j == 0 || j == self.width - 1 {
                    continue;
                }
                // println!("y:{}, x:{}", i, j);
                self.topple(i, j);
            }
        }
    }

    fn topple(&mut self, y: usize, x: usize) {
        let directions: Vec<Directions> = vec![
            Directions::Up,
            Directions::Down,
            Directions::Left,
            Directions::Right,
        ];

        let mut rng = thread_rng();

        // 2行目以降は上を選択しない，壁がある場所は壁を選択しない
        let direction: Directions;
        if y > 2 {
            direction = directions[rng.gen_range(1..4)];
        } else {
            direction = directions[rng.gen_range(0..4)];
        }

        match direction {
            Directions::Up => {
                // y-1
                println!("Up");
                if self.can_topple(y - 1, x) {
                    self.maze.set_path(y - 1, x);
                }
            }
            Directions::Down => {
                // y+1
                println!("Down");
                if self.can_topple(y + 1, x) {
                    self.maze.set_path(y + 1, x);
                }
            }
            Directions::Left => {
                // x-1
                println!("Left");
                if self.can_topple(y, x - 1) {
                    self.maze.set_path(y, x - 1);
                }
            }
            Directions::Right => {
                // x+1
                println!("Right");
                if self.can_topple(y, x + 1) {
                    self.maze.set_path(y, x + 1);
                }
            }
        }
    }

    fn can_topple(&self, y: usize, x: usize) -> bool {
        self.maze.grid[y][x] == self.maze.path
    }

    pub fn inspect_maze(&self) {
        self.maze.inspect_grid();
    }
}
