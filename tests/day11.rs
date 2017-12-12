extern crate advent_of_code_2017;

use advent_of_code_2017::day11::HexaCase;

#[test]
pub fn test_day11_steps_sample1() {
    let input: &str = "ne,ne,ne";
    let mut case: HexaCase = HexaCase::new();
    case.follow_path(input);

    assert_eq!(case.get_steps(), 3);
}

#[test]
pub fn test_day11_steps_sample2() {
    let input: &str = "ne,ne,sw,sw";
    let mut case: HexaCase = HexaCase::new();
    case.follow_path(input);

    assert_eq!(case.get_steps(), 0);
}

#[test]
pub fn test_day11_steps_sample3() {
    let input: &str = "ne,ne,s,s";
    let mut case: HexaCase = HexaCase::new();
    case.follow_path(input);

    assert_eq!(case.get_steps(), 2);
}
#[test]

pub fn test_day11_steps_sample4() {
    let input: &str = "se,sw,se,sw,sw";
    let mut case: HexaCase = HexaCase::new();
    case.follow_path(input);

    assert_eq!(case.get_steps(), 3);
}
