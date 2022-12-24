use std::collections::HashSet;

use aoc_2022_common::challenge_input;
use nalgebra::{Point2, Vector2};

type Elve = Point2<i64>;
type Dir = Vector2<i64>;

const N: Dir = Dir::new(0, 1);
const S: Dir = Dir::new(0, -1);
const W: Dir = Dir::new(-1, 0);
const E: Dir = Dir::new(1, 0);

const NE: Dir = Dir::new(1, 1);
const NW: Dir = Dir::new(-1, 1);
const SE: Dir = Dir::new(1, -1);
const SW: Dir = Dir::new(-1, -1);

const N_SCAN: [Dir; 3] = [NW, N, NE];
const S_SCAN: [Dir; 3] = [SW, S, SE];
const W_SCAN: [Dir; 3] = [NW, W, SW];
const E_SCAN: [Dir; 3] = [NE, E, SE];

const ALL_SCAN: [Dir; 8] = [N, NE, E, SE, S, SE, W, SW];
const SCANS: [([Dir; 3], Dir); 4] = [(N_SCAN, N), (S_SCAN, S), (W_SCAN, W), (E_SCAN, E)];

trait ElveExt {
    fn scan(&self, scan: &[Dir], others: &HashSet<Elve>) -> bool;
    fn proposed_dir(&self, round: usize, others: &HashSet<Elve>) -> Dir;
}

impl ElveExt for Elve {
    fn scan(&self, scan: &[Dir], others: &HashSet<Elve>) -> bool {
        scan.iter()
            .filter(|&&dir| others.contains(&(self + dir)))
            .count()
            == self.len()
    }

    #[must_use]
    fn proposed_dir(&self, round: usize, others: &HashSet<Elve>) -> Dir {
        if self.scan(&ALL_SCAN, others) {
            return Vector2::default();
        }

        for i in 0..SCANS.len() {
            let (scan, dir) = SCANS[(round + i) % (SCANS.len())];
            if self.scan(&scan, others) {
                return dir;
            }
        }

        println!("unspecified in puzzle");
        Vector2::default()
    }
}

#[must_use]
pub fn parse_elves(input: &str) -> HashSet<Elve> {
    let mut elves = HashSet::<Elve>::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    elves.insert(Elve::new(x as i64, y as i64));
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
        println!("{elve:?}");
    }
}
