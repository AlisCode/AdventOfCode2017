extern crate advent_of_code_2017;

use advent_of_code_2017::day15::{resolve_part_one, resolve_part_two};

#[test]
pub fn test_day15_generators_one_sample1() {
    assert_eq!(resolve_part_one(65, 8921), 588);
}


#[test]
pub fn test_day15_generators_two_sample1() {
    assert_eq!(resolve_part_two(65, 8921), 309);
}