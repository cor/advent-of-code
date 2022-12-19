use std::collections::HashSet;

use aoc_2022_common::challenge_input;

use derive_more::Add;
use nom::{
    bytes::complete::tag,
    character::{
        complete::{i32, line_ending},
        streaming::char,
    },
    combinator::map,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Element {
    pub point: Point,
    pub ty: ElementType, // type is a keyword
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Add)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn points_between(self, other: Self) -> HashSet<Self> {
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
    Rock,
    Sand,
}

#[derive(Clone, Debug)]
struct World {
    elements: HashSet<Element>,
    start: Point,
    floor_height: Option<i32>,
}

impl World {
    pub fn new(rock_corner_sequences: &[Vec<Point>], with_floor: bool) -> Self {
        let elements: HashSet<Element> = rock_corner_sequences
            .iter()
            .fold(HashSet::<Point>::new(), |mut all_rocks, seq| {
                let new_rocks = seq
                    .iter()
                    .fold(
                        (HashSet::<Point>::new(), seq[0]),
                        |(mut rocks, last_corner), next_corner| {
                            rocks.extend(last_corner.points_between(*next_corner));
                            (rocks, *next_corner)
                        },
                    )
                    .0;
                all_rocks.extend(new_rocks);
                all_rocks
            })
            .iter()
            .map(|&point| Element {
                point,
                ty: ElementType::Rock,
            })
            .collect();

        let floor_height = if with_floor {
            Some(elements.iter().map(|el| el.point.y).max().unwrap() + 2)
        } else {
            None
        };

        Self {
            elements,
            start: Point::new(500, 0),
            floor_height,
        }
    }

    pub fn add_sand(&mut self) -> Option<Point> {
        let mut current_point = self.start;

        loop {
            if self.void_below(current_point) {
                return None;
            }

            let below = current_point + Point::new(0, 1);
            if self.get(below).is_none() {
                current_point = below;
            } else {
                let bottom_left = current_point + Point::new(-1, 1);
                let bottom_right = current_point + Point::new(1, 1);
                if self.get(bottom_left).is_none() {
                    current_point = bottom_left;
                } else if self.get(bottom_right).is_none() {
                    current_point = bottom_right;
                } else {
                    // We have reached a stable point, drop the sand here
                    self.elements.insert(Element {
                        point: current_point,
                        ty: ElementType::Sand,
                    });
                    return Some(current_point);
                }
            }
        }
    }

    pub fn get(&self, point: Point) -> Option<ElementType> {
        use ElementType::{Rock, Sand};
        if self.elements.contains(&Element { point, ty: Rock }) {
            Some(Rock)
        } else if self.elements.contains(&Element { point, ty: Sand }) {
            Some(Sand)
        } else if self.floor_height == Some(point.y) {
            Some(Rock)
        } else {
            None
        }
    }

    pub fn void_below(&self, Point { x, y }: Point) -> bool {
        self.floor_height.is_none()
            && !self
                .elements
                .iter()
                .any(|el| el.point.x == x && el.point.y > y)
    }

    pub fn sand_count(&self) -> usize {
        self.elements
            .iter()
            .filter(|el| el.ty == ElementType::Sand)
            .count()
    }
}

#[must_use]
fn part_1(rock_corner_sequences: &[Vec<Point>]) -> usize {
    let mut world = World::new(rock_corner_sequences, false);
    loop {
        let added_sand = world.add_sand();
        if added_sand.is_none() {
            break;
        }
    }
    world.sand_count()
}

#[must_use]
fn part_2(rock_corner_sequences: &[Vec<Point>]) -> usize {
    let mut world = World::new(rock_corner_sequences, true);
    loop {
        let added_sand = world.add_sand();
        if added_sand == Some(world.start) {
            break;
        }
    }
    world.sand_count()
}

fn main() {
    let input = challenge_input();
    let (_, rock_corner_sequences) =
        Point::parse_sequence_list(&input).expect("Invalid rock corners in input");

    println!("{}", part_1(&rock_corner_sequences));
    println!("{}", part_2(&rock_corner_sequences));
}
