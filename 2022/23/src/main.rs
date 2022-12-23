use std::collections::HashSet;

use aoc_2022_common::challenge_input;
use nalgebra::Point2;

const N: Point2<i64> = Point2::new(0, 1);
const S: Point2<i64> = Point2::new(0, -1);
const W: Point2<i64> = Point2::new(-1, 0);
const E: Point2<i64> = Point2::new(1, 0);

const NE: Point2<i64> = Point2::new(1, 1);
const NW: Point2<i64> = Point2::new(-1, 1);
const SE: Point2<i64> = Point2::new(1, -1);
const SW: Point2<i64> = Point2::new(-1, -1);

const N_SCAN: [Point2<i64>; 3] = [NW, N, NE];
const S_SCAN: [Point2<i64>; 3] = [SW, S, SE];
const W_SCAN: [Point2<i64>; 3] = [NW, W, SW];
const E_SCAN: [Point2<i64>; 3] = [NE, E, SE];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Elve {
    pub position: Point2<i64>,
}

pub fn parse_elves(input: &str) -> HashSet<Elve> {
    let mut elves = HashSet::<Elve>::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    elves.insert(Elve {
                        position: Point2::new(x as i64, y as i64),
                    });
                }
                '.' => {} // ground tile => do nothing
                t => panic!("Invalid tile {t} in input!"),
            };
        }
    }
    elves
}

fn main() {
    let input = challenge_input();
    let elves = parse_elves(&input);
    for elve in elves {
        println!("{:?}", elve);
    }
}
