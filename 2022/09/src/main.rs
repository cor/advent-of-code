use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

use aoc_2022_common::challenge_input;

fn main() {
    let directions = parse_input(&challenge_input());

    let mut ropes_1 = vec![Rope([Vec2::default(); 2])];
    let mut ropes_2 = vec![Rope([Vec2::default(); 10])];

    for direction in directions {
        ropes_1.push(ropes_1.last().unwrap().next(&direction));
        ropes_2.push(ropes_2.last().unwrap().next(&direction));
    }

    let tail_positions_1: HashSet<Vec2> = ropes_1.iter().map(|s| s.tail()).collect();
    let tail_positions_2: HashSet<Vec2> = ropes_2.iter().map(|s| s.tail()).collect();

    println!("{}", tail_positions_1.len());
    println!("{}", tail_positions_2.len());
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Rope<const N: usize>([Vec2; N]);

impl<const N: usize> Rope<N> {
    pub fn next(&self, dir: &Vec2) -> Self {
        let mut new_rope: [Vec2; N] = [Vec2::default(); N];

        new_rope[0] = self.0[0] + *dir;
        for (i, segment) in self.0[1..].iter().enumerate() {
            let delta = new_rope[i] - *segment;
            new_rope[i + 1] = *segment + delta.corrective_move();
        }

        Rope(new_rope)
    }

    pub fn tail(&self) -> Vec2 {
        self.0[N - 1]
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
        let normalize = |n: isize| (n.cmp(&0) as isize);

        match self {
            Vec2(2, y) => Self::EAST + Vec2(0, normalize(*y)),
            Vec2(-2, y) => Self::WEST + Vec2(0, normalize(*y)),
            Vec2(x, 2) => Self::NORTH + Vec2(normalize(*x), 0),
            Vec2(x, -2) => Self::SOUTH + Vec2(normalize(*x), 0),
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
