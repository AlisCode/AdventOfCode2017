extern crate day16_parser;

use self::day16_parser::{get_exchange_infos, get_partner_infos, get_spin_infos};

pub fn spin(input: &String, size: usize) -> String {
    let mut to_return: String = "".into();
    let mut iter = input.chars().cycle().skip(input.len() - size);

    for _ in 0..input.len() {
        to_return.push(iter.next().unwrap());
    }
    to_return
}

pub fn exchange(input: &String, pos_a: usize, pos_b: usize) -> String {
    let mut bytes: Vec<u8> = input.clone().into_bytes();

    let char_a = bytes[pos_a];
    let char_b = bytes[pos_b];

    bytes[pos_a] = char_b;
    bytes[pos_b] = char_a;

    String::from_utf8(bytes.to_owned()).unwrap()
}

pub fn partner(input: &String, char_a: char, char_b: char) -> String {
    let (index_a, _) = input
        .char_indices()
        .filter(|&(_, b)| b == char_a)
        .next()
        .unwrap();

    let (index_b, _) = input
        .char_indices()
        .filter(|&(_, b)| b == char_b)
        .next()
        .unwrap();

    exchange(input, index_a, index_b)
}

pub fn parse_input(input: &str, base: &str) -> String {
    let mut to_return: String = base.into();
    input.split(",").for_each(|a| {
        to_return = handle_line(&to_return, a);
    });

    to_return
}

fn handle_line(input: &String, line: &str) -> String {
    let action: char = line.chars().nth(0).unwrap();

    let result = match action {
        's' => spin(input, get_spin_infos(line)),
        'x' => {
            let (a, b) = get_exchange_infos(line);
            exchange(input, a, b)
        }
        'p' => {
            let (a, b) = get_partner_infos(line);
            partner(input, a, b)
        }
        _ => unreachable!(),
    };

    result
}

pub fn resolve_part_two(input: &str, base: &str) -> String {
    let list = get_cycle(input, base);

    let cycle_size = list.len();
    let index_we_want = 1_000_000_000 % cycle_size;

    list[index_we_want].clone()
}

fn get_cycle(input: &str, base: &str) -> Vec<String> {
    let mut list: Vec<String> = Vec::new();
    let mut to_return: String = base.into();

    list.push(to_return.clone());

    for _ in 0.. {
        to_return = parse_input(input, &to_return);

        if list.contains(&to_return) {
            break;
        }
        list.push(to_return.clone());
    }

    list
}
