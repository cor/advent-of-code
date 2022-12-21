use std::collections::HashSet;

use aoc_2022_common::challenge_input;

use derive_more::{Add, Constructor};
use rayon::prelude::*;

use nom::sequence::preceded;
use nom::{
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    combinator::map,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Add, Constructor)]
struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Add, Constructor)]
struct Sensor {
    pub position: Point,
    pub beacon: Point,
}

/// Inclusive upper bound
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Range(i64, i64);
impl Range {
    fn contains(&self, x: i64) -> bool {
        self.0 <= x && x <= self.1
    }

    fn length(&self) -> i64 {
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
    fn length(&self) -> i64;
    fn contains(&self, x: i64) -> bool;
    fn remove_overlaps_and_sort(&mut self);
    fn find_gap(&self, within: Range) -> Option<i64>;
}

impl Ranges for Vec<Range> {
    fn length(&self) -> i64 {
        self.iter().map(Range::length).sum()
    }

    fn contains(&self, x: i64) -> bool {
        self.iter().any(|r| r.contains(x))
    }

    fn remove_overlaps_and_sort(&mut self) {
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
        self.sort_by(|r0, r1| r0.0.cmp(&r1.0));
    }

    /// Assumes [`remove_overlaps_and_sort()`] is called on self before.
    fn find_gap(&self, within: Range) -> Option<i64> {
        for i in 0..(self.len() - 1) {
            let possible_gap = self[i].1 + 1;

            if within.contains(possible_gap) && possible_gap < self[i + 1].0 {
                return Some(possible_gap);
            }
        }
        None
    }
}

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

    pub fn radius(&self) -> i64 {
        self.position.manhattan(self.beacon)
    }

    pub fn intersection(&self, y: i64) -> Option<Range> {
        let r = self.radius();
        let (px, py) = (self.position.x, self.position.y);

        (!(y > py + r || y < py - r)).then(|| {
            let dy = (y - py).abs();
            let dx = r - dy;
            Range(px - dx, px + dx)
        })
    }
}

trait Sensors {
    fn intersections(&self, y: i64) -> Vec<Range>;
}

impl Sensors for Vec<Sensor> {
    fn intersections(&self, y: i64) -> Vec<Range> {
        let mut intersections = self
            .iter()
            .filter_map(|s| s.intersection(y))
            .collect::<Vec<_>>();
        intersections.remove_overlaps_and_sort();
        intersections
    }
}

impl Point {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                preceded(tag("x="), i64),
                tag(", "),
                preceded(tag("y="), i64),
            ),
            |(x, y)| Self::new(x, y),
        )(input)
    }

    pub fn manhattan(self, other: Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
fn part_1(sensors: &Vec<Sensor>, y: i64) -> i64 {
    let beacons_on_y = sensors
        .iter()
        .map(|s| s.beacon)
        .filter(|b| b.y == y)
        .collect::<HashSet<_>>()
        .len() as i64;

    sensors.intersections(y).length() - beacons_on_y
}

fn part_2(sensors: &Vec<Sensor>, limit: i64) -> Option<i64> {
    let target_range = Range(0, limit);

    (0..limit).into_par_iter().find_map_first(|y| {
        sensors
            .intersections(y)
            .find_gap(target_range)
            .map(|x| x * 4_000_000 + y)
    })
}

fn main() {
    let input = challenge_input();
    let (_, sensors) = Sensor::parse_list0(&input).expect("Invalid sensors in input");

    println!("{}", part_1(&sensors, 2_000_000));
    println!(
        "{}",
        part_2(&sensors, 4_000_000).expect("no part 2 solution")
    );
}
