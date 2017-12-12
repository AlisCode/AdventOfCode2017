extern crate day12_parser;

use self::day12_parser::parse_input;
use self::day12_parser::Program;

pub fn resolve_first_part(input: &str)
{
    let test: Vec<Program> = parse_input(input);

    test.iter().for_each(|a| {

        println!("ID : {}", a.id);
        println!("Linked_Ids: {:?}", a.linked_programs);

    });
    
}

