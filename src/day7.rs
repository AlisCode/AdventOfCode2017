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

pub fn get_unbalanced_process_name(list: &Vec<Process>) -> String {
    let proc_unbalanced = list.iter()
        .filter(|a| !a.is_balanced(&list))
        .next()
        .unwrap();

    proc_unbalanced.name.clone()
}

pub fn get_parent_process(name: &String, list: &Vec<Process>) -> String {
    let parent_proc = list.iter()
        .filter(|a| a.children_names.contains(name))
        .next()
        .unwrap();

    parent_proc.name.clone()
}

pub fn resolve_part_two(list: &Vec<Process>) -> u32 {
    let unbalanced: Vec<&Process> = list.iter().filter(|a| !a.is_balanced(list)).collect();
    println!("len : {}", unbalanced.len());
    let process: &Process = unbalanced[2];
    process.get_unbalanced_child_correct_weight(list)
}
