extern crate day25_parser;
extern crate advent_of_code_2017;

use day25_parser::{Blueprint, Action, State, get_blueprint};
use advent_of_code_2017::day25::Tape;

#[test]
pub fn test_day25_parser_blueprint()
{
    let input: &str = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";

    let string_a: String = "A".into();
    let string_b: String = "B".into();

    let blueprint: Blueprint = get_blueprint(input);
    assert_eq!(blueprint.start_state, string_a);
    assert_eq!(blueprint.count_max, 6);

    let state_a: &State = &blueprint.states[0];
    let state_b: &State = &blueprint.states[1];

    assert_eq!(state_a.name, string_a);
    assert_eq!(state_b.name, string_b);

    let action_z_a: &Action = &state_a.action_zero;
    assert_eq!(action_z_a.value_to_write, 1);
    assert_eq!(action_z_a.move_left, false);
    assert_eq!(action_z_a.next_state, string_b);

    let action_o_a: &Action = &state_a.action_one;
    assert_eq!(action_o_a.value_to_write, 0);
    assert_eq!(action_o_a.move_left, true);
    assert_eq!(action_o_a.next_state, string_b);

    let action_z_b: &Action = &state_b.action_zero;
    assert_eq!(action_z_b.value_to_write, 1);
    assert_eq!(action_z_b.move_left, true);
    assert_eq!(action_z_b.next_state, string_a);

    let action_o_b: &Action = &state_b.action_one;
    assert_eq!(action_o_b.value_to_write, 1);
    assert_eq!(action_o_b.move_left, false);
    assert_eq!(action_o_b.next_state, string_a);
}

#[test]
pub fn test_day25_tape_checksum() {
    let mut tape: Tape = Tape::new();

    tape.records.insert(0, 1);
    tape.records.insert(-1, 1);
    tape.records.insert(-2, 0);
    tape.records.insert(1, 0);
    tape.records.insert(2, 1);

    assert_eq!(tape.checksum(), 3);
}