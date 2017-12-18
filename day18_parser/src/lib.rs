#[macro_use]
extern crate nom;

use nom::{alphanumeric, digit, space};
use std::str;

#[derive(Debug)]
pub enum Action {
    Sound,
    Set,
    Add,
    Mul,
    Modulo,
    Receive,
    JumpGreaterZero,
}

impl Action {
    fn from_str(input: &str) -> Action {
        match input {
            "snd" => Action::Sound,
            "set" => Action::Set,
            "add" => Action::Add,
            "mul" => Action::Mul,
            "mod" => Action::Modulo,
            "rcv" => Action::Receive,
            "jgz" => Action::JumpGreaterZero,
            _ => unreachable!(),
        }
    }
}

named!(
    action_tags,
    alt!(
        tag!("snd") | tag!("set") | tag!("add") | tag!("mul") | tag!("mod") | tag!("rcv")
            | tag!("jgz")
    )
);

named!(
    parse_action<Action>,
    map!(map_res!(action_tags, str::from_utf8), Action::from_str)
);

named!(
    number<i32>,
    map_res!(
        map_res!(recognize!(pair!(opt!(tag!("-")), digit)), str::from_utf8),
        str::parse
    )
);

named!(parse_reg<&str>, map_res!(alphanumeric, str::from_utf8));


named!(
    parse_line<(Action, &str, Option<i32>)>,
    do_parse!(
        action: parse_action >> space >> reg: parse_reg >> opt!(space) >> val: opt!(number)
            >> (action, reg, val)
    )
);

pub fn do_parse_line(input: &str) -> (Action, &str, Option<i32>) {
    parse_line(input.as_bytes()).to_result().unwrap()
}
