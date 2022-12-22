use std::collections::HashSet;

use aoc_2022_common::challenge_input;
use derive_more::{Add, Sub};

const SIDES: [Point; 6] = [
    Point { x: 1, y: 0, z: 0 },
    Point { x: -1, y: 0, z: 0 },
    Point { x: 0, y: 1, z: 0 },
    Point { x: 0, y: -1, z: 0 },
    Point { x: 0, y: 0, z: 1 },
    Point { x: 0, y: 0, z: -1 },
];

#[derive(Debug, Hash, Add, Sub, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    /// Panics if input is invalid
    pub fn parse(input: &str) -> Self {
        let (x, yz) = input.split_once(',').unwrap();
        let (y, z) = yz.split_once(',').unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        }
    }

    pub fn parse_list0(input: &str) -> HashSet<Self> {
        input.lines().map(Self::parse).collect()
    }

    pub fn exposed_sides(&self, others: &HashSet<Self>) -> usize {
        SIDES
            .iter()
            .filter(|&side| !others.contains(&(*self + *side)))
            .count()
    }
    pub fn really_exposed_sides(&self, others: &HashSet<Self>) -> usize {
        SIDES
            .iter()
            .filter(|&side| {
                let start = *self + *side;
                if others.contains(&start) {
                    return false;
                }
                let mut visited: HashSet<Self> = HashSet::new();
                let mut steps = Vec::<HashSet<Point>>::new();
                visited.insert(start);
                steps.push(visited.clone());

                for _ in 1..20 {
                    let last = steps.last().unwrap();
                    if last.is_empty() {
                        return false;
                    }

                    let next = last
                        .iter()
                        .flat_map(|last| SIDES.iter().map(|&s| *last + s))
                        .filter(|p| !visited.contains(p) && !others.contains(p))
                        .collect::<HashSet<Point>>();

                    visited.extend(&next);
                    steps.push(next);
                }

                true
            })
            .count()
    }
}

fn main() {
    let input = challenge_input();
    let points = Point::parse_list0(&input);
    // let points = Point::parse_list0("1,1,1\n2,1,1");
    let exposed_sides: usize = points.iter().map(|p| p.exposed_sides(&points)).sum();
    let really_exposed_sides: usize = points.iter().map(|p| p.really_exposed_sides(&points)).sum();

    println!("{exposed_sides}");
    println!("{really_exposed_sides}");
}
