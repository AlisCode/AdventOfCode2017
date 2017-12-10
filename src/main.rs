extern crate advent_of_code_2017;

use advent_of_code_2017::day10::KnotHasher;

fn main() {

    let mut my_knot_hasher: KnotHasher = KnotHasher::new();
    let input: &str = "199,0,255,136,174,254,227,16,51,85,1,2,22,17,7,192";

    my_knot_hasher.do_knot_for_input_first(input);
    println!("res: {}", my_knot_hasher.get_result());
}