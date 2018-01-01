extern crate advent_of_code_2017;
extern crate day23_parser;

use day23_parser::{do_parse_line, Action, Value};
use advent_of_code_2017::day23::{parse_input, Registers};

#[test]
fn test_day23_parser_sample1() {

    let input: &str = "set x 10";
    let (act, reg, val) = do_parse_line(input);

    assert_eq!(act, Action::Set);
    assert_eq!(reg, "x");
    assert_eq!(val, Value::Number(10));
}

#[test]
fn test_day23_parser_sample2() {

    let input: &str = "mul ab -10";
    let (act, reg, val) = do_parse_line(input);

    assert_eq!(act, Action::Mul);
    assert_eq!(reg, "ab");
    assert_eq!(val, Value::Number(-10));
}

#[test]
fn test_day23_parser_sample3() {

    let input: &str = "jnz A y";
    let (act, reg, val) = do_parse_line(input);

    assert_eq!(act, Action::Jnz);
    assert_eq!(reg, "A");
    assert_eq!(val, Value::Register("y".into()));
}

#[test]
fn test_day23_parse_input_sample1() {
    let input: &str = "set a y\nmul b 10\njnz x -3";

    let instructions = parse_input(input);

    assert_eq!(
        instructions,
        vec![
            (Action::Set, "a", Value::Register("y".into())),
            (Action::Mul, "b", Value::Number(10)),
            (Action::Jnz, "x", Value::Number(-3)),
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