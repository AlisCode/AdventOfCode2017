extern crate advent_of_code_2017;

use advent_of_code_2017::day3::SpiralMemoryCaseSequence;

#[test]
fn test_day3_manhattan_sample1() {
    let item = SpiralMemoryCaseSequence::new().next().unwrap();
    assert_eq!(item.get_manhattan_distance(), 0);
}

#[test]
fn test_day3_manhattan_sample2() {
    let index: usize = 12;
    let item = SpiralMemoryCaseSequence::new()
        .skip(index - 1)
        .next()
        .unwrap();
    assert_eq!(item.get_manhattan_distance(), 3);
}

#[test]
fn test_day3_manhattan_sample3() {
    let index: usize = 23;
    let item = SpiralMemoryCaseSequence::new()
        .skip(index - 1)
        .next()
        .unwrap();
    assert_eq!(item.get_manhattan_distance(), 2);
}

#[test]
fn test_day3_value_sample1() {
    let index: usize = 5;
    let item = SpiralMemoryCaseSequence::new()
        .skip(index - 1)
        .next()
        .unwrap();
    assert_eq!(item.value, 5);
}

#[test]
fn test_day3_value_sample2() {
    let index: usize = 2;
    let item = SpiralMemoryCaseSequence::new()
        .skip(index - 1)
        .next()
        .unwrap();
    assert_eq!(item.value, 1);
}

#[test]
fn test_day3_value_sample3() {
    let index: usize = 4;
    let item = SpiralMemoryCaseSequence::new()
        .skip(index - 1)
        .next()
        .unwrap();
    assert_eq!(item.value, 4);
}

#[test]
fn test_day3_value_sample4() {
    let index: usize = 3;
    let item = SpiralMemoryCaseSequence::new()
        .skip(index - 1)
        .next()
        .unwrap();
    assert_eq!(item.value, 2);
}