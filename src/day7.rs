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

pub fn get_correct_weight(
    parent_name: &String,
    unbalanced_name: &String,
    list: &Vec<Process>,
) -> u32 {
    let parent = list.iter()
        .filter(|a| a.name == *parent_name)
        .next()
        .unwrap();

    list.iter()
        .filter_map(|a| {
            if parent.children_names.contains(&a.name) && a.name != *unbalanced_name {
                Some(a.weight.clone())
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

pub fn resolve_part_two(list: &Vec<Process>) -> u32 {
    let unbalanced_name = get_unbalanced_process_name(list);
    let unbalanced_process = list.iter()
        .filter(|a| a.name == unbalanced_name)
        .next()
        .unwrap();

    list.iter()
        .filter(|a| unbalanced_process.children_names.contains(&a.name))
        .for_each(|b| {
            println!(
                "Global weight of {} is : {}",
                b.name,
                b.get_global_weight(list)
            )
        });

    0
}
