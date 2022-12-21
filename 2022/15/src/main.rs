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

/// Inclusive upper bound
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Range(i32, i32);
impl Range {
    fn contains(self, x: i32) -> bool {
        self.0 <= x && x <= self.1
    }

    fn length(&self) -> i32 {
        self.1 - self.0 + 1
    }

    fn overlaps(self, other: Range) -> bool {
        (self.0 >= other.0 && self.0 <= other.1) || (self.1 >= other.0 && self.1 <= other.1)
    }

    pub fn combined(self, other: Self) -> Option<Self> {
        self.overlaps(other)
            .then(|| Range(self.0.min(other.0), self.1.max(other.1)))
    }
}

trait Ranges {
    fn length(&self) -> i32;
    fn contains(&self, x: i32) -> bool;
    fn remove_overlaps(&mut self);
}

impl Ranges for Vec<Range> {
    fn length(&self) -> i32 {
        self.iter().map(Range::length).sum()
    }

    fn contains(&self, x: i32) -> bool {
        self.iter().any(|r| r.contains(x))
    }

    fn remove_overlaps(&mut self) {
        'root: loop {
            for i in 0..self.len() {
                for j in 0..self.len() {
                    if i == j {
                        continue; // don't merge a range with itself
                    }
                    if let Some(r) = self[i].combined(self[j]) {
                        self[i] = r;
                        self.remove(j);
                        continue 'root;
                    }
                }
            }
            break;
        }
    }
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

    pub fn intersection(&self, y: i32) -> Option<Range> {
        let r = self.radius();
        let (px, py) = (self.position.x, self.position.y);

        (!(y > py + r || y < py - r)).then(|| {
            let dy = (y - py).abs();
            let dx = r - dy;
            Range(px - dx, px + dx)
        })
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
    let y = 2000000;
    let (_, sensors) = Sensor::parse_list0(&input).expect("Invalid sensors in input");
    let mut intersections = sensors
        .iter()
        .filter_map(|s| s.intersection(y))
        .collect::<Vec<_>>();

    dbg!(&intersections);
    intersections.remove_overlaps();
    dbg!(&intersections);
    let beacons_on_y = sensors
        .iter()
        .map(|s| s.beacon)
        .collect::<HashSet<_>>()
        .iter()
        .filter(|b| b.y == y && intersections.contains(b.x))
        .count() as i32;

    let total_length = &intersections.length();

    dbg!(sensors);
    dbg!(total_length);
    dbg!(beacons_on_y);
    dbg!(total_length - beacons_on_y);
}
