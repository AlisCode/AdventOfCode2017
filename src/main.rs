extern crate advent_of_code_2017;

use advent_of_code_2017::day20::parse_input;

fn main() {

    let input: &str = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>\np=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";

    let list = parse_input(input);
    println!("Particles: {:?}", list);

}
