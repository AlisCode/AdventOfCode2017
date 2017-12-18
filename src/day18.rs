extern crate day18_parser;

use self::day18_parser::{do_parse_line, Action};

pub fn parse_input(input: &str) {
    input
        .lines()
        .map(|a| do_parse_line(a))
        .for_each(|(action, reg, val)| println!("----\nAction: {:?}\nReg: {:?}\nVal: {:?}", action, reg, val));
}
