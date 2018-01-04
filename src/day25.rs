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

    pub fn tick(&mut self, blueprint: &Blueprint) {
        if !self.records.contains_key(&self.curr_index) {
            self.records.insert(self.curr_index.clone(), 0);
        }

        let state: &State = blueprint.states.get(&self.curr_state).unwrap();

        let action = match self.records.get(&self.curr_index) {
            Some(val) => *val,
            None => unreachable!(),
        };

        match action {
            0 => { self.handle_action(&state.action_zero); }
            1 => { self.handle_action(&state.action_one); }
            _ => unreachable!(),
        }
    }

    pub fn handle_action(&mut self, action: &Action) {

        // Writes the value
        let ref_current_record: &mut u32 = self.records.get_mut(&self.curr_index).unwrap();
        *ref_current_record = action.value_to_write;

        // Moves the index
        if action.move_left {
            self.curr_index -= 1;
        } else {
            self.curr_index += 1;
        }

        // Changes the next state
        self.curr_state = action.next_state.clone();
    }

    /// Computes the checksum of the tape
    pub fn checksum(&self) -> u32 {
        self.records.values().sum()
    }
}

/// Resolves the part one
/// NB: Run in release mode. Cargo run takes a looong time to compute this :)
pub fn resolve_part_one(input: &str) -> u32 {
    let blueprint: Blueprint = get_blueprint(input);
    let mut tape: Tape = Tape::new();
    tape.curr_state = blueprint.start_state.clone();

    for _ in 0..blueprint.count_max {
        tape.tick(&blueprint);
    }

    tape.checksum()
}

