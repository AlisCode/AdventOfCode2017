extern crate day8_parser;

use std::collections::HashMap;
use self::day8_parser::{Instruction, Action, Condition, parse_instruction};

pub struct Registers {
    values: HashMap<String, i32>,
}

impl Registers {
    pub fn new() -> Self {
        Registers { values: HashMap::new() }
    }

    pub fn get_value(&self, key: &str) -> i32 {
        match self.values.get(key) {
            Some(val) => *val,
            None => 0,
        }
    }

    pub fn set_value(&mut self, key: &str, new_val: i32) {
        match self.values.get_mut(key) {
            Some(val) => {
                *val = new_val;
                return;
            }
            None => (),
        }

        self.values.insert(key.into(), new_val);
    }

    pub fn handle_instruction(&mut self, instruction: &Instruction) {
        if self.check_condition(
            &instruction.cond_reg_name,
            &instruction.condition,
            &instruction.cond_val,
        )
        {
            self.do_action(&instruction.reg_name, &instruction.action, &instruction.val);
        }
    }


    pub fn check_condition(&mut self, cond_reg: &str, cond: &Condition, cond_val: &i32) -> bool {
        let val = self.get_value(cond_reg);

        match cond {
            &Condition::Eq => val == *cond_val,
            &Condition::Ge => val >= *cond_val,
            &Condition::Gt => val > *cond_val,
            &Condition::Le => val <= *cond_val,
            &Condition::Lt => val < *cond_val,
            &Condition::Ne => val != *cond_val,
        }
    }

    pub fn do_action(&mut self, reg: &str, act: &Action, act_val: &i32) {
        let mut val = self.get_value(reg);

        match act {
            &Action::Inc => val += act_val,
            &Action::Dec => val -= act_val,
        }

        self.set_value(reg, val);
    }

    pub fn get_max_value(&self) -> i32 {
        *self.values.values().max().unwrap()
    }
}

pub fn parse_input(input: &str) -> Vec<Instruction> {

    input.lines().map(|a| parse_instruction(a)).collect()

}

pub fn resolve_part_one(input: &str) -> i32 {

    let mut registers: Registers = Registers::new();

    let instructions: Vec<Instruction> = parse_input(input);

    instructions.iter().for_each(
        |a| registers.handle_instruction(a),
    );

    registers.get_max_value()

}

pub fn resolve_part_two(input: &str) -> i32 {

    let mut registers: Registers = Registers::new();

    let instructions: Vec<Instruction> = parse_input(input);

    instructions
        .iter()
        .map(|a| {
            registers.handle_instruction(a);
            registers.get_max_value()
        })
        .max()
        .unwrap()
}