extern crate day7_parser;

use day7_parser::Process;

pub fn get_root_process<'a>(process: &'a Process) -> &'a str {
    match process.parent {
        Some(parent) => get_root_process(&parent),
        _ => process.name,
    }
}
