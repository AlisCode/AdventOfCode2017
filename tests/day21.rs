extern crate advent_of_code_2017;

use advent_of_code_2017::day21::{Grid, parse_grid, parse_input};
use std::collections::HashMap;

#[test]
pub fn test_day21_grid_parsing_sample1() {
    let input: &str = ".#./..#/###";
    let grid: Grid = parse_grid(input);

    let wanted_grid = Grid::new(vec!(
        vec!(false, true, false),
        vec!(false, false, true),
        vec!(true, true, true),
    ));

    assert_eq!(grid.values, wanted_grid.values);
    assert_eq!(grid.size, wanted_grid.size);
}

#[test]
pub fn test_day21_grid_transpose_sample1() {
    let init_grid = Grid::new(vec!(
        vec!(false, true, false),
        vec!(false, false, true),
        vec!(true, true, true),
    ));
    let transposed_grid: Grid = init_grid.transpose();

    let wanted_grid = Grid::new(vec!(
        vec!(false, false, true),
        vec!(true, false, true),
        vec!(false, true, true),
    ));

    assert_eq!(transposed_grid.values, wanted_grid.values);
}

#[test]
pub fn test_day21_grid_transpose_sample2() {
    let init_grid = Grid::new(vec!(
        vec!(false, false),
        vec!(true, false),
    ));
    let transposed_grid: Grid = init_grid.transpose();

    let wanted_grid = Grid::new(vec!(
        vec!(false, true),
        vec!(false, false),
    ));

    assert_eq!(transposed_grid.values, wanted_grid.values);
}

#[test]
pub fn test_day21_grid_rotate_right_sample1() {
    let init_grid = Grid::new(vec!(
        vec!(false, true, false),
        vec!(false, false, true),
        vec!(true, true, true),
    ));
    let transposed_grid: Grid = init_grid.rotate_right();

    let wanted_grid = Grid::new(vec!(
        vec!(true, false, false),
        vec!(true, false, true),
        vec!(true, true, false),
    ));

    assert_eq!(transposed_grid.values, wanted_grid.values);
}

#[test]
pub fn test_day21_grid_rotate_right_sample2() {
    let init_grid = Grid::new(vec!(
        vec!(false, true),
        vec!(true, false),
    ));
    let transposed_grid: Grid = init_grid.rotate_right();

    let wanted_grid = Grid::new(vec!(
        vec!(true, false),
        vec!(false, true),
    ));

    assert_eq!(transposed_grid.values, wanted_grid.values);
}

#[test]
pub fn test_day21_grid_flip_horizontal_sample1() {
    let init_grid = Grid::new(vec!(
        vec!(false, true, false),
        vec!(false, false, true),
        vec!(true, true, true),
    ));
    let flipped_grid: Grid = init_grid.flip_horizontal();

    let wanted_grid = Grid::new(vec!(
        vec!(true, true, true),
        vec!(false, false, true),
        vec!(false, true, false),
    ));

    assert_eq!(flipped_grid.values, wanted_grid.values);
}

#[test]
pub fn test_day21_grid_flip_vertical_sample1() {
    let init_grid = Grid::new(vec!(
        vec!(false, true, false),
        vec!(false, false, true),
        vec!(true, true, true),
    ));
    let flipped_grid: Grid = init_grid.flip_vertical();

    let wanted_grid = Grid::new(vec!(
        vec!(false, true, false),
        vec!(true, false, false),
        vec!(true, true, true),
    ));

    assert_eq!(flipped_grid.values, wanted_grid.values);
}

#[test]
pub fn test_day21_grid_matching_sample1() {
    let grid_one: Grid = parse_grid(".#./..#/###");
    let grid_two: Grid = parse_grid(".#./#../###");
    let grid_three: Grid = parse_grid("#../#.#/##.");
    let grid_four: Grid = parse_grid("###/..#/.#.");

    let grid_five: Grid = parse_grid("##/##");

    assert_ne!(grid_one, grid_five);
    assert_ne!(grid_two, grid_five);
    assert_ne!(grid_three, grid_five);
    assert_ne!(grid_four, grid_five);

    assert_eq!(grid_one, grid_two);
    assert_eq!(grid_one, grid_three);
    assert_eq!(grid_one, grid_four);

    assert_eq!(grid_two, grid_three);
    assert_eq!(grid_two, grid_four);

    assert_eq!(grid_three, grid_four);
}

