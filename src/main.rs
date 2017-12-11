extern crate advent_of_code_2017;

use advent_of_code_2017::day11::HexaPath;

fn main() {
    let mut path: HexaPath = HexaPath::new();
    path.populate("ne,ne,s,s");

    println!("Val: {}", path.get_steps());
}
