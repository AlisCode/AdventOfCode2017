pub fn decode_captcha_iter(captcha: &str) -> u32 {
    captcha
        .chars()
        .zip(captcha.chars().cycle().skip(1))
        .filter_map(|(a, b)| if a == b { a.to_digit(10) } else { None })
        .sum()
}

pub fn decode_captcha_iter_second(captcha: &str) -> u32 {
    captcha
        .chars()
        .zip(captcha.chars().cycle().skip(captcha.len() / 2))
        .filter_map(|(a, b)| if a == b { a.to_digit(10) } else { None })
        .sum()
}