#[test]
pub fn test_day21_grid_matching_sample2() {
    let grid_one: Grid = parse_grid("#./..");
    let grid_two: Grid = parse_grid(".#/..");
    let grid_three: Grid = parse_grid("../#.");
    let grid_four: Grid = parse_grid("../.#");

    let grid_match: Grid = parse_grid("../.#");

    assert_eq!(grid_one, grid_match);
    assert_eq!(grid_two, grid_match);
    assert_eq!(grid_three, grid_match);
    assert_eq!(grid_four, grid_match);
}

#[test]
pub fn test_day21_grid_reconstruct_sample1() {
    let grid_one: Grid = parse_grid("#./..");
    let grid_two: Grid = parse_grid(".#/..");
    let grid_three: Grid = parse_grid("../#.");
    let grid_four: Grid = parse_grid("../.#");

    let mut list_grid = vec!(grid_one, grid_two, grid_three, grid_four);
    let final_grid = Grid::reconstruct(&mut list_grid);

    let wanted_grid = parse_grid("#..#/..../..../#..#");

    assert_eq!(final_grid.values, wanted_grid.values);
}

#[test]
pub fn test_day21_grid_reconstruct_sample2() {
    let grid_one: Grid = parse_grid("#./..");

    let mut list_grid = vec!(grid_one);
    let final_grid = Grid::reconstruct(&mut list_grid);

    let wanted_grid = parse_grid("#./..");

    assert_eq!(final_grid.values, wanted_grid.values);
}

#[test]
pub fn test_day21_grid_reconstruct_sample3() {
    let grid_one: Grid = parse_grid("##./#../...");
    let grid_two: Grid = parse_grid("##./#../...");
    let grid_three: Grid = parse_grid("##./#../...");
    let grid_four: Grid = parse_grid("##./#../...");

    let mut list_grid = vec!(grid_one, grid_two, grid_three, grid_four);
    let final_grid = Grid::reconstruct(&mut list_grid);

    let wanted_grid = parse_grid("##.##./#..#../....../##.##./#..#../......");

    assert_eq!(final_grid.values, wanted_grid.values);
}

#[test]
pub fn test_day21_grid_split_sample1() {
    let global_grid = parse_grid("#..#/..../..../#..#");

    let grid_one: Grid = parse_grid("#./..");
    let grid_two: Grid = parse_grid(".#/..");
    let grid_three: Grid = parse_grid("../#.");
    let grid_four: Grid = parse_grid("../.#");

    let list_grid = global_grid.split();

    assert_eq!(grid_one.values, list_grid[0].values);
    assert_eq!(grid_two.values, list_grid[1].values);
    assert_eq!(grid_three.values, list_grid[2].values);
    assert_eq!(grid_four.values, list_grid[3].values);
}

#[test]
pub fn test_day21_grid_split_sample2() {
    let global_grid = parse_grid("#.#/.../#.#");
    let grid: Grid = parse_grid("#.#/.../#.#");
    let splitted_grid = global_grid.split();

    assert_eq!(grid.values, splitted_grid[0].values);
}

#[test]
pub fn test_day21_grid_count_on_sample1() {
    let grid: Grid = parse_grid("##/##");
    assert_eq!(grid.count_on_cases(), 4);
}

#[test]
pub fn test_day21_grid_count_on_sample2() {
    let grid: Grid = parse_grid("../..");
    assert_eq!(grid.count_on_cases(), 0);
}

#[test]
pub fn test_day21_grid_count_on_sample3() {
    let grid: Grid = parse_grid(".##/#.#/.#.");
    assert_eq!(grid.count_on_cases(), 5);
}

#[test]
pub fn test_day21_rules_hashmap_sample1() {
    let rules: HashMap<Grid, Grid> = parse_input("../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#");

    let grid_one: Grid = parse_grid("#./..");
    let grid_two: Grid = parse_grid(".#/..");
    let grid_three: Grid = parse_grid("../#.");
    let grid_four: Grid = parse_grid("../.#");

    assert_eq!(rules.contains_key(&grid_four), true);
    assert_eq!(rules.contains_key(&grid_one), true);
    assert_eq!(rules.contains_key(&grid_two), true);
    assert_eq!(rules.contains_key(&grid_three), true);
}