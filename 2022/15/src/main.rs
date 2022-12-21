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
    fn remove_overlaps(ranges: &mut Vec<Self>) {
        'root: loop {
            for i in 0..ranges.len() {
                for j in 0..ranges.len() {
                    if i == j {
                        continue; // don't merge a range with itself
                    }
                    if let Some(r) = ranges[i].combined(ranges[j]) {
                        ranges[i] = r;
                        ranges.remove(j);
                        continue 'root;
                    }
                }
            }
            break;
        }
    }

    fn total_length(ranges: &[Self]) -> i32 {
        ranges.iter().map(Self::length).sum()
    }

    fn contains(self, x: i32) -> bool {
        self.0 >= x && x <= self.1
    }

    fn list_contains(ranges: &[Self], x: i32) -> bool {
        ranges.iter().any(|r| r.contains(x))
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
        if y > py + r || y < py - r {
            None
        } else {
            let dy = (y - py).abs();
            let dx = r - dy;
            Some(Range(px - dx, px + dx))
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
    let y = 2000000;
    let (_, sensors) = Sensor::parse_list0(&input).expect("Invalid sensors in input");
    let mut intersections = sensors
        .iter()
        .filter_map(|s| s.intersection(y))
        .collect::<Vec<_>>();

    dbg!(&intersections);
    Range::remove_overlaps(&mut intersections);
    dbg!(&intersections);
    let beacons_on_y = sensors
        .iter()
        .map(|s| s.beacon)
        .filter(|b| b.y == y && Range::list_contains(&intersections, b.x))
        .count() as i32;
    let total_length = Range::total_length(&intersections);

    dbg!(total_length);
    dbg!(beacons_on_y);
    dbg!(Range::total_length(&intersections) - beacons_on_y);

    // dbg!(Point::new(0, 2).manhattan(Point::new(-4, 9)));
    // dbg!(beaconless_points);
}
