extern crate advent_of_code_2017;

use advent_of_code_2017::day14::{count_groups, count_used_cells, create_grid_for_input};

fn main() {
    //let input: &str = "jxqlasbh";
    let input: &str = "flqrgnkx";
    let grid: Vec<Vec<i32>> = create_grid_for_input(input);

    //println!("res: {}", count_used_cells(&grid));
    println!("res: {}", count_groups(&grid));
    // should print 1242
}
