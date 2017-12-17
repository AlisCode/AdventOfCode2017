extern crate advent_of_code_2017;

use advent_of_code_2017::day16::{exchange, partner, spin};

/// For example, s3 on abcde produces cdeab
#[test]
pub fn test_day16_spin_sample1() {
    let input: String = "abcde".into();
    let wanted: String = "cdeab".into();
    assert_eq!(spin(&input, 3), wanted);
}

/// s1, a spin of size 1: eabcd.
#[test]
pub fn test_day16_spin_sample2() {
    let input: String = "abcde".into();
    let wanted: String = "eabcd".into();
    assert_eq!(spin(&input, 1), wanted);
}

/// x3/4, swapping the last two programs: eabdc.
#[test]
pub fn test_day16_exchange_sample1() {
    let input: String = "eabcd".into();
    let wanted: String = "eabdc".into();
    assert_eq!(exchange(&input, 3, 4), wanted);
}

/// x3/4, swapping the last two programs: eabdc.
#[test]
pub fn test_day16_partner_sample1() {
    let input: String = "eabdc".into();
    let wanted: String = "baedc".into();
    assert_eq!(partner(&input, 'e', 'b'), wanted);
}
