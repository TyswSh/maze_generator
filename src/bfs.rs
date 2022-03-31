struct MapGrid {
    height: usize,
    width: usize,
    grid: Vec<Vec<char>>,
}

impl MapGrid {
    fn new(height: usize, width: usize, grid: Vec<Vec<char>>) -> MapGrid {
        MapGrid {
            height,
            width,
            grid,
        }
    }

    fn check_grid(&self, y: usize, x: usize, s: char) -> bool {
        self.grid[y][x] == s
    }

    fn change_grid(&mut self, y: usize, x: usize, s: char) {
        self.grid[y][x] = s;
    }

    fn is_in_grid(&self, y: usize, x: usize) -> bool {
        if y < self.height && x < self.width {
            return true;
        }

        false
    }

    fn print_map_grid(&self) {
        for l in &self.grid {
            println!("{}", l.iter().collect::<String>());
        }
    }
}
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
use std::{collections::VecDeque, panic};
pub struct BFS {
    grid: MapGrid,
    start_positions: VecDeque<(usize, usize)>,
    wall: char,
    start: char,
    goal: char,
    seached: char,
}

impl BFS {
    pub fn new(
        h: usize,
        w: usize,
        g: Vec<Vec<char>>,
        wall: char,
        start: char,
        goal: char,
        seached: char,
    ) -> BFS {
        let start_positions = VecDeque::new();
        BFS {
            grid: MapGrid::new(h, w, g),
            start_positions,
            wall,
            start,
            goal,
            seached,
        }
    }

    pub fn start_searching(&mut self) -> bool {
        let direction = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];

        while let Some((y, x)) = self.start_positions.pop_front() {
            self.print_map_grid();
            if self.grid.check_grid(y, x, self.goal) {
                return true;
            }
            for d in &direction {
                match d {
                    Direction::Up => {
                        if let Some(y) = y.checked_sub(1) {
                            if self.can_search(y, x) {
                                self.start_positions.push_back((y, x));
                            }
                        }
                    }
                    Direction::Right => {
                        if self.can_search(y, x + 1) {
                            self.start_positions.push_back((y, x + 1));
                        }
                    }
                    Direction::Down => {
                        if self.can_search(y + 1, x) {
                            self.start_positions.push_back((y + 1, x));
                        }
                    }
                    Direction::Left => {
                        if let Some(x) = x.checked_sub(1) {
                            if self.can_search(y, x) {
                                self.start_positions.push_back((y, x));
                            }
                        }
                    }
                }
            }
            self.set_searched(y, x);
        }
        false
    }

    fn can_search(&self, y: usize, x: usize) -> bool {
        if !self.grid.is_in_grid(y, x) {
            return false;
        }

        if self.grid.check_grid(y, x, self.seached) {
            return false;
        }

        if self.grid.check_grid(y, x, self.seached) {
            return false;
        }

        if self.grid.check_grid(y, x, self.wall) {
            return false;
        }

        true
    }

    fn set_searched(&mut self, y: usize, x: usize) {
        self.grid.change_grid(y, x, self.seached);
    }

    pub fn set_start_goal(&mut self, sy: usize, sx: usize, gy: usize, gx: usize) {
        if !self.grid.is_in_grid(sy, sx) {
            panic!("Invalid value: sy:{} or sx:{} are not in grid!!", sy, sx);
        }
        if !self.grid.is_in_grid(gy, gx) {
            panic!("Invalid value: gy:{} or gx:{} are not in grid!!", gy, gx);
        }
        self.grid.change_grid(sy, sx, self.start);
        self.grid.change_grid(gy, gx, self.goal);
        self.start_positions.push_back((sy, sx));
    }

    pub fn print_map_grid(&self) {
        self.grid.print_map_grid();
    }
}
