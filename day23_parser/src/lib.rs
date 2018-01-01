#[macro_use]
extern crate nom;

use nom::{alpha, digit, space};
use std::str;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Set,
    Sub,
    Mul,
    Jnz,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Number(i32),
    Register(String),
}

pub struct Registers {
    pub values: HashMap<String, i32>,
}

impl Registers {
    pub fn new() -> Self {
        Registers { values: HashMap::new() }
    }

    pub fn get_val(&mut self, reg: &str) -> i32 {

        if self.values.contains_key(reg) {
            return *self.values.get(reg).unwrap();
        }

        return 0;
    }

    pub fn set_val(&mut self, reg: &str, new_val: i32) {

        if self.values.contains_key(reg) {
            let val = self.values.get_mut(reg).unwrap();
            *val = new_val;
        } else {
            self.values.insert(reg.into(), new_val);
        }
    }
}

impl Action {
    fn from_str(input: &str) -> Action {
        match input {
            "set" => Action::Set,
            "sub" => Action::Sub,
            "mul" => Action::Mul,
            "jnz" => Action::Jnz,
            _ => unreachable!(),
        }
    }
}

impl Value {
    fn from_nb(input: i32) -> Value {
        Value::Number(input)
    }

    fn from_str(input: &str) -> Value {
        Value::Register(input.into())
    }

    pub fn get_val(&self, regs: &Registers) -> i32 {
        match self {
            Value::Number(n) => n,
            Value::Register(r) => regs.get_val(r),
        }
    }
}

named!(
    action_tags,
    alt!(tag!("set") | tag!("sub") | tag!("mul") | tag!("jnz"))
);

named!(
    get_action<Action>,
    map!(map_res!(action_tags, str::from_utf8), Action::from_str)
);

named!(
    number<i32>,
    map_res!(
        map_res!(recognize!(pair!(opt!(tag!("-")), digit)), str::from_utf8),
        str::parse
    )
);

named!(parse_reg<&str>, map_res!(alpha, str::from_utf8));

named!(
    get_value<Value>,
    alt!(
        map!(number, Value::from_nb) |
        map!(parse_reg, Value::from_str)
    )
);

named!(
    parse_line<(Action, &str, Value)>, do_parse!(
        action: get_action >>
        space >>
        register: parse_reg >>
        space >>
        value: get_value >>
        (action, register, value)
    )
);

pub fn do_parse_line(input: &str) -> (Action, &str, Value) {

    parse_line(input.as_bytes()).to_result().unwrap()

}