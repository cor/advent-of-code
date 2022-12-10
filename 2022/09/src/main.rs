use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

use aoc_2022_common::challenge_input;

fn main() {
    let dirs = parse_input(&challenge_input());

    let mut states = vec![State::default()];
    let mut states2 = vec![State2::default()];

    for dir in dirs {
        states.push(states.last().unwrap().next(&dir));
        states2.push(states2.last().unwrap().next(&dir));
    }

    let locations: HashSet<Vec2> = states.iter().map(|s| s.tail).collect();
    let locations2: HashSet<Vec2> = states2.iter().map(|s| s.tail[8]).collect();

    println!("{}", locations.len());
    println!("{}", locations2.len());
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
struct State {
    pub head: Vec2,
    pub tail: Vec2,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
struct State2 {
    pub head: Vec2,
    pub tail: [Vec2; 9],
}

impl State2 {
    pub fn next(&self, dir: &Vec2) -> Self {
        let head = self.head + *dir;
        let mut new_tail: [Vec2; 9] = [Vec2::default(); 9];

        for (i, v) in self.tail.iter().enumerate() {
            let neighbor = if i == 0 { head } else { new_tail[i - 1] };

            let delta = neighbor - *v;
            new_tail[i] = *v + delta.corrective_move();
        }

        State2 {
            head,
            tail: new_tail,
        }
    }
}

impl State {
    pub fn next(&self, dir: &Vec2) -> Self {
        let head = self.head + *dir;
        let delta = head - self.tail;

        State {
            head,
            tail: self.tail + delta.corrective_move(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default, Hash)]
struct Vec2(isize, isize);

impl Vec2 {
    const NORTH: Vec2 = Vec2(0, 1);
    const SOUTH: Vec2 = Vec2(0, -1);
    const EAST: Vec2 = Vec2(1, 0);
    const WEST: Vec2 = Vec2(-1, 0);

    fn corrective_move(&self) -> Vec2 {
        let norm = |n: isize| {
            if n >= 1 {
                1
            } else if n <= -1 {
                -1
            } else {
                0
            }
        };
        match self {
            Vec2(2, y) => Self::EAST + Vec2(0, norm(*y)),
            Vec2(-2, y) => Self::WEST + Vec2(0, norm(*y)),
            Vec2(x, 2) => Self::NORTH + Vec2(norm(*x), 0),
            Vec2(x, -2) => Self::SOUTH + Vec2(norm(*x), 0),
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
