fn main() {
    println!("Hello, world!");
}

mod digging_method {
    fn setup() {
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
    }
}
