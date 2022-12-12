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
    let instructions = Instruction::parse_list0(&input);
    dbg!("{}", instructions);
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
