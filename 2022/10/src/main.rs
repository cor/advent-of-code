use aoc_2022_common::challenge_input;
use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    combinator::{map, success},
    multi::separated_list0,
    sequence::preceded,
    IResult,
};

fn main() {
    let input = challenge_input();
    let (_, instructions) = Instruction::parse_list0(&input).expect("Invalid input");

    let mut x_history: Vec<i64> = vec![1, 1];
    for instr in instructions {
        let last = *x_history.last().unwrap();

        x_history.push(last);
        if let Instruction::Addx(n) = instr {
            x_history.push(last + n);
        }
    }

    part_1(&x_history);
    part_2(&x_history);
}

pub fn part_1(x_history: &[i64]) {
    let sigs = x_history
        .iter()
        .enumerate()
        .map(|(i, x)| i as i64 * x)
        .collect::<Vec<_>>();

    if sigs.len() >= 220 {
        let sum = sigs[20] + sigs[60] + sigs[100] + sigs[140] + sigs[180] + sigs[220];
        println!("{sum}");
    } else {
        println!("Output is too short to produce signal sum");
    }
}

pub fn part_2(x_history: &[i64]) {
    for (i, &x) in x_history.iter().skip(1).enumerate() {
        let i = i as i64 % 40;
        if i % 40 == 0 {
            println!();
        }
        if i == (x - 1) || i == x || i == (x + 1) {
            print!("█");
        } else {
            print!(" ");
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl Instruction {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            preceded(tag("noop"), success(Self::Noop)),
            map(preceded(tag("addx "), i64), Self::Addx),
        ))(input)
    }
    pub fn parse_list0(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list0(line_ending, Self::parse)(input)
    }
}
