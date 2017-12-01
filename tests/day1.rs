extern crate AdventOfCode2017;

use AdventOfCode2017::day1::decode_captcha_iter;

#[test]
fn test_day1_first_iter() {
    assert_eq!(decode_captcha_iter("1122"), 3);
    assert_eq!(decode_captcha_iter("1111"), 4);
    assert_eq!(decode_captcha_iter("1234"), 0);
    assert_eq!(decode_captcha_iter("91212129"), 9);
}
