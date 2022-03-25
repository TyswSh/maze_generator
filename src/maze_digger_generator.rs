use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn new(h: usize, w: usize, wall: char) -> Map {
        Map {
            map: (0..h).map(|_| vec![wall; w]).collect(),
        }
    }

    fn set_path(&mut self, y: usize, x: usize, path: char) {
        self.map[y][x] = path;
    }

    fn get_map(self) -> Self {
        self.clone()
    }
}

#[derive(Debug)]
pub(crate) struct DiggerMethod {
    width: usize,
    height: usize,
    path: char,
    wall: char,
    maze: Map,
}

impl DiggerMethod {
    pub fn new(width: usize, height: usize, path: char, wall: char) -> DiggerMethod {
        DiggerMethod {
            width,
            height,
            path,
            wall,
            maze: Map::new(height, width, wall),
        }
    }

    pub fn generate(&mut self) {
        let x = get_random_position(&self.width);
        let y = get_random_position(&self.height);
        self.digger(y, x);
    }

    fn digger(&mut self, y: usize, x: usize) {
        // store direction
        let mut directions: Vec<Directions> = vec![
            Directions::Up,
            Directions::Down,
            Directions::Left,
            Directions::Right,
        ];

        self.maze.set_path(x, y, self.path);

        // get random directions
        let mut rng = thread_rng();
        directions.shuffle(&mut rng);

        // let mut updated_maze: Vec<Vec<char>> = maze.to_vec();
        for direction in directions {
            match direction {
                Directions::Up => {
                    // y-1, y-2
                    println!("Up");
                    if let Some(steped_y) = y.checked_sub(2) {
                        if self.can_dig(steped_y, x) {
                            self.maze.set_path(y - 1, x, self.path);
                            self.digger(steped_y, x);
                        }
                    }
                }
                Directions::Down => {
                    // y+1, y+2
                    println!("Down");
                    if self.can_dig(y, x + 2) {
                        self.maze.set_path(y, x + 1, self.path);
                        self.digger(y, x + 2);
                    }
                }
                Directions::Left => {
                    // x-1, x-2
                    println!("Left");
                    if let Some(steped_x) = x.checked_sub(2) {
                        if self.can_dig(y, steped_x) {
                            self.maze.set_path(y, x - 1, self.path);
                            self.digger(y, steped_x);
                        }
                    }
                }
                Directions::Right => {
                    // x+1, x+2
                    println!("Right");
                    if self.can_dig(y, x + 2) {
                        self.maze.set_path(y, x + 1, self.path);
                    }
                }
            }
            // self.inspect_maze();
        }
    }

    fn can_dig(&self, dy: usize, dx: usize) -> bool {
        let y = self.maze.map.get(dy);
        return match y {
            None => false,
            Some(x) => {
                let z = x.get(dx);
                match z {
                    None => false,
                    Some(&w) => {
                        if w == ' ' {
                            false
                        } else {
                            true
                        }
                    }
                }
            }
        };
    }

    pub fn inspect_maze(&self) {
        for i in &self.maze.map {
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
