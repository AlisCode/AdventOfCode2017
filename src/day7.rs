extern crate day7_parser;
extern crate petgraph;

use day7_parser::Process;

use self::petgraph::Graph;

pub fn generate_graph<'a>(list: &Vec<Process>) -> Graph<(), Process> {
    let mut graph = Graph::new();

    list.iter().for_each(|a| {
        graph.add_node(&a);
    });


    graph
}
