extern crate advent_of_code_2017;

use advent_of_code_2017::day17::resolve_part_one;

#[test]
pub fn test_day17_spinlock_one_sample1() {
    assert_eq!(resolve_part_one(3), 638);
}
