use itertools::Itertools;
use std::collections::HashSet;

fn run(instructions: &Vec<(&str, i64)>) -> Result<i64, i64> {
    let mut accumulator: i64 = 0;
    let mut executed_instrs: HashSet<i64> = HashSet::new();
    let mut pointer: i64 = 0;
    let exit_ptr: i64 = instructions.len() as i64;
    loop {
        if executed_instrs.contains(&pointer) {
            println!("Loop detected on instruction {}.", pointer);
            return Err(accumulator);
        }
        if pointer == exit_ptr {
            println!("Program completed.");
            return Ok(accumulator);
        }
        let (operation, argument) = instructions[pointer as usize];
        executed_instrs.insert(pointer);
        match operation {
            "acc" => {
                accumulator += argument;
                pointer += 1;
            }
            "jmp" => pointer += argument,
            "nop" => pointer += 1,
            _ => panic!(format!("Unkown operation '{}'", operation)),
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = include_str!("../8.in").trim();
    let mut instructions: Vec<(&str, i64)> = Vec::new();
    for instruction in input.lines() {
        let (op, arg) = instruction.split(" ").collect_tuple().unwrap();
        let arg = arg.parse::<i64>().unwrap();
        instructions.push((op, arg));
    }
    println!("Part1: {:?}", run(&instructions));

    for i in 0..instructions.len() {
        let (op, arg) = instructions[i];
        let mut mutated_instructions = instructions.clone();
        if op == "jmp" {
            mutated_instructions[i] = ("nop", arg)
        } else if op == "nop" {
            mutated_instructions[i] = ("jmp", arg)
        }
        if let Ok(result) = run(&mutated_instructions) {
            println!("Part2: {}", result);
            break;
        }
    }
    Ok(())
}
