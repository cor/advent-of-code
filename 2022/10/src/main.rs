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
        let last = *x_history.last().unwrap() as i64;

        x_history.push(last);
        if let Instruction::Addx(n) = instr {
            x_history.push(last + n);
        }
    }

    let signals = x_history
        .iter()
        .enumerate()
        .map(|(i, x)| i as i64 * x)
        .collect::<Vec<_>>();

    if signals.len() >= 220 {
        let sum =
            signals[20] + signals[60] + signals[100] + signals[140] + signals[180] + signals[220];
        println!("{}", sum);
    } else {
        println!("Output is too short to produce signal sum");
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
