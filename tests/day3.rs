extern crate advent_of_code_2017;

use advent_of_code_2017::day3::SpiralMemoryCase;

#[test]
fn test_day3_spirale_sample1()
{
    let item = SpiralMemoryCase::new_seq_generator();
    assert_eq!(item.get_manhattan_distance(), 0);
}

#[test]
fn test_day3_spirale_sample2()
{
    let index: usize = 12;
    let item = SpiralMemoryCase::new_seq_generator().skip(index-1).next().unwrap();
    assert_eq!(item.get_manhattan_distance(), 3);
}

#[test]
fn test_day3_spirale_sample3()
{
    let index: usize = 23;
    let item = SpiralMemoryCase::new_seq_generator().skip(index-1).next().unwrap();
    assert_eq!(item.get_manhattan_distance(), 2);
}

#[test]
fn test_day3_spirale_sample4()
{
    let index: usize = 1024;
    let item = SpiralMemoryCase::new_seq_generator().skip(index-1).next().unwrap();
    assert_eq!(item.get_manhattan_distance(), 31);
}