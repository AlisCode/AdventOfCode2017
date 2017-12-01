extern crate advent_of_code_2017;

use advent_of_code_2017::day1::decode_captcha_iter;
use advent_of_code_2017::day1::decode_captcha_iter_second;

#[test]
fn test_day1_first_iter() {
    assert_eq!(decode_captcha_iter("1122"), 3);
    assert_eq!(decode_captcha_iter("1111"), 4);
    assert_eq!(decode_captcha_iter("1234"), 0);
    assert_eq!(decode_captcha_iter("91212129"), 9);
}

#[test]
fn test_day1_second_iter() {
    assert_eq!(decode_captcha_iter_second("1212"), 6);
    assert_eq!(decode_captcha_iter_second("1221"), 0);
    assert_eq!(decode_captcha_iter_second("123425"), 4);
    assert_eq!(decode_captcha_iter_second("123123"), 12);
    assert_eq!(decode_captcha_iter_second("12131415"), 4);
}
