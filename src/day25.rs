use day25_parser::{Blueprint, Action, State, get_blueprint};

use std::collections::HashMap;

pub struct Tape {
    pub curr_index: i32,
    pub curr_state: String,
    pub records: HashMap<i32, u32>,
}

impl Tape {
    pub fn new() -> Self {
        Tape {
            curr_index: 0,
            curr_state: "".into(),
            records: HashMap::new(),
        }
    }

    pub fn tick(&mut self, blueprint: &Blueprint) {}

    pub fn checksum(&self) -> u32 {
        self.records.values().sum()
    }
}

pub fn resolve_part_one(input: &str) {
    let blueprint: Blueprint = get_blueprint(input);
    let mut tape: Tape = Tape::new();

    for _ in 0..blueprint.count_max {
        tape.tick(&blueprint);
    }

    println!("{:?}", blueprint);
}

