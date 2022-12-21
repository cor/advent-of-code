use std::collections::HashSet;

use aoc_2022_common::challenge_input;

use derive_more::{Add, Constructor};

use nom::sequence::preceded;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    combinator::map,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Add, Constructor)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Add, Constructor)]
struct Sensor {
    pub position: Point,
    pub beacon: Point,
}

const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
];

impl Sensor {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            preceded(
                tag("Sensor at "),
                separated_pair(Point::parse, tag(": closest beacon is at "), Point::parse),
            ),
            |(position, closest_beacon)| Sensor::new(position, closest_beacon),
        )(input)
    }

    pub fn parse_list0(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list0(line_ending, Self::parse)(input)
    }

    pub fn radius(&self) -> i32 {
        self.position.manhattan(self.beacon)
    }

    pub fn intersection(&self, y: i32) -> Option<(i32, i32)> {
        let r = self.radius();
        let (px, py) = (self.position.x, self.position.y);
        if y > py + r || y < py - r {
            None
        } else {
            let dy = (y - py).abs();
            let dx = r - dy;
            Some((px - dx, px + dx))
        }
    }
}

impl Point {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                preceded(tag("x="), i32),
                tag(", "),
                preceded(tag("y="), i32),
            ),
            |(x, y)| Self::new(x, y),
        )(input)
    }

    pub fn neighbors(self) -> HashSet<Self> {
        DIRECTIONS.iter().map(|&dir| self + dir).collect()
    }

    pub fn manhattan(self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn main() {
    let input = challenge_input();
    let (_, sensors) = Sensor::parse_list0(&input).expect("Invalid sensors in input");
    dbg!(&sensors[0]);
    dbg!(&sensors[0].radius());

    let s = Sensor::new(Point::new(4, 0), Point::new(4, 4));
    dbg!(s.intersection(-4));
    // dbg!(Point::new(0, 2).manhattan(Point::new(-4, 9)));
    // dbg!(beaconless_points);
}
