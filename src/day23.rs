extern crate day23_parser;

use std::collections::HashMap;
use day23_parser::{Action, Value, Registers, do_parse_line};

pub fn resolve_part_one() {}

pub fn parse_input(input: &str) -> Vec<(Action, &str, Value)> {
    input.lines().map(|a| do_parse_line(a)).collect()
}

pub fn handle_action(
    act: &Action,
    reg: &str,
    val: &Value,
    regs: &mut Registers,
    index: &mut usize,
) {
    match act {
        &Action::Set => (),
        &Action::Mul => (),
        &Action::Sub => (),
        &Action::Jnz => (),
    }
}