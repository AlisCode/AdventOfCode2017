extern crate advent_of_code_2017;

use advent_of_code_2017::day12::{resolve_first_part, resolve_second_part};

#[test]
pub fn test_day12_part1_sample1() {
    let input: &str = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
    assert_eq!(resolve_first_part(input), 6);
}

#[test]
pub fn test_day12_part2_sample1() {
    let input: &str = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
    assert_eq!(resolve_second_part(input), 2);
}
