use aoc_2020_common::common::load_file;
use regex::Regex;

#[derive(Debug)]
enum Operation {
    NOP,
    ACC,
    JMP,
}

#[derive(Debug)]
struct MachineState {
    accumulator: i64,
    program_counter: i64,
    running: bool,
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

fn main() {
    let input = load_file("./input/1.txt");
    let instructions = parse_instructions(&input);

    let mut st = MachineState { accumulator: 0, program_counter: 0, running: true };
    let mut visited_instructions: Vec<usize> = Vec::new();

    while st.running {
        let pc = st.program_counter as usize;

        // Terminate before running an instruction a second time
        if visited_instructions.contains(&pc) {
            st.running = false;
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
            st.running = false;
        }
    }
    println!("{:?}", st.accumulator);
}
