#[macro_use]
extern crate nom;

use nom::{alpha, line_ending, digit};
use std::str;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Action {
    pub value_to_write: u32,
    pub move_left: bool,
    pub next_state: String,
}

#[derive(Debug)]
pub struct State {
    pub name: String,
    pub action_zero: Action,
    pub action_one: Action,
}

#[derive(Debug)]
pub struct Blueprint {
    pub count_max: u32,
    pub start_state: String,
    pub states: HashMap<String, State>,
}

impl Action {
    pub fn new(val: u32, move_left: bool, next: &str) -> Self {
        Action {
            value_to_write: val,
            move_left,
            next_state: next.into(),
        }
    }
}

impl State {
    pub fn new(name: &str, action_z: Action, action_o: Action) -> Self {
        State {
            name: name.into(),
            action_zero: action_z,
            action_one: action_o,
        }
    }
}

impl Blueprint {
    pub fn new(count_max: u32, start_state: &str, states: Vec<State>) -> Self {
        let mut states_hash: HashMap<String, State> = HashMap::new();

        // Not the cleanest way to do this.
        // Maybe using a HashSet and impl Hash for State is a better way ?
        for state in states {
            states_hash.insert(state.name.clone(), state);
        }

        Blueprint {
            count_max,
            start_state: start_state.into(),
            states: states_hash,
        }
    }
}

named!(get_str < & str >, map_res! (alpha, str::from_utf8));

named!(number < u32 >, map_res !( map_res ! (digit, str::from_utf8), str::parse));

named!(get_states < Vec < State >>,
    separated_list_complete!(line_ending, parse_state)
);

named!(parse_state < State >,
    do_parse ! (
        opt!(line_ending) >>
        tag !("In state ") >> name: get_str >> tag ! (":") >> line_ending >>
        action_z: parse_action >> line_ending >>
        action_o: parse_action >>
        (State::new(name, action_z, action_o))
    )
);

named!(parse_action < Action >,
    do_parse ! (
        tag!("  ") >> tag!("If the current value is ") >> alt ! ( tag ! ("0:") | tag ! ("1:") ) >> line_ending >>
        tag!("    ") >> tag!("- Write the value ") >> value_to_write: number >> tag ! (".") >> line_ending >>
        tag!("    ") >> tag!("- Move one slot to the ") >> dir: get_dir >> tag ! (".") >> line_ending >>
        tag!("    ") >> tag!("- Continue with state ") >> next_state: get_str >> tag ! (".") >>
        (Action::new(value_to_write, dir, next_state))
    )
);

named!(parse_blueprint < Blueprint >,
    do_parse ! (
        tag!("Begin in state ") >> start_state: get_str >> tag!(".") >> line_ending >>
        tag!("Perform a diagnostic checksum after ") >> count_max: number >> tag!(" steps.") >> line_ending >> line_ending >>
        states: get_states >>
        (Blueprint::new(count_max, start_state, states))
    )
);

named!(
    dir_tags,
    alt!(
        tag!("left") |
        tag!("right")
    )
);

named!(
    get_dir < bool >,
    map!(map_res!(dir_tags, str::from_utf8), parse_direction)
);

fn parse_direction(input: &str) -> bool {
    match input {
        "left" => true,
        "right" => false,
        _ => unreachable!(),
    }
}

pub fn get_blueprint(input: &str) -> Blueprint {
    parse_blueprint(input.as_bytes()).to_result().unwrap()
}