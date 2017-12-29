extern crate advent_of_code_2017;
extern crate day8_parser;

use day8_parser::{Action, Condition, Instruction, parse_instruction};
use advent_of_code_2017::day8::Registers;

#[test]
pub fn test_day8_parser_line_sample1() {

    let line: &str = "b inc 5 if a > 1";
    let instruction: Instruction = parse_instruction(line);

    let target_reg_name: String = "b".into();
    let cond_reg_name: String = "a".into();

    assert_eq!(instruction.reg_name, target_reg_name);
    assert_eq!(instruction.action, Action::Inc);
    assert_eq!(instruction.val, 5);
    assert_eq!(instruction.cond_reg_name, cond_reg_name);
    assert_eq!(instruction.condition, Condition::Gt);
    assert_eq!(instruction.cond_val, 1);

}

#[test]
pub fn test_day8_parser_line_sample2() {

    let line: &str = "a inc 1 if b < 5";
    let instruction: Instruction = parse_instruction(line);

    let target_reg_name: String = "a".into();
    let cond_reg_name: String = "b".into();

    assert_eq!(instruction.reg_name, target_reg_name);
    assert_eq!(instruction.action, Action::Inc);
    assert_eq!(instruction.val, 1);
    assert_eq!(instruction.cond_reg_name, cond_reg_name);
    assert_eq!(instruction.condition, Condition::Lt);
    assert_eq!(instruction.cond_val, 5);

}

#[test]
pub fn test_day8_parser_line_sample3() {

    let line: &str = "c dec -10 if a >= 1";
    let instruction: Instruction = parse_instruction(line);

    let target_reg_name: String = "c".into();
    let cond_reg_name: String = "a".into();

    assert_eq!(instruction.reg_name, target_reg_name);
    assert_eq!(instruction.action, Action::Dec);
    assert_eq!(instruction.val, -10);
    assert_eq!(instruction.cond_reg_name, cond_reg_name);
    assert_eq!(instruction.condition, Condition::Ge);
    assert_eq!(instruction.cond_val, 1);

}

#[test]
pub fn test_day8_parser_line_sample4() {

    let line: &str = "c inc -20 if c == 10";
    let instruction: Instruction = parse_instruction(line);

    let target_reg_name: String = "c".into();
    let cond_reg_name: String = "c".into();

    assert_eq!(instruction.reg_name, target_reg_name);
    assert_eq!(instruction.action, Action::Inc);
    assert_eq!(instruction.val, -20);
    assert_eq!(instruction.cond_reg_name, cond_reg_name);
    assert_eq!(instruction.condition, Condition::Eq);
    assert_eq!(instruction.cond_val, 10);

}

#[test]
pub fn test_day8_registers_get_set() {

    let mut registers: Registers = Registers::new();
    registers.set_value("b".into(), 1);
    registers.set_value("ab".into(), -1);

    let val_a = registers.get_value("a".into());
    let val_b = registers.get_value("b".into());
    let val_ab = registers.get_value("ab".into());

    assert_eq!(val_a, 0);
    assert_eq!(val_b, 1);
    assert_eq!(val_ab, -1);

}

#[test]
pub fn test_day8_registers_conditions() {

    let mut registers: Registers = Registers::new();
    registers.set_value("a".into(), -1);
    registers.set_value("b".into(), 1);

    let check_gt_true = registers.check_condition("b", &Condition::Gt, &0);
    let check_gt_false = registers.check_condition("a", &Condition::Gt, &0);

    let check_ge_true = registers.check_condition("b", &Condition::Ge, &1);
    let check_ge_false = registers.check_condition("a", &Condition::Ge, &1);

    let check_eq_true = registers.check_condition("b", &Condition::Eq, &1);
    let check_eq_false = registers.check_condition("a", &Condition::Eq, &1);

    let check_ne_true = registers.check_condition("a", &Condition::Ne, &1);
    let check_ne_false = registers.check_condition("b", &Condition::Ne, &1);

    let check_lt_true = registers.check_condition("a", &Condition::Lt, &0);
    let check_lt_false = registers.check_condition("b", &Condition::Lt, &0);

    let check_le_true = registers.check_condition("a", &Condition::Le, &-1);
    let check_le_false = registers.check_condition("b", &Condition::Le, &-1);

    assert_eq!(check_gt_true, true);
    assert_eq!(check_ge_true, true);
    assert_eq!(check_eq_true, true);
    assert_eq!(check_ne_true, true);
    assert_eq!(check_lt_true, true);
    assert_eq!(check_le_true, true);

    assert_eq!(check_gt_false, false);
    assert_eq!(check_ge_false, false);
    assert_eq!(check_eq_false, false);
    assert_eq!(check_ne_false, false);
    assert_eq!(check_lt_false, false);
    assert_eq!(check_le_false, false);
}

#[test]
pub fn test_day8_actions() {
    let mut registers: Registers = Registers::new();
    registers.set_value("b", 1);

    registers.do_action("a", &Action::Inc, &3);
    assert_eq!(registers.get_value("a"), 3);

    registers.do_action("b", &Action::Dec, &1);
    assert_eq!(registers.get_value("b"), 0);

    registers.do_action("a", &Action::Dec, &3);
    assert_eq!(registers.get_value("b"), 0);
}

#[test]
pub fn test_day8_instructions() {

    let mut registers: Registers = Registers::new();

    let instr1_input: &str = "a inc 3 if a >= -1";
    let instruction1 = parse_instruction(instr1_input);

    let instr2_input: &str = "b inc 3 if a == 3";
    let instruction2 = parse_instruction(instr2_input);

    let instr3_input: &str = "b inc 3 if a == 100";
    let instruction3 = parse_instruction(instr3_input);

    registers.handle_instruction(&instruction1);
    registers.handle_instruction(&instruction2);
    registers.handle_instruction(&instruction3);

    assert_eq!(registers.get_value("a"), 3);
    assert_eq!(registers.get_value("b"), 3);
}

#[test]
pub fn test_day8_registers_max() {

    let mut registers: Registers = Registers::new();

    registers.set_value("a", 0);
    registers.set_value("b", 10);
    registers.set_value("c", 5);
    registers.set_value("d", -5);
    registers.set_value("e", 2);
    registers.set_value("f", 9);
    registers.set_value("g", -32);

    assert_eq!(registers.get_max_value(), 10);

}