use aoc_2020_common::common::load_file;
use regex::Regex;

#[derive(Debug, Clone)]
enum Operation {
    NOP,
    ACC,
    JMP,
}

#[derive(Debug)]
struct MachineState {
    accumulator: i64,
    program_counter: i64,
    finished: bool,
}

fn parse_instructions(input: &str) -> Vec<(Operation, i64)> {
    let instruction_re: Regex = Regex::new(r#"([a-z]{3}) ([+\-])(\d+)"#).unwrap();

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
                "-" => -1 * cap[3].parse::<i64>().unwrap(),
                _ => panic!("Invalid sign before argument"),
            };

            (operation, argument)
        })
        .collect()
}

fn run_machine_instructions(instructions: &Vec<(Operation, i64)>) -> MachineState {
    let mut st = MachineState { accumulator: 0, program_counter: 0, finished: false };
    let mut visited_instructions: Vec<usize> = Vec::new();

    while !st.finished {
        let pc = st.program_counter as usize;

        // Terminate before running an instruction a second time
        if visited_instructions.contains(&pc) {
            break;
        }

        // Mark instruction as visited
        visited_instructions.push(pc);

        // Execute instruction
        match &instructions[pc] {
            (Operation::NOP, _) => st.program_counter += 1,
            (Operation::ACC, n) => { st.accumulator += n; st.program_counter += 1 }
            (Operation::JMP, n) => st.program_counter += n,
        }

        // Check if we're at the end of our program
        if st.program_counter >= instructions.len() as i64 {
            st.finished = true;
        }
    }

    st
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
    for (index, instr) in instructions.iter().enumerate() {
        let mut modified_instructions = instructions.to_vec();
        modified_instructions[index] = match instr {
            (Operation::NOP, n) => (Operation::JMP, *n),
            (Operation::JMP, n) => (Operation::NOP, *n),
            (Operation::ACC, n) => (Operation::ACC, *n),
        };

        let st = run_machine_instructions(&modified_instructions);

        if st.finished {
            println!("{}", st.accumulator);
        }
    }
}
