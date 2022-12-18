use std::collections::HashSet;

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
    pub ty: ElementType, // type is a keyword
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn points_between(&self, other: &Self) -> HashSet<Self> {
        if self.x == other.x {
            if self.y < other.y {
                (self.y..=other.y).map(|y| Self::new(self.x, y)).collect()
            } else {
                (other.y..=self.y).map(|y| Self::new(self.x, y)).collect()
            }
        } else if self.y == other.y {
            if self.x < other.x {
                (self.x..=other.x).map(|x| Self::new(x, self.y)).collect()
            } else {
                (other.x..=self.x).map(|x| Self::new(x, self.y)).collect()
            }
        } else {
            panic!("x's should be the same or y's should be the same");
        }
    }

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

#[derive(Clone, Debug)]
struct World {
    elements: HashSet<Element>,
    start: Point,
}

impl World {
    pub fn new(rock_corner_sequences: &[Vec<Point>]) -> Self {
        let elements: HashSet<Element> = rock_corner_sequences
            .iter()
            .fold(HashSet::<Point>::new(), |mut rocks, seq| {
                let new_rocks = seq
                    .iter()
                    .fold(
                        (HashSet::<Point>::new(), seq[0]),
                        |(mut all_rocks, last_corner), next_corner| {
                            all_rocks.extend(&last_corner.points_between(next_corner));
                            (all_rocks, *next_corner)
                        },
                    )
                    .0;
                rocks.extend(new_rocks);

                rocks
            })
            .iter()
            .map(|&point| Element {
                point,
                ty: ElementType::Stone,
            })
            .collect();

        Self {
            elements,
            start: Point::new(500, 0),
        }
    }
}

fn main() {
    let input = challenge_input();
    let (_, point_sequences) = Point::parse_sequence_list(&input).expect("invalid points in input");
    dbg!(&point_sequences);

    println!("{}", input);
    let world = World::new(&point_sequences);
    dbg!(world);
    // dbg!(Point::new(2, 4).points_between(&Point::new(2, 8)));
}
