#[macro_use]
extern crate nom;

use std::str;
use nom::{alphanumeric, space};


/// Gets the process' name
named!(process_name<&str>, map_res!(alphanumeric, str::from_utf8));

/// Gets the process' weight
named!(
    process_weight<u32>,
    map_res!(
        map_res!(
            delimited!(char!('('), is_not!(")"), char!(')')),
            str::from_utf8
        ),
        str::parse
    )
);

named!(
    process_children<Vec<&str>>,
    separated_list_complete!(tag!(", "), process_name)
);

named!(
    parse_line<(&str, u32, Vec<&str>)>,
    do_parse!(
        name: process_name >> space >> weight: process_weight >> space >> opt!(tag!(" -> "))
            >> children: process_children >> (name, weight, children)
    )
);

pub struct Process<'a> {
    pub name: &'a str,
    pub weight: u32,
    pub children_names: Vec<&'a str>,
    pub children_process: Vec<&'a Process<'a>>,
    pub parent: Option<&'a Process<'a>>,
}

impl<'a> Process<'a> {
    pub fn new(name: &'a str, weight: u32, children: Vec<&'a str>) -> Self {
        Process {
            name,
            weight,
            children_names: children,
            children_process: Vec::new(),
            parent: None,
        }
    }
}

pub fn parse_input<'a>(input: &str) -> Vec<Process> {
    input
        .lines()
        .filter_map(|a| parse_line(a.as_bytes()).to_result().ok())
        .map(|(name, weight, children)| {
            Process::new(name, weight, children)
        })
        .collect()
}
