#[macro_use]
extern crate nom;

use nom::{digit, alpha, space};
use std::str;

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Inc,
    Dec,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Condition {
    Gt,
    Lt,
    Ge,
    Le,
    Eq,
    Ne,
}

#[derive(Debug)]
pub struct Instruction {
    pub reg_name: String,
    pub action: Action,
    pub val: i32,
    pub cond_reg_name: String,
    pub condition: Condition,
    pub cond_val: i32,
}

impl Instruction {
    pub fn new(r_n: String, a: Action, v: i32, c_r_n: String, c: Condition, c_v: i32) -> Self {
        Instruction {
            reg_name: r_n,
            action: a,
            val: v,
            cond_reg_name: c_r_n,
            condition: c,
            cond_val: c_v,
        }
    }
}

named!(get_str<&str>, map_res!(alpha, str::from_utf8));

named!(
    get_number<i32>,
    map_res!(
        map_res!(recognize!(pair!(opt!(tag!("-")), digit)), str::from_utf8),
        str::parse
    )
);

named!(
    condition<Condition>,
    map!(map_res!(condition_tags, str::from_utf8), parse_condition)
);

named!(condition_tags, 
    alt!(
        tag!(">=") |
        tag!("<=") |
        tag!(">") |
        tag!("<") |
        tag!("==") |
        tag!("!=")
    )
);

named!(
    action<Action>,
    map!(map_res!(action_tags, str::from_utf8), parse_action)
);

named!(action_tags, 
    alt!(
        tag!("inc") |
        tag!("dec")
    )
);

named!(parse_line<(&str, Action, i32, &str, Condition, i32)>, do_parse!(
    reg_name: get_str >>
    space >>
    act: action >>
    space >>
    val: get_number >>
    tag!(" if ") >>
    cond_reg_name: get_str >>
    space >>
    cond: condition >>
    space >>
    cond_val: get_number >>

    (reg_name, act, val, cond_reg_name, cond, cond_val)
));

fn parse_action(input: &str) -> Action {
    match input {
        "inc" => Action::Inc,
        "dec" => Action::Dec,
        _ => unreachable!(),
    }
}

fn parse_condition(input: &str) -> Condition {
    match input {
        ">" => Condition::Gt,
        "<" => Condition::Lt,
        ">=" => Condition::Ge,
        "<=" => Condition::Le,
        "==" => Condition::Eq,
        "!=" => Condition::Ne,
        _ => unreachable!(),
    }
}

pub fn parse_instruction(input: &str) -> Instruction {

    let (a, b, c, d, e, f) = parse_line(input.as_bytes()).to_result().unwrap();
    Instruction::new(a.into(), b, c, d.into(), e, f)

}
