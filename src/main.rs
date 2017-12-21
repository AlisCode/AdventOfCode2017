extern crate advent_of_code_2017;

use self::advent_of_code_2017::day21::parse_input;

fn main() {

  let input: &str = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";

  let rules = parse_input(input);
  println!("{:?}", rules);

}
