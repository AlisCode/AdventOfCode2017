extern crate advent_of_code_2017;

use advent_of_code_2017::day10::KnotHasher;

#[test]
pub fn test_day10_knot_hash_second_sample1() {
    let input: &str = "";
    let mut my_knot_hasher: KnotHasher = KnotHasher::new();
    my_knot_hasher.do_knot_for_input_second(input);

    assert_eq!(
        "a2582a3a0e66e6e86e3812dcb672a272".to_owned(),
        my_knot_hasher.get_dense_hash()
    );
}

#[test]
pub fn test_day10_knot_hash_second_sample2() {
    let input: &str = "AoC 2017";
    let mut my_knot_hasher: KnotHasher = KnotHasher::new();
    my_knot_hasher.do_knot_for_input_second(input);

    assert_eq!(
        "33efeb34ea91902bb2f59c9920caa6cd".to_owned(),
        my_knot_hasher.get_dense_hash()
    );
}


#[test]
pub fn test_day10_knot_hash_second_sample3() {
    let input: &str = "1,2,3";
    let mut my_knot_hasher: KnotHasher = KnotHasher::new();
    my_knot_hasher.do_knot_for_input_second(input);

    assert_eq!(
        "3efbe78a8d82f29979031a4aa0b16a9d".to_owned(),
        my_knot_hasher.get_dense_hash()
    );
}


#[test]
pub fn test_day10_knot_hash_second_sample4() {
    let input: &str = "1,2,4";
    let mut my_knot_hasher: KnotHasher = KnotHasher::new();
    my_knot_hasher.do_knot_for_input_second(input);

    assert_eq!(
        "63960835bcdc130f0b66d7ff4f6a5a8e".to_owned(),
        my_knot_hasher.get_dense_hash()
    );
}
