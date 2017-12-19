extern crate day18_parser;

use self::day18_parser::{do_parse_line, Action, Value};

use std::collections::HashMap;
use std::process;

#[derive(Debug, Clone)]
pub struct Instruction {
    action: Action,
    register: String,
    arg: Option<Value>,
}

impl Instruction {
    pub fn new(data: (Action, &str, Option<Value>)) -> Self {
        Instruction {
            action: data.0,
            register: data.1.into(),
            arg: data.2,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|a| Instruction::new(do_parse_line(a)))
        .collect()
}

pub fn exec_instructions_first(instructions: Vec<Instruction>, is_first: bool) {
    let mut index: usize = 0;
    let mut last_sound: i64 = 0;
    let mut rcv_sound: i64 = 0;
    let mut registers: HashMap<String, i64> = HashMap::new();

    while index < instructions.len() {
        let instruction = &instructions[index];
        match instruction.action {
            Action::Sound => exec_sound(instruction, &registers, &mut last_sound),
            Action::Set => exec_set(instruction, &mut registers),
            Action::Add => exec_add(instruction, &mut registers),
            Action::Mul => exec_mul(instruction, &mut registers),
            Action::Modulo => exec_mod(instruction, &mut registers),
            Action::Receive => exec_recover(
                instruction,
                &registers,
                &mut rcv_sound,
                &last_sound,
                &is_first,
            ),
            Action::JumpGreaterZero => exec_jump(instruction, &mut registers, &mut index),
        }

        index += 1;
    }
}

pub fn exec_sound(i: &Instruction, registers: &HashMap<String, i64>, last_sound: &mut i64) {
    *last_sound = *registers.get(&i.register).unwrap();
}

pub fn exec_set(i: &Instruction, registers: &mut HashMap<String, i64>) {
    let val = i.arg.clone().unwrap().get_val(registers);
    if let Some(_) = registers.insert(i.register.clone(), val) {
        let val_reg = registers.get_mut(&i.register).unwrap();
        *val_reg = val;
    }
}

pub fn exec_add(i: &Instruction, registers: &mut HashMap<String, i64>) {
    let val = i.arg.clone().unwrap().get_val(registers);
    let val_reg_opt = registers.get_mut(&i.register);
    match val_reg_opt {
        Some(val_reg) => {
            let new_val = *val_reg as i64 + val;
            *val_reg = new_val;
        }
        _ => (),
    }
}

pub fn exec_mul(i: &Instruction, registers: &mut HashMap<String, i64>) {
    let val = i.arg.clone().unwrap().get_val(registers);
    let val_reg_opt = registers.get_mut(&i.register);
    match val_reg_opt {
        Some(val_reg) => {
            let new_val = *val_reg as i64 * val;
            *val_reg = new_val;
        }
        _ => (),
    }
}

pub fn exec_mod(i: &Instruction, registers: &mut HashMap<String, i64>) {
    let val = i.arg.clone().unwrap().get_val(registers);
    let val_reg_opt = registers.get_mut(&i.register);
    match val_reg_opt {
        Some(val_reg) => {
            let new_val = *val_reg as i64 % val;
            *val_reg = new_val;
        }
        _ => (),
    }
}

pub fn exec_recover(
    i: &Instruction,
    registers: &HashMap<String, i64>,
    rcv_sound: &mut i64,
    last_played: &i64,
    is_first: &bool,
) {
    let val = *registers.get(&i.register).unwrap();
    if val != 0 {
        *rcv_sound = *last_played;
        if *is_first {
            println!("Recover sound: {}, val was {}", rcv_sound, val);
            process::exit(0);
        }
    }
}

pub fn exec_jump(i: &Instruction, registers: &mut HashMap<String, i64>, index: &mut usize) {
    let val = i.arg.clone().unwrap().get_val(registers);
    let val_reg_opt = registers.get(&i.register.clone());
    match val_reg_opt {
        Some(val_reg) => if *val_reg > 0 {
            let new_index = *index as i64 + val - 1;
            *index = new_index as usize;
        },
        _ => (),
    }
}

pub fn resolve_part_one(input: &str) {
    let list: Vec<Instruction> = parse_input(input);
    exec_instructions_first(list, true);
}


pub fn resolve_part_two(input: &str) {
    let list_one: Vec<Instruction> = parse_input(input);
    let list_two: Vec<Instruction> = list_one.clone();

    let mut count_send_one: i64 = 0;

    let mut index_one: usize = 0;
    let mut index_two: usize = 0;

    let mut buffer_one: Vec<i64> = Vec::new();
    let mut buffer_two: Vec<i64> = Vec::new();

    let mut registers_one: HashMap<String, i64> = HashMap::new();
    let mut registers_two: HashMap<String, i64> = HashMap::new();

    let mut lock_one: bool = false;
    let mut lock_two: bool = false;

    registers_one.insert("p".into(), 0);
    registers_two.insert("p".into(), 1);

    while index_one < list_one.len() && index_two < list_two.len() {

        while !lock_one {
            let instruction_one = &list_one[index_one];
            handle_instruction(
                instruction_one,
                &mut index_one,
                &mut buffer_one,
                &mut buffer_two,
                &mut registers_one,
                false,
                &mut count_send_one,
                &mut lock_one,
            );
            index_one += 1;
        }

        while !lock_two {
            let instruction_two = &list_two[index_two];
            handle_instruction(
                instruction_two,
                &mut index_two,
                &mut buffer_two,
                &mut buffer_one,
                &mut registers_two,
                true,
                &mut count_send_one,
                &mut lock_two,
            );
            index_two += 1;
        }
    }

    println!("reached end");
}

pub fn handle_instruction(
    instruction: &Instruction,
    index: &mut usize,
    own_buffer: &mut Vec<i64>,
    other_buffer: &mut Vec<i64>,
    registers: &mut HashMap<String, i64>,
    is_prog_one: bool,
    count_send_one: &mut i64,
    lock_prog: &mut bool,
) {
    match instruction.action {
        Action::Sound => exec_send(
            instruction,
            registers,
            own_buffer,
            is_prog_one,
            count_send_one,
        ),
        Action::Set => exec_set(instruction, registers),
        Action::Add => exec_add(instruction, registers),
        Action::Mul => exec_mul(instruction, registers),
        Action::Modulo => exec_mod(instruction, registers),
        Action::Receive => exec_receive_two(instruction, registers, other_buffer, lock_prog),
        Action::JumpGreaterZero => exec_jump(instruction, registers, index),
    }
}

pub fn exec_send(
    instruction: &Instruction,
    registers: &HashMap<String, i64>,
    own_buffer: &mut Vec<i64>,
    is_prog_one: bool,
    count_send_one: &mut i64,
) {
    let val = *registers.get(&instruction.register).unwrap();
    if is_prog_one {
        *count_send_one += 1;
        println!("Prog one sent a data, count: {}", count_send_one);
    }
    own_buffer.push(val);
}

pub fn exec_receive_two(
    instruction: &Instruction,
    registers: &mut HashMap<String, i64>,
    other_buffer: &mut Vec<i64>,
    lock_prog: &mut bool,
) {
    if other_buffer.len() <= 0 {
        *lock_prog = true;
    } else {
        *lock_prog = false;
        let val = other_buffer[0].clone();
        other_buffer.remove(0);

        if let Some(_) = registers.insert(instruction.register.clone(), val) {
            let val_reg = registers.get_mut(&instruction.register).unwrap();
            *val_reg = val;
        }
    }
}
