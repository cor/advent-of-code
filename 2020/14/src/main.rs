use aoc_2020_common::common::load_file;
use std::str::FromStr;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
enum Instruction {
    MASK(Mask),
    MEM(u64, u64),
}

lazy_static! {
    static ref MEM_INSTR_RE: Regex = Regex::new(r#"mem\[(\d+)] = (\d+)"#).unwrap();
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            if let Some(mask_str) = s.split(" = ").nth(1) {
                if let Ok(mask) = Mask::from_str(mask_str) {
                    return Ok(Instruction::MASK(mask));
                }
            }
            return Err(String::from("Invalid mask instruction"));
        } else if s.starts_with("mem") {
            if let Some(captures) = MEM_INSTR_RE.captures(s) {
                let addr = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                let value = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
                return Ok(Instruction::MEM(addr, value))
            }
            return Err(String::from("Invalid mem instruction"));
        }

        Err(String::from("Invalid operation in instruction"))
    }
}


#[derive(Debug)]
struct Mask {
    ones: u64,  // All 0, except for the bits that should be set to 1
    zeros: u64, // All 1, except for the bits that should be set to 0
}

impl Mask {
    /// Returns the target with the mask applied to it
    fn apply(&self, target: u64) -> u64 {
        (target | self.ones) & self.zeros
    }

    fn print(&self) {
        println!("{:64b}", self.zeros);
        println!("{:64b}", self.ones);
    }
}

impl FromStr for Mask {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ones: u64 = 0;
        let mut zeros = u64::max_value();

        for (index, value) in s.chars().rev().enumerate() {
            match value {
                'X' => continue,
                '1' => ones += 1 << index,
                '0' => zeros -= 1 << index,
                _ => return Err(String::from("Invalid char in mask"))
            }
        }
        Ok(Mask { ones, zeros })
    }
}



fn main() {
    let input = load_file("./input/example.txt");

    let instructions: Vec<Instruction> = input
        .lines()
        .map(Instruction::from_str)
        .filter_map(Result::ok)
        .collect();


    println!("{:#?}", &instructions);
    if let Instruction::MASK(m) = &instructions[0] {
        m.print();
    }
}
