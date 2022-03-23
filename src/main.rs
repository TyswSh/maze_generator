mod maze_digger_generator;

fn main() {
    let maze = maze_digger_generator::generate();
    for l in maze {
        let x: String = l.into_iter().collect();
        println!("{}", x);
    }
}
