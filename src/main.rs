extern crate advent_of_code_2017;

use advent_of_code_2017::day14::{create_grid_for_input, count_used_cells};

fn main() {
    
    let input: &str = "jxqlasbh";
    let grid: Vec<String> = create_grid_for_input(input);
    println!("res: {}", count_used_cells(&grid));

}
