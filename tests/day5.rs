extern crate advent_of_code_2017;

use advent_of_code_2017::day5::count_steps_first;
use advent_of_code_2017::day5::count_steps_second;

#[test]
fn test_day5_count_steps_first_sample1() {

    let input = "0
    3
    0
    1
    -3";

    assert_eq!(count_steps_first(input), 5);
}

#[test]
fn test_day5_count_steps_second_sample1() {

    let input = "0
    3
    0
    1
    -3";

    assert_eq!(count_steps_second(input), 10);
}