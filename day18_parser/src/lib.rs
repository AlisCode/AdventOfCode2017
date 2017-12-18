#[macro_use]
extern crate nom;

use nom::{alphanumeric, digit, space};
use std::str;

use std::collections::HashMap;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Value {
    register_value: Option<String>,
    direct_value: Option<i64>,
}

impl Value {
    pub fn new_with_register(reg: &str) -> Self {
        Value {
            register_value: Some(reg.into()),
            direct_value: None,
        }
    }

    pub fn new_with_value(val: i64) -> Self {
        Value {
            register_value: None,
            direct_value: Some(val),
        }
    }

    pub fn get_val(&self, registers: &HashMap<String, i64>) -> i64 {
        if let Some(val) = self.direct_value {
            return val;
        }
        let val_opt = registers.get(&self.register_value.clone().unwrap());

        match val_opt {
            Some(val) => *val,
            _ => 0,
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
    number<i64>,
    map_res!(
        map_res!(recognize!(pair!(opt!(tag!("-")), digit)), str::from_utf8),
        str::parse
    )
);

named!(parse_reg<&str>, map_res!(alphanumeric, str::from_utf8));

named!(
    parse_value_nb<Value>,
    do_parse!(val: number >> (Value::new_with_value(val)))
);


named!(
    parse_value_str<Value>,
    do_parse!(val: parse_reg >> (Value::new_with_register(val)))
);

named!(parse_value<Value>, alt!(parse_value_nb | parse_value_str));

named!(
    parse_line_with_arg<(Action, &str, Option<Value>)>,
    do_parse!(
        action: parse_action >> space >> reg: parse_reg >> space >> val: parse_value
            >> (action, reg, Some(val))
    )
);

named!(
    parse_line_without_arg<(Action, &str, Option<Value>)>,
    do_parse!(action: parse_action >> space >> reg: parse_reg >> (action, reg, None))
);

named!(
    parse_line<(Action, &str, Option<Value>)>,
    alt_complete!(parse_line_with_arg | parse_line_without_arg)
);



pub fn do_parse_line(input: &str) -> (Action, &str, Option<Value>) {
    parse_line(input.as_bytes()).to_result().unwrap()
}
