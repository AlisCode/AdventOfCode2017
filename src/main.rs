extern crate advent_of_code_2017;

use advent_of_code_2017::day18::parse_input;

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

    parse_input(input);
}
