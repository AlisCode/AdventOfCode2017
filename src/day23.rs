extern crate day23_parser;

use day23_parser::{Action, Value, Registers, do_parse_line};

pub fn resolve_part_one(input: &str) -> u32 {
    let mut registers: Registers = Registers::new();
    let instructions: Vec<(Action, Value, Value)> = parse_input(input);
    let mut curr_index: usize = 0;

    let mut count_mul: u32 = 0;

    while curr_index < instructions.len() {
        let instr = &instructions[curr_index];
        handle_action(&instr.0, &instr.1, &instr.2, &mut registers, &mut curr_index);

        if instr.0 == Action::Mul { count_mul += 1; }

        curr_index += 1;
        if instr.0 == Action::Jnz && instr.1.get_val(&mut registers) != 0 {
            curr_index -= 1;
        }
    }

    count_mul
}

pub fn resolve_part_two(input: &str) -> usize {
    let mut registers: Registers = Registers::new();
    registers.set_val("a", 1);

    // The 8 first actions make b and c equal to the min and max value that we're going to iterate through
    let mut instructions: Vec<(Action, Value, Value)> = parse_input(input);
    instructions = instructions.into_iter().take(8).collect();

    let mut curr_index: usize = 0;

    while curr_index < instructions.len() {
        let instr = &instructions[curr_index];
        handle_action(&instr.0, &instr.1, &instr.2, &mut registers, &mut curr_index);

        curr_index += 1;
        if instr.0 == Action::Jnz && instr.1.get_val(&mut registers) != 0 {
            curr_index -= 1;
        }
    }

    // We get the min and max value
    let min = registers.get_val("b");
    let max = registers.get_val("c");

    // BLACK MAGIC!
    (min..max + 1)
        .filter(|&n| (n - min) % 17 == 0) // I do think 17 is input-dependant
        .filter(|&n| is_not_prime(n))
        .count()
}

fn is_not_prime(n: i32) -> bool {
    (2..).take_while(|&d| d * d <= n).any(|d| n % d == 0)
}


pub fn parse_input(input: &str) -> Vec<(Action, Value, Value)> {
    input.lines().map(|a| {
        do_parse_line(a)
    }).collect()
}

pub fn handle_action(
    act: &Action,
    val_one: &Value,
    val_two: &Value,
    regs: &mut Registers,
    index: &mut usize,
) {
    match act {
        &Action::Set => {
            let val = val_two.get_val(regs);
            match val_one {
                &Value::Register(ref reg) => {
                    regs.set_val(&reg, val);
                }
                _ => unreachable!(),
            }
        }
        &Action::Mul => {
            let val = val_one.get_val(regs);
            let other_val = val_two.get_val(regs);
            match val_one {
                &Value::Register(ref reg) => {
                    regs.set_val(&reg, val * other_val);
                }
                _ => unreachable!(),
            }
        }
        &Action::Sub => {
            let val = val_one.get_val(regs);
            let other_val = val_two.get_val(regs);
            match val_one {
                &Value::Register(ref reg) => {
                    regs.set_val(&reg, val - other_val);
                }
                _ => unreachable!(),
            }
        }
        &Action::Jnz => {
            let val = val_one.get_val(regs);
            let jmp_val = val_two.get_val(regs);
            if val != 0 {
                let new_index: i32 = *index as i32 + jmp_val;
                *index = new_index as usize;
            }
        }
    }
}