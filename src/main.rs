
fn main() {
    digging_method::setup();
}

mod digging_method {

    use rand::prelude::*;

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

        // can dig or not
        // Check if usize is a negative value
        if let Some(step_y) = y.checked_sub(2) {
            if can_dig(step_y, x, &maze) {
                println!("px: {}, py: {}", x, step_y);
                directions.push(Directions::Up);
            }
        }

        if can_dig(y+2, x, &maze) {
            println!("px: {}, py: {}", x, y+2);
            directions.push(Directions::Down);
        }

        // Check if usize is a negative value
        if let Some(step_x) = x.checked_sub(2) {
            if can_dig(y, step_x, &maze) {
                println!("px: {}, py: {}", step_x, y);
                directions.push(Directions::Left);
            }
        }
        if can_dig(y, x+2, &maze) {
            println!("px: {}, py: {}", x+2, y);
            directions.push(Directions::Right);
        }

        if directions.is_empty() {
            return;
        }

        maze[y][x] = '.';
        for d in directions {
            println!("{:?}", d);
        }

        for line in maze {
            println!("{:?}", line);
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
