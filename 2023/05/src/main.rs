use aoc_2023_common::challenge_input;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{line_ending, newline, space1, u64},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};
use rayon::prelude::*;

#[derive(Debug)]
struct Range {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

#[derive(Debug)]
struct Map {
    _name: String, // didn't need it but parsed just in case
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                (delimited(tag("seeds: "), separated_list0(space1, u64), newline)),
                preceded(
                    newline,
                    separated_list1(pair(line_ending, line_ending), Map::parse),
                ),
            )),
            |(seeds, maps)| Self { seeds, maps },
        )(input)
    }

    pub fn destination(&self, seed: u64) -> u64 {
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
                _name: name.to_string(),
                ranges,
            },
        )(input)
    }

    pub fn destination(&self, input: u64) -> u64 {
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
            tuple((u64, preceded(space1, u64), preceded(space1, u64))),
            |(destination_start, source_start, length)| Self {
                destination_start,
                source_start,
                length,
            },
        )(input)
    }

    fn includes(&self, input: u64) -> bool {
        input >= self.source_start && input < (self.source_start + self.length)
    }

    pub fn destination(&self, input: u64) -> Option<u64> {
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
    let part_1 = &almanac
        .seeds
        .iter()
        .map(|&seed| almanac.destination(seed))
        .min()
        .expect("There should be an answer");

    println!("{part_1}");

    let seed_ranges = &almanac
        .seeds
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<_>>();

    let part_2 = seed_ranges
        .par_iter()
        .map(|(start, length)| {
            (*start..(start + length))
                .into_par_iter()
                .map(|seed| almanac.destination(seed))
                .min()
                .expect("should be a min dest")
        })
        .min()
        .expect("should be a min dest");

    println!("{part_2}");
}
