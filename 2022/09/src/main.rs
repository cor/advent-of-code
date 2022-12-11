use aoc_2022_common::challenge_input;
use derive_more::{Add, Sub};
use std::collections::HashSet;

fn main() {
    let directions = parse_input(&challenge_input());

    println!("{}", unique_tail_places::<2>(&directions));
    println!("{}", unique_tail_places::<10>(&directions));
}

fn unique_tail_places<const N: usize>(directions: &Vec<Vec2>) -> usize {
    let mut ropes = vec![Rope([Vec2::default(); N])];
    for direction in directions {
        ropes.push(ropes.last().unwrap().next(direction));
    }
    ropes.iter().map(|s| s.tail()).collect::<HashSet<_>>().len()
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

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default, Hash, Add, Sub)]
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

impl From<&str> for Vec2 {
    fn from(str: &str) -> Self {
        match str {
            "R" => Vec2::EAST,
            "U" => Vec2::NORTH,
            "L" => Vec2::WEST,
            "D" => Vec2::SOUTH,
            _ => panic!("invalid direction in output"),
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
