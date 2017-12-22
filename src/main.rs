extern crate advent_of_code_2017;

use self::advent_of_code_2017::day21::resolve_part_one;

fn main() {
    let input: &str = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";

    let res = resolve_part_one(input);
    println!("result: {}", res);
}
