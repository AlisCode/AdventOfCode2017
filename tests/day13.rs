extern crate advent_of_code_2017;

use advent_of_code_2017::day13::{generate_firewall, get_path_severity};

#[test]
pub fn test_day13_path_severity_sample1() {
    let input: &str = "0: 3
1: 2
4: 4
6: 4";

    let (mut hash_map, max_depth) = generate_firewall(input);
    assert_eq!(get_path_severity(&mut hash_map, max_depth), (24, true));
}
