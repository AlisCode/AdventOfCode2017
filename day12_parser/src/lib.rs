#[macro_use]
extern crate nom;

use nom::{digit, space};
use std::str;

pub struct Program {
    id: i32,
    linked_programs: Vec<i32>,
}

impl Program {

    pub fn new(id: i32, linked_programs: Vec<i32>) -> Self {
        Program {
            id,
            linked_programs
        }
    }

}

named!(number<i32>, 
    map_res!(
        map_res!(
            recognize!(
                pair!(
                    opt!(tag!("-")),
                    digit
                )
            ), str::from_utf8
        ), str::parse
    )
);

named!(get_linked_ids<Vec<i32>>, 
    separated_list_complete!(number, tag!(", "))
);


named!(parse_whole_line<(i32, Vec<i32>)>, do_parse!(
    id: number >>
    space >>
    tag!("<->") >>
    space >>
    linked_ids: get_linked_ids >>
    (id, linked_ids)
));

pub fn parse_input(input: &str) -> Vec<Program> {
    Vec::new()
}

pub fn parse_line(input: &str) -> Result<Program, nom::ErrorKind> {
    Ok(Program::new(0,Vec::new()))
}