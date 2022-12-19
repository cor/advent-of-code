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
    pub closest_beacon: Point,
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

    pub fn beaconless_points(&self) -> HashSet<Point> {
        //   2
        //  212
        // 21012
        //  212
        //   2
        let mut points: Vec<HashSet<Point>> = Vec::new();
        let mut all_points: HashSet<Point> = HashSet::new();
        all_points.insert(self.position);
        points.push(all_points.clone());

        loop {
            let new_neighbors = points
                .last()
                .unwrap()
                .iter()
                .flat_map(Point::neighbors)
                .filter(|p| !all_points.contains(&p))
                .collect::<HashSet<Point>>();
            all_points.extend(&new_neighbors);

            if new_neighbors.contains(&self.closest_beacon) {
                break;
            } else {
                points.push(new_neighbors);
            }
        }

        all_points.remove(&self.closest_beacon);
        all_points
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

    pub fn neighbors(&self) -> HashSet<Self> {
        DIRECTIONS.iter().map(|&dir| *self + dir).collect()
    }
}

fn main() {
    let input = challenge_input();
    let (_, sensors) = Sensor::parse_list0(&input).expect("Invalid sensors in input");
    let beaconless_points = sensors
        .iter()
        .flat_map(Sensor::beaconless_points)
        .collect::<HashSet<_>>();

    let y10 = beaconless_points.iter().filter(|p| p.y == 10).count();
    println!("{}", y10);
    // dbg!(beaconless_points);
}
