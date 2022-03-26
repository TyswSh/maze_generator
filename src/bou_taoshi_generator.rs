use std::panic;

use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug)]
struct MazeGrid {
    grid: Vec<Vec<char>>,
    path: char,
}

impl MazeGrid {
    fn new(h: usize, w: usize, path: char, wall: char) -> MazeGrid {
        let mut gird: Vec<Vec<char>> = Vec::new();
        for i in 0..h {
            for j in 0..w {
                if i == 0 || i == h - 1 || j == 0 || j == w - 1 || (i % 2 != 0 && j % 2 != 0) {
                    grid[i][j] = wall;
                } else {
                    grid[i][j] = path;
                }
            }
        }
        MazeGrid { grid, path }
    }

    fn inspect_grid(&self) {
        self.grid.iter().for_each(|x| pritln!("{:?}", x));
    }
}

#[derive(Debug)]
pub(crate) struct BouTaoshiMethod {
    width: usize,
    heiht: usize,
    maze: MazeGrid,
}

impl BouTaoshiMethod {
    pub fn new(width: usize, height: usize, path: char, wall: char) -> BouTaoshiMethod {
        BouTaoshiMethod {
            width,
            height,
            maze: MazeGrid::new(height, width, path, wall),
        }
    }

    pub fn inspect_maze(&self) {
        self.inspect_grid();
    }
}
