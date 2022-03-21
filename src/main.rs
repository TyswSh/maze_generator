
fn main() {
    digging_method::setup();
}

mod digging_method {

    use rand::prelude::*;
    use rand::seq::SliceRandom;

    pub fn setup() {

        let maze_width = 7;
        let maze_height = 5;

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
        println!("x: {}, y: {}", x, y);
        digger(x, y, &mut maze);
    }

    #[derive(Debug)]
    enum Directions {
        Up,
        Down,
        Left,
        Right,
    }

    fn digger(x: usize, y: usize, maze: &mut Vec<Vec<char>>) {

        // store direction
        let mut directions: Vec<Directions> = Vec::new();
        let mut positions: Vec<(usize, usize)> = Vec::new();

        // can dig or not
        // Check if usize is a negative value
        if let Some(step_y) = y.checked_sub(2) {
            if can_dig(step_y, x, &maze) {
                directions.push(Directions::Up);
                positions.push((step_y, x));
            }
        }

        if can_dig(y+2, x, &maze) {
            directions.push(Directions::Down);
            positions.push((y+2, x));
        }

        // Check if usize is a negative value
        if let Some(step_x) = x.checked_sub(2) {
            if can_dig(y, step_x, &maze) {
                directions.push(Directions::Left);
                positions.push((y, step_x));
            }
        }
        if can_dig(y, x+2, &maze) {
            directions.push(Directions::Right);
            positions.push((y, x+2));
        }

        // cannot dig
        if directions.is_empty() {
            return;
        }

        maze[y][x] = '.';

        for line in maze.iter() {
            println!("{:?}", line);
        }
        println!("{:?}", positions);

        // get random directions
        let mut rng = thread_rng();
        directions.shuffle(&mut rng);

        for direction in directions {
            let mut update_x = 0;
            let mut update_y = 0;
            match direction {
                Directions::Up => {
                    println!("Up");
                    // y-1, y-2
                    maze[y-1][x] = '.';
                    update_x = x;
                    update_y = y-2;
                },
                Directions::Down => {
                    println!("Down");
                    // y+1, y+2
                    maze[y+1][x] = '.';
                    update_x = x;
                    update_y = y + 1;
                },
                Directions::Left => {
                    println!("Left");
                    // x-1, x-2
                    maze[y][x-1] = '.';
                    update_x = x - 2;
                    update_y = y;
                },
                Directions::Right=> {
                    println!("Right");
                    // x+1, x+2
                    maze[y][x+1] = '.';
                    update_x = x + 2;
                    update_y = y;
                },
            }

            digger(update_y, update_x, maze);
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
                    Some(&w)=> {
                        if w == '.' {
                            false
                        } else {
                            true
                        }
                    }
                }
            }
        }
    }
}
