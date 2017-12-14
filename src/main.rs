extern crate advent_of_code_2017;
extern crate day7_parser;

use day7_parser::{parse_input, Process};

fn main() {
    let input: &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    let list: Vec<Process> = parse_input(input);
}
