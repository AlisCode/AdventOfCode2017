extern crate advent_of_code_2017;

use self::advent_of_code_2017::day24::{Component, resolve_part_one, resolve_part_two};

#[test]
pub fn test_day24_component_strength() {
    let c: Component = Component::new(0, 1);
    let c2: Component = Component::new(1, 0);
    let c3: Component = Component::new(2, 3);

    assert_eq!(c.strength, 1);
    assert_eq!(c2.strength, 1);
    assert_eq!(c3.strength, 5);
}

#[test]
pub fn test_day24_component_from_input() {
    let c1: Component = Component::new(0, 1);
    let c2: Component = Component::new(1, 0);
    let c3: Component = Component::new(2, 3);

    let s1: &str = "0/1";
    let s2: &str = "1/0";
    let s3: &str = "2/3";

    assert_eq!(Component::from_input(s1), c1);
    assert_eq!(Component::from_input(s2), c2);
    assert_eq!(Component::from_input(s3), c3);
}

const INPUT: &str = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

#[test]
pub fn test_day24_part_one() {
    assert_eq!(resolve_part_one(INPUT), 31);
}

#[test]
pub fn test_day24_part_two() {
    assert_eq!(resolve_part_two(INPUT), 23);
}
