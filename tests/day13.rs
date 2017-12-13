extern crate advent_of_code_2017;

use advent_of_code_2017::day13::{generate_firewall, get_min_delay, get_path_severity};

#[test]
pub fn test_day13_path_severity_sample1() {
    let input: &str = "0: 3
1: 2
4: 4
6: 4";

    let firewall = generate_firewall(input);
    assert_eq!(get_path_severity(&firewall), 24);
}

#[test]
pub fn test_day13_min_delay_sample1() {
    let input: &str = "0: 3
1: 2
4: 4
6: 4";

    let firewall = generate_firewall(input);
    assert_eq!(get_min_delay(&firewall), 10);
}
