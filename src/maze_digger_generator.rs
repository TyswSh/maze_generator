use rand::prelude::*;
use rand::seq::SliceRandom;

pub fn generate() -> Vec<Vec<char>> {
    let maze_width = 13;
    let maze_height = 13;

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
    return digger(y, x, &mut maze);
}

#[derive(Debug)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

fn digger(y: usize, x: usize, maze: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    // store direction
    let mut directions: Vec<Directions> = vec![
        Directions::Up,
        Directions::Down,
        Directions::Left,
        Directions::Right,
    ];

    maze[y][x] = ' ';

    // get random directions
    let mut rng = thread_rng();
    directions.shuffle(&mut rng);

    let mut updated_maze: Vec<Vec<char>> = maze.to_vec();
    for direction in directions {
        match direction {
            Directions::Up => {
                // println!("Up");
                // y-1, y-2
                if let Some(step_y) = y.checked_sub(2) {
                    if can_dig(step_y, x, maze) {
                        maze[y - 1][x] = ' ';
                        updated_maze = digger(step_y, x, maze);
                    }
                }
            }
            Directions::Down => {
                // println!("Down");
                // y+1, y+2
                if can_dig(y + 2, x, maze) {
                    maze[y + 1][x] = ' ';
                    updated_maze = digger(y + 2, x, maze);
                }
            }
            Directions::Left => {
                // println!("Left");
                // x-1, x-2
                if let Some(step_x) = x.checked_sub(2) {
                    if can_dig(y, step_x, maze) {
                        maze[y][x - 1] = ' ';
                        updated_maze = digger(y, step_x, maze);
                    }
                }
            }
            Directions::Right => {
                // println!("Right");
                // x+1, x+2
                if can_dig(y, x + 2, maze) {
                    maze[y][x + 1] = ' ';
                    updated_maze = digger(y, x + 2, maze);
                }
            }
        }
        debug_printer(maze.to_vec());
    }
    updated_maze
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

fn debug_printer(maze: Vec<Vec<char>>) {
    for i in maze {
        println!("{}", i.into_iter().collect::<String>());
    }
}
