extern crate day7_parser;

use day7_parser::Process;
use itertools::Itertools;
use itertools::structs::Group;

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
    println!("Unbalanced len: {}", unbalanced.len());

    /*let groups = list.into_iter().group_by(|a| a.get_global_weight(list));

    let group: Vec<(u32, Vec<&Process>)> = groups
        .into_iter()
        .filter_map(|(key, val)| {
            let child_list: Vec<&Process> = val.into_iter().collect();
            if child_list.len() > 1 {
                Some((key, child_list))
            } else {
                None
            }
        })
        .collect();

    group.sort();*/

    /*
    for (key, val) in group {
        val.sort();
    }*/

    /*
    for (key, val) in groups.into_iter() {
        let test: Vec<&Process> = val.into_iter().collect();
        println!("{:?}", test.len());
    }*/

    /*let mut groups_

    /*groups
        .into_iter()
        .for_each(|(key, b)| b.into_iter().for_each(|c| println!("{:?}", c)));*/

    let targetWeight = groups
        .into_iter()
        .filter(|pair| {
            let paire = pair.clone();
            paire.1.into_iter().for_each(|a| println!("{:?}", a));
            true
        })
        .next()
        .unwrap();*/

    0
}
