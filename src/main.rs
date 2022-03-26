mod bou_taoshi_generator;
mod maze_digger_generator;

fn main() {
    // height and width must be greater than 2.
    let mut mdm = maze_digger_generator::DiggerMethod::new(7, 7, '.', '#');
    mdm.generate();

    // only print in console
    mdm.inspect_maze();

    // return vec<vec<char>> of maze grid
    // mdm.get_maze_grid();

    // panic
    // let mut mdm = maze_digger_generator::DiggerMethod::new(2, 2, 'O', 'X');

    let mut btm = bou_taoshi_generator::BouTaoshiMethod::new(13, 13, '.', '#');

    btm.generate();

    btm.inspect_maze();
}
