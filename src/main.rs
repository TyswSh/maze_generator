fn main() {
    digging_method::setup();
}

mod digging_method {

    use rand::prelude::*;
    use rand::seq::SliceRandom;

    pub fn setup() {
        let maze_width = 7;
        let maze_height = 7;

        let mut maze: Vec<Vec<char>> = Vec::new();

        // #: wall, .: path
        for _ in 0..maze_height {
            let mut line: Vec<char> = Vec::new();
            for _ in 0..maze_width {
                line.push('#');
            }
            maze.push(line);
        }

        let mut rng = thread_rng();
        let x: usize = {
            loop {
                let n = rng.gen_range(0..maze_width) as usize;
                if n % 2 != 0 {
                    break n;
                }
            }
        };

        let y: usize = {
            loop {
                let n = rng.gen_range(0..maze_height) as usize;
                if n % 2 != 0 {
                    break n;
                }
            }
        };
        println!("Start position...");
        println!("x: {}, y: {}", x, y);
        digger(y, x, &mut maze);
    }

    #[derive(Debug)]
    enum Directions {
        Up,
        Down,
        Left,
        Right,
    }

    fn digger(y: usize, x: usize, maze: &mut Vec<Vec<char>>) {
        // store direction
        let mut directions: Vec<Directions> = vec![
            Directions::Up,
            Directions::Down,
            Directions::Left,
            Directions::Right,
        ];

        maze[y][x] = '.';

        // get random directions
        let mut rng = thread_rng();
        directions.shuffle(&mut rng);

        for direction in directions {
            match direction {
                Directions::Up => {
                    // println!("Up");
                    // y-1, y-2
                    if let Some(step_y) = y.checked_sub(2) {
                        if can_dig(step_y, x, maze) {
                            maze[y - 1][x] = '.';
                            digger(step_y, x, maze);
                        }
                    }
                }
                Directions::Down => {
                    // println!("Down");
                    // y+1, y+2
                    if can_dig(y + 2, x, maze) {
                        maze[y + 1][x] = '.';
                        digger(y + 2, x, maze);
                    }
                }
                Directions::Left => {
                    // println!("Left");
                    // x-1, x-2
                    if let Some(step_x) = x.checked_sub(2) {
                        if can_dig(y, step_x, maze) {
                            maze[y][x - 1] = ' ';
                            digger(y, step_x, maze);
                        }
                    }
                }
                Directions::Right => {
                    // println!("Right");
                    // x+1, x+2
                    if can_dig(y, x + 2, maze) {
                        maze[y][x + 1] = '.';
                        digger(y, x + 2, maze);
                    }
                }
            }

            for l in maze.iter() {
                println!("{:?}", l);
            }
        }
    }

    fn can_dig(dy: usize, dx: usize, maze: &Vec<Vec<char>>) -> bool {
        let y = maze.get(dy);
        return match y {
            None => false,
            Some(x) => {
                let z = x.get(dx);
                match z {
                    None => false,
                    Some(&w) => {
                        if w == '.' {
                            false
                        } else {
                            true
                        }
                    }
                }
            }
        };
    }
}
