use aoc_2020_common::common::load_file;
use regex::Regex;
use std::collections::HashSet;


#[derive(Debug, Clone)]
enum Operation {
    NOP,
    ACC,
    JMP,
}

type Instruction = (Operation, i64);

#[derive(Debug)]
struct MachineState {
    accumulator: i64,
    instruction_pointer: i64,
    finished: bool,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {

    instruction_re
        .captures_iter(&input)
        .map(|cap| {
            let operation = match &cap[1] {
                "nop" => Operation::NOP,
                "acc" => Operation::ACC,
                "jmp" => Operation::JMP,
                _ => panic!("Invalid instruction")
            };
            let argument = match &cap[2] {
                "+" => cap[3].parse::<i64>().unwrap(),
                "-" => -cap[3].parse::<i64>().unwrap(),
                _ => panic!("Invalid sign before argument"),
            };

            (operation, argument)
        })
        .collect()
}

fn run_machine_instructions(instructions: &[Instruction]) -> MachineState {
    let mut state = MachineState { accumulator: 0, instruction_pointer: 0, finished: false };
    let mut visited_instructions: HashSet<usize> = HashSet::new();

    while !state.finished {
        let ip = state.instruction_pointer as usize;

        // Terminate before running an instruction a second time
        if visited_instructions.contains(&ip) {
            break;
        }

        // Mark instruction as visited
        visited_instructions.insert(ip);

        // Execute instruction
        match &instructions[ip] {
            (Operation::NOP, _) => state.instruction_pointer += 1,
            (Operation::ACC, n) => { state.accumulator += n; state.instruction_pointer += 1 }
            (Operation::JMP, n) => state.instruction_pointer += n,
        }

        // Check if we're at the end of our program
        if state.instruction_pointer >= instructions.len() as i64 {
            state.finished = true;
        }
    }

    state
}

fn main() {
    let input = load_file("./input/1.txt");
    let instructions = parse_instructions(&input);

    // Part 1 answer
    let st = run_machine_instructions(&instructions);
    println!("{}", st.accumulator);

    // Part 2 answer: for every instruction, try to change NOP to JMP (or vice versa),
    // After the change, check if it does finish execution (ie, it reaches the end of the file).
    // If it does, then the answer is in our accumulator.
    for (index, instruction) in instructions.iter().enumerate() {
        let mut modified_instructions = instructions.to_vec();
        modified_instructions[index] = match instruction {
            (Operation::NOP, n) => (Operation::JMP, *n),
            (Operation::JMP, n) => (Operation::NOP, *n),
            (Operation::ACC, _) => continue,
        };

        let st = run_machine_instructions(&modified_instructions);

        if st.finished {
            println!("{}", st.accumulator);
        }
    }
}
