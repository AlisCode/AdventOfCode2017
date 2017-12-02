extern crate advent_of_code_2017;

use advent_of_code_2017::day2::decode_spreadsheet;
use advent_of_code_2017::day2::decode_spreadsheet_second;
use advent_of_code_2017::day2::find_min_max;
use advent_of_code_2017::day2::find_evenly_divisible_numbers;

// One-line-only sample case for part 1
#[test]
fn day2_test_decode_sample1() {

    let input = "5 1 9 5";
    assert_eq!(find_min_max(input), Some((1,9)));

}

// Complete sample case for part 1
#[test]
fn day2_test_decode_sample2() {

    let input = "5 1 9 5
    7 5 3
    2 4 6 8";
    assert_eq!(decode_spreadsheet(input), 18);

}


// This was unexpected, but numbers can have more than one digit lol
// So here is a test
#[test]
fn day2_test_decode_sample3() {

    let input = "51 13 94 53
    72 54 35
    23 42 61 84";
    assert_eq!(decode_spreadsheet(input), 179);

}


// One-line-only sample case for part 2
#[test]
fn day2_test_decode_second_sample1() {

    let input = "5 9 2 8";
    assert_eq!(find_evenly_divisible_numbers(input), Some((8,2)));

}


// Complete sample case for part 2
#[test]
fn day2_test_decode_second_sample2() {

    let input = "5 9 2 8
    9 4 7 3
    3 8 6 5";
    assert_eq!(decode_spreadsheet_second(input), 9);

}