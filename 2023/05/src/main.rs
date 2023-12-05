use aoc_2023_common::challenge_input;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{line_ending, newline, space1, u128},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Range {
    destination_start: u128,
    source_start: u128,
    length: u128,
}

#[derive(Debug)]
struct Map {
    name: String,
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u128>,
    maps: Vec<Map>,
}

impl Almanac {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                (delimited(tag("seeds: "), separated_list0(space1, u128), newline)),
                preceded(
                    newline,
                    separated_list1(pair(line_ending, line_ending), Map::parse),
                ),
            )),
            |(seeds, maps)| Self { seeds, maps },
        )(input)
    }

    pub fn destination(&self, seed: u128) -> u128 {
        let mut seed = seed;

        for map in &self.maps {
            seed = map.destination(seed)
        }
        seed
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

    pub fn destination(&self, input: u128) -> u128 {
        for range in &self.ranges {
            if let Some(destination) = range.destination(input) {
                return destination;
            }
        }
        input
    }
}

impl Range {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((u128, preceded(space1, u128), preceded(space1, u128))),
            |(destination_start, source_start, length)| Self {
                destination_start,
                source_start,
                length,
            },
        )(input)
    }

    fn includes(&self, input: u128) -> bool {
        input >= self.source_start && input < (self.source_start + self.length)
    }

    pub fn destination(&self, input: u128) -> Option<u128> {
        if !self.includes(input) {
            return None;
        }
        let offset = input - self.source_start;
        Some(self.destination_start + offset)
    }
}

fn main() {
    let input = challenge_input();
    let almanac = Almanac::parse(&input).expect("Invalid input").1;
    let destinations = almanac
        .seeds
        .iter()
        .map(|&seed| almanac.destination(seed))
        .min()
        .expect("There should be an answer");
    println!("{destinations:#?}");
    // println!("{almanac:#?}");
}
