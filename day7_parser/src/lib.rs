#[macro_use]
extern crate nom;

use std::str;
use nom::{alphanumeric, space};

use std::collections::HashMap;


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
        name: process_name >> space >> weight: process_weight >> opt!(complete!(tag!(" -> ")))
            >> children: process_children >> (name, weight, children)
    )
);

#[derive(Clone)]
pub struct Process {
    pub name: String,
    pub weight: u32,
    pub children_names: Vec<String>,
}

impl Process {
    pub fn new(name: String, weight: u32, children: Vec<String>) -> Self {
        Process {
            name,
            weight,
            children_names: children,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Process> {
    let mut list: Vec<Process> = Vec::new();

    input
        .lines()
        .filter_map(|a| parse_line(a.as_bytes()).to_result().ok())
        .map(|(name, weight, children)| {
            Process::new(
                name.into(),
                weight,
                children.into_iter().map(|a| a.into()).collect(),
            )
        })
        .for_each(|a| {
            list.push(a);
        });

    list
}
