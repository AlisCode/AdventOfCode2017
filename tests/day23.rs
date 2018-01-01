extern crate advent_of_code_2017;
extern crate day23_parser;

use day23_parser::{do_parse_line, Action, Value, Registers};
use advent_of_code_2017::day23::{parse_input, handle_action};

#[test]
fn test_day23_parser_sample1() {
    let input: &str = "set x 10";
    let (act, val_one, val_two) = do_parse_line(input);

    assert_eq!(act, Action::Set);
    assert_eq!(val_one, Value::Register("x".into()));
    assert_eq!(val_two, Value::Number(10));
}

#[test]
fn test_day23_parser_sample2() {
    let input: &str = "mul ab -10";
    let (act, val_one, val_two) = do_parse_line(input);

    assert_eq!(act, Action::Mul);
    assert_eq!(val_one, Value::Register("ab".into()));
    assert_eq!(val_two, Value::Number(-10));
}

#[test]
fn test_day23_parser_sample3() {
    let input: &str = "jnz A y";
    let (act, val_one, val_two) = do_parse_line(input);

    assert_eq!(act, Action::Jnz);
    assert_eq!(val_one, Value::Register("A".into()));
    assert_eq!(val_two, Value::Register("y".into()));
}

#[test]
fn test_day23_parse_input_sample1() {
    let input: &str = "set a y\nmul b 10\njnz x -3";

    let instructions = parse_input(input);

    assert_eq!(
        instructions,
        vec![
            (Action::Set, Value::Register("a".into()), Value::Register("y".into())),
            (Action::Mul, Value::Register("b".into()), Value::Number(10)),
            (Action::Jnz, Value::Register("x".into()), Value::Number(-3)),
        ]
    );
}

#[test]
fn test_day23_registers_get() {
    let mut registers: Registers = Registers::new();
    registers.values.insert("b".into(), 1);

    assert_eq!(registers.get_val("a"), 0);
    assert_eq!(registers.get_val("b"), 1);
}

#[test]
fn test_day23_registers_set() {
    let mut registers: Registers = Registers::new();
    registers.set_val("a", 1);

    assert_eq!(registers.get_val("a"), 1);
}

#[test]
fn test_day23_actions_sample1() {
    let mut registers: Registers = Registers::new();
    registers.set_val("a", 2);

    let action: Action = Action::Mul;
    let val_one: Value = Value::Register("a".into());
    let val_two: Value = Value::Number(5);
    let mut curr_index: usize = 0;

    handle_action(&action, &val_one, &val_two, &mut registers, &mut curr_index);

    assert_eq!(registers.get_val("a"), 10);
}

#[test]
fn test_day23_actions_sample2() {
    let mut registers: Registers = Registers::new();
    registers.set_val("a", 1);

    let action: Action = Action::Set;
    let val_one: Value = Value::Register("a".into());
    let val_two: Value = Value::Number(5);
    let mut curr_index: usize = 0;

    handle_action(&action, &val_one, &val_two, &mut registers, &mut curr_index);

    assert_eq!(registers.get_val("a"), 5);
}

#[test]
fn test_day23_actions_sample3() {
    let mut registers: Registers = Registers::new();
    registers.set_val("a", 1);

    let action: Action = Action::Sub;
    let val_one: Value = Value::Register("a".into());
    let val_two: Value = Value::Number(2);
    let mut curr_index: usize = 0;

    handle_action(&action, &val_one, &val_two, &mut registers, &mut curr_index);

    assert_eq!(registers.get_val("a"), -1);
}

#[test]
fn test_day23_actions_sample4() {
    let mut registers: Registers = Registers::new();
    registers.set_val("a", 1);

    let action: Action = Action::Jnz;
    let val_one: Value = Value::Register("a".into());
    let val_two: Value = Value::Number(5);
    let mut curr_index: usize = 1;

    handle_action(&action, &val_one, &val_two, &mut registers, &mut curr_index);

    assert_eq!(curr_index, 6);
}

#[test]
fn test_day23_actions_sample5() {
    let mut registers: Registers = Registers::new();
    registers.set_val("a", 1);

    let action: Action = Action::Jnz;
    let val_one: Value = Value::Register("a".into());
    let val_two: Value = Value::Number(-2);
    let mut curr_index: usize = 5;

    handle_action(&action, &val_one, &val_two, &mut registers, &mut curr_index);

    assert_eq!(curr_index, 3);
}