use aoc_2023_common::challenge_input;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{line_ending, newline, space1, u32},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Range {
    destination_start: u32,
    source_start: u32,
    length: u32,
}

#[derive(Debug)]
struct Map {
    name: String,
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

impl Almanac {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                (delimited(tag("seeds: "), separated_list0(space1, u32), newline)),
                preceded(
                    newline,
                    separated_list1(pair(line_ending, line_ending), Map::parse),
                ),
            )),
            |(seeds, maps)| Self { seeds, maps },
        )(input)
    }
}

impl Map {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                take_till(|c| c == ' '),
                tag(" map:\n"),
                separated_list1(newline, Range::parse),
            ),
            |(name, ranges)| Self {
                name: name.to_string(),
                ranges,
            },
        )(input)
    }
}

impl Range {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((u32, preceded(space1, u32), preceded(space1, u32))),
            |(destination_start, source_start, length)| Self {
                destination_start,
                source_start,
                length,
            },
        )(input)
    }
}

fn main() {
    let input = challenge_input();
    let almanac = Almanac::parse(&input).expect("Invalid input").1;
    println!("{almanac:#?}");
}
