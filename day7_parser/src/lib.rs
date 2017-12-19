#[macro_use]
extern crate nom;
extern crate itertools;

use std::str;
use nom::{alphanumeric, space};

use itertools::Itertools;

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

#[derive(Debug, Clone)]
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

    pub fn get_global_weight(&self, list: &Vec<Process>) -> u32 {
        let children_weight: u32 = self.get_children_process(list)
            .iter()
            .map(|a| a.get_global_weight(list))
            .sum();

        self.weight + children_weight
    }

    /// NOTA : 
    /// Take the children process
    /// Group em by global_weight
    /// Balanced <==> only one group
    pub fn is_balanced(&self, list: &Vec<Process>) -> bool {
        let groups = self.get_children_process(list).into_iter().group_by(|a| a.get_global_weight(list));
        if groups.into_iter().count() == 1 {
            println!("process {} is balanced", self.name);
            return true;
        }
        else {
            println!("process {} is unbalanced", self.name);
            return false;
        }
    }

    /// NOTA :
    /// Take the children process
    /// Group by global_weight
    /// target weight is the key from the group that has a count > 1
    /// unbalanced child is the process that is not in this group (<==> the process is alone in its group)
    pub fn get_unbalanced_child_correct_weight() -> u32 {
        0
    }

    pub fn get_children_process<'a>(&self, list: &'a Vec<Process>) -> Vec<&'a Process> {
        list.iter()
        .filter_map(|a| if self.children_names.contains(&a.name) {
            Some(a)
        } else {
            None
        })
        .collect()
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
