#[macro_use]
extern crate nom;

use std::str;
use nom::{alpha, digit};

named!(
    number<u32>,
    map_res!(map_res!(digit, str::from_utf8), str::parse)
);

named!(
    spin_infos<usize>,
    do_parse!(tag!("s") >> size: number >> (size as usize))
);

named!(
    exchange_infos<(usize, usize)>,
    do_parse!(
        tag!("x") >> pos_a: number >> tag!("/") >> pos_b: number
            >> (pos_a as usize, pos_b as usize)
    )
);

named!(
    partner_infos<(char, char)>,
    do_parse!(
        tag!("p") >> char_a: alpha >> tag!("/") >> char_b: alpha
            >> (char_a[0] as char, char_b[0] as char)
    )
);

pub fn get_partner_infos(input: &str) -> (char, char) {
    partner_infos(input.as_bytes()).to_result().unwrap()
}

pub fn get_exchange_infos(input: &str) -> (usize, usize) {
    exchange_infos(input.as_bytes()).to_result().unwrap()
}

pub fn get_spin_infos(input: &str) -> usize {
    spin_infos(input.as_bytes()).to_result().unwrap()
}
