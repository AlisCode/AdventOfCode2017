extern crate rayon;

use self::rayon::prelude::*;

use day10::KnotHasher;

pub fn create_grid_for_input(input: &str) -> Vec<String>
{
    let mut hasher = KnotHasher::new();
    
    (0..128)
    .map(|a| {
        let new_input: String = format!("{}-{}", input, a);
        hasher.do_knot_for_input_second(new_input.as_str());
        hash_to_binary(hasher.get_dense_hash())
    })
    .collect()
}

pub fn hash_to_binary(input: String) -> String {
    let mut global: String = String::from("");
    input
    .chars()
    .for_each(|a| {
        let to_concat = format!("{:b}", a.to_digit(16).unwrap());
        global = format!("{}{}", global, to_concat);
    });

    global
}

pub fn count_used_cells(grid: &Vec<String>) -> i32
{
    grid
    .par_iter()
    .map(|a| {
        a.chars().filter(|b| *b == '1').count() as i32
    })
    .sum()
}