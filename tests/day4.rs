extern crate advent_of_code_2017;

use advent_of_code_2017::day4::is_passphrase_ok_first;
use advent_of_code_2017::day4::is_passphrase_ok_second;
use advent_of_code_2017::day4::prime_hash;

#[test]
fn test_day4_passphrase_first_sample1() {
    assert_eq!(is_passphrase_ok_first("aa bb cc dd ee"), true);
}

#[test]
fn test_day4_passphrase_first_sample2() {
    assert_eq!(is_passphrase_ok_first("aa bb cc dd aa"), false);
}

#[test]
fn test_day4_passphrase_first_sample3() {
    assert_eq!(is_passphrase_ok_first("aa bb cc dd aaa"), true);
}

#[test]
fn test_day4_anagram_sample1() {
    let val: bool = prime_hash("test") == prime_hash("estt");
    assert_eq!(val, true);
}

#[test]
fn test_day4_anagram_sample2() {
    let val: bool = prime_hash("west") == prime_hash("rstt");
    assert_eq!(val, false);
}

#[test]
fn test_day4_passphrase_second_sample1() {
    assert_eq!(is_passphrase_ok_second("abcde fghij"), true);
}

#[test]
fn test_day4_passphrase_second_sample2() {
    assert_eq!(is_passphrase_ok_second("abcde xyz ecdab"), false);
}

#[test]
fn test_day4_passphrase_second_sample3() {
    assert_eq!(is_passphrase_ok_second("a ab abc abd abf abj"), true);
}

#[test]
fn test_day4_passphrase_second_sample4() {
    assert_eq!(is_passphrase_ok_second("iiii oiii ooii oooi oooo"), true);
}

#[test]
fn test_day4_passphrase_second_sample5() {
    assert_eq!(is_passphrase_ok_second("oiii ioii iioi iiio"), false);
}
