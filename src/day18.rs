extern crate day18_parser;

use self::day18_parser::{do_parse_line, Action};

#[derive(Debug)]
pub struct Instruction {
    action: Action,
    register: String,
    arg: Option<i32>,
}

impl Instruction {

    pub fn new(data: (Action, &str, Option<i32>)) -> Self {
        Instruction {
            action: data.0,
            register: data.1.into(),
            arg: data.2,
        }
    }

}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|a| Instruction::new(do_parse_line(a)))
        .collect()
}

