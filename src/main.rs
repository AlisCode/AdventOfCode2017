extern crate advent_of_code_2017;

use advent_of_code_2017::day18::{Instruction,parse_input};


fn main() {
    let input: &str = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

    let list: Vec<Instruction> = parse_input(input);
    list.iter().for_each(|a| println!("{:?}", a));
}
