use aoc_2022_common::challenge_input;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::i32,
    character::{complete::line_ending, streaming::char},
    combinator::map,
    multi::{count, many0, separated_list0},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Element {
    pub point: Point,
    pub ty: ElementType,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_pair(i32, char(','), i32), |(x, y)| Point { x, y })(input)
    }
    pub fn parse_sequence(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list0(tag(" -> "), Self::parse)(input)
    }
    pub fn parse_sequence_list(input: &str) -> IResult<&str, Vec<Vec<Self>>> {
        separated_list0(line_ending, Self::parse_sequence)(input)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum ElementType {
    Stone,
    Sand,
}

fn main() {
    let input = challenge_input();
    let (_, point_sequences) = Point::parse_sequence_list(&input).expect("invalid points in input");
    dbg!(point_sequences);
    println!("{}", input);
}
