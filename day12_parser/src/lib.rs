#[macro_use]
extern crate nom;

use nom::{digit, space};
use std::str;

pub struct Program {
    pub id: i32,
    pub linked_programs: Vec<i32>,
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
            digit , str::from_utf8
        ), str::parse
    )
);

named!(get_linked_ids<Vec<i32>>, 
    separated_list_complete!(tag!(", "), number)
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
    input.lines().filter_map(|a| parse_line(a).ok()).collect()
}

pub fn parse_line(input: &str) -> Result<Program, nom::ErrorKind> {
    let (id, vec_ids) = parse_whole_line(input.as_bytes()).to_result()?;
    Ok(Program::new(id,vec_ids))
}