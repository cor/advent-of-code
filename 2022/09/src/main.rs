use std::ops::{Add, Sub};

use aoc_2022_common::challenge_input;

fn main() {
    let dirs = parse_input(&challenge_input());

    let mut states = vec![State::default()];

    for dir in dirs {
        states.push(states.last().unwrap().next(&dir));
    }

    dbg!(states);
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
struct State {
    pub head: Vec2,
    pub tail: Vec2,
}

impl State {
    pub fn next(&self, dir: &Vec2) -> Self {
        let head = self.head + *dir;
        let delta = self.tail - head;

        State {
            head,
            tail: self.tail - delta.corrective_move(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
struct Vec2(isize, isize);

impl Vec2 {
    const NORTH: Vec2 = Vec2(0, 1);
    const NORTH_EAST: Vec2 = Vec2(1, 1);
    const NORTH_WEST: Vec2 = Vec2(-1, 1);
    const SOUTH: Vec2 = Vec2(0, -1);
    const SOUTH_EAST: Vec2 = Vec2(1, -1);
    const SOUTH_WEST: Vec2 = Vec2(-1, -1);
    const EAST: Vec2 = Vec2(1, 0);
    const WEST: Vec2 = Vec2(-1, 0);

    fn corrective_move(&self) -> Vec2 {
        match self {
            Vec2(2, y) => Self::EAST + Vec2(0, *y),
            Vec2(-2, y) => Self::SOUTH + Vec2(0, *y),
            Vec2(x, 2) => Self::NORTH + Vec2(*x, 0),
            Vec2(x, -2) => Self::WEST + Vec2(*x, 0),
            _ => Self::default(),
        }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl From<&str> for Vec2 {
    fn from(str: &str) -> Self {
        match str {
            "R" => Vec2::EAST,
            "U" => Vec2::NORTH,
            "L" => Vec2::WEST,
            "D" => Vec2::SOUTH,
            _ => panic!("invalid char in output"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec2> {
    input
        .lines()
        .flat_map(|line| {
            let (dir, count) = line.split_once(' ').unwrap();
            let count = count.parse::<i32>().unwrap();
            (0..count).map(|_| Vec2::from(dir)).collect::<Vec<Vec2>>()
        })
        .collect()
}
