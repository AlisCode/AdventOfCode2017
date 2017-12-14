extern crate advent_of_code_2017;

use advent_of_code_2017::day14::{create_grid_for_input, count_used_cells, count_groups};

#[test]
pub fn test_day14_count_cells_sample1() {
    let input: &str = "flqrgnkx";
    let grid = create_grid_for_input(input);
    assert_eq!(count_used_cells(&grid), 8108); 
}

#[test]
pub fn test_day14_count_groups_sample1() {
    let input: &str = "flqrgnkx";
    let mut grid = create_grid_for_input(input);
    assert_eq!(count_groups(&mut grid), 1242); 
}