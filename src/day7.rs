extern crate day7_parser;

use day7_parser::Process;

pub fn get_root_process<'a>(list: &'a Vec<Process>) -> &'a str {
    let mut children_processes: Vec<&String> = Vec::new();

    list.iter().for_each(|a| {
        a.children_names.iter().for_each(|b| {
            children_processes.push(&b);
        });
    });

    match list.iter()
        .filter(|a| !children_processes.contains(&&a.name))
        .next()
    {
        Some(res) => &res.name,
        _ => "No result",
    }
}
