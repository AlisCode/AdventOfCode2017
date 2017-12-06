extern crate advent_of_code_2017;

use advent_of_code_2017::day6::do_redistribution_cycle;
use advent_of_code_2017::day6::count_redistribution_cycle;
use advent_of_code_2017::day6::count_redistribution_cycle_loop;

#[test]
fn test_day6_redistribution_cycle_first_sample1() {
    let input: &str = "0 2 7 0";
    assert_eq!(do_redistribution_cycle(input), vec![2, 4, 1, 2]);
}

#[test]
fn test_day6_redistribution_cycle_first_sample2() {
    let input: &str = "2 4 1 2";
    assert_eq!(do_redistribution_cycle(input), vec![3, 1, 2, 3]);
}

#[test]
fn test_day6_redistribution_cycle_first_sample3() {
    let input: &str = "3 1 2 3";
    assert_eq!(do_redistribution_cycle(input), vec![0, 2, 3, 4]);
}

#[test]
fn test_day6_redistribution_cycle_first_sample4() {
    let input: &str = "0 2 3 4";
    assert_eq!(do_redistribution_cycle(input), vec![1, 3, 4, 1]);
}

#[test]
fn test_day6_redistribution_cycle_first_sample5() {
    let input: &str = "1 3 4 1";
    assert_eq!(do_redistribution_cycle(input), vec![2, 4, 1, 2]);
}

#[test]
fn test_day6_count_cycles_first_sample1() {
    let input: &str = "0 2 7 0";
    assert_eq!(count_redistribution_cycle(input), 5);
}


#[test]
fn test_day6_count_cycles_second_sample1() {
    let input: &str = "0 2 7 0";
    assert_eq!(count_redistribution_cycle_loop(input), 4);
}
