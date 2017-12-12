extern crate day12_parser;
extern crate petgraph;

use self::day12_parser::parse_input;
use self::day12_parser::Program;

use self::petgraph::graph::{Graph, NodeIndex};
use self::petgraph::algo::{connected_components, has_path_connecting};

/// Resolves the first part
/// Yes, there is a shorter way to write this with HashSet / HashMap
pub fn resolve_first_part(input: &str) -> i32 {
    let list_programs: Vec<Program> = parse_input(input);
    let mut graph = Graph::<i32, i32>::new();

    // Adds the nodes to the list
    list_programs.iter().for_each(|a| {
        graph.add_node(a.id.clone());
    });

    // Adds the edges
    list_programs.iter().for_each(|a| {
        a.linked_programs.iter().for_each(|b| {
            graph.add_edge(
                NodeIndex::new(a.id as usize),
                NodeIndex::new(*b as usize),
                1,
            );
        });
    });

    // Computes the total number of nodes in group of program with id 0
    count_nodes_going_to(0, &graph)
}


/// Resolves the second part
/// Yes, there is ALSO a shorter way to write this with HashSet / HashMap
pub fn resolve_second_part(input: &str) -> i32 {
    let list_programs: Vec<Program> = parse_input(input);
    let mut graph = Graph::<i32, i32>::new();

    // Adds the nodes to the list
    list_programs.iter().for_each(|a| {
        graph.add_node(a.id.clone());
    });

    // Adds the edges
    list_programs.iter().for_each(|a| {
        a.linked_programs.iter().for_each(|b| {
            graph.add_edge(
                NodeIndex::new(a.id as usize),
                NodeIndex::new(*b as usize),
                1,
            );
        });
    });

    // Computes the total number of groups using what petgraph allows us to do.
    // Yes, searching the doc is useful sometimes :)
    connected_components(&graph) as i32
}

/// Counts the nodes having path that connects to
fn count_nodes_going_to(index: i32, graph: &Graph<i32, i32>) -> i32 {
    graph
        .node_indices()
        .map(|a| {
            if has_path_connecting(graph, a, NodeIndex::new(index as usize), None) {
                1
            } else {
                0
            }
        })
        .sum()
}
