mod maze_digger_generator;

fn main() {
    let mut mdm = maze_digger_generator::DiggerMethod::new(7, 7, '.', '#');
    mdm.generate();
    mdm.inspect_maze();
}
