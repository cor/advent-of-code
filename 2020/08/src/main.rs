use aoc_2020_common::common::load_file;
use regex::Regex;

#[derive(Debug)]
enum Instruction {
    NOP,
    ACC,
    JMP,
}

fn main() {
    let input = load_file("./input/1.txt");
    let instruction_re: Regex = Regex::new(r#"([a-z]{3}) ([+\-])(\d+)"#).unwrap();

    let instructions: Vec<(Instruction, i64)> = instruction_re
        .captures_iter(&input)
        .map(|cap| {
            let instruction = match &cap[1] {
                "nop" => Instruction::NOP,
                "acc" => Instruction::ACC,
                "jmp" => Instruction::JMP,
                _ => panic!("Invalid instruction")
            };
            let argument = match &cap[2] {
                "+" => cap[3].parse::<i64>().unwrap(),
                "-" => -1 * cap[3].parse::<i64>().unwrap(),
                _ => panic!("Invalid sign before argument"),
            };

            (instruction, argument)
        })
        .collect();

    println!("{:#?}", instructions);
}
