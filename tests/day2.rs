extern crate advent_of_code_2017;

use advent_of_code_2017::day2::decode_spreadsheet;
use advent_of_code_2017::day2::decode_spreadsheet_second;

// Sample case for part 1
#[test]
fn test_decode_sample1() {

    let input = "5 1 9 5
    7 5 3
    2 4 6 8";
    assert_eq!(decode_spreadsheet(input), 18);

}


// This was unexpected, but numbers can have more than one digit lol
#[test]
fn test_decode_sample2() {

    let input = "51 13 94 53
    72 54 35
    23 42 61 84";
    assert_eq!(decode_spreadsheet(input), 179);

}

// Sample case for part 2
#[test]
fn test_decode_second_sample1() {

    let input = "5 9 2 8
    9 4 7 3
    3 8 6 5";
    assert_eq!(decode_spreadsheet_second(input), 9);

}