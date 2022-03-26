use std::panic;

use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug)]
struct MazeGrid {
    grid: Vec<Vec<char>>,
    path: char,
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
        }
    }

    fn inspect_grid(&self) {
        for i in &self.grid {
            println!("{}", i.iter().collect::<String>());
        }
    }
}

#[derive(Debug)]
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
        let y = get_random_position(&self.height);
        let x = get_random_position(&self.width);

        for i in (0..self.height).step_by(2) {
            for j in (0..self.width).step_by(2) {
                if i == 0 || i == self.height || j == 0 || j == self.width {
                    println!("y:{}, x:{}", i, j);
                }
            }
        }
    }

    fn topple(&mut self, y: usize, x: usize) {
        let mut directions: Vec<Directions> = vec![
            Directions::Up,
            Directions::Down,
            Directions::Left,
            Directions::Right,
        ];

        let mut rng = thread_rng();
        directions.shuffle(&mut rng);

        // 3行目以降は上を選択しない，壁がある場所は壁を選択しない
        for direction in directions {}
    }

    pub fn inspect_maze(&self) {
        self.maze.inspect_grid();
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
