use core::time;
use std::{collections::HashSet, thread};

use aoc_2022_common::challenge_input;
use nalgebra::{Point2, Vector2};

type Elve = Point2<i64>;
type Elves = HashSet<Elve>;
type Dir = Vector2<i64>;

const N: Dir = Dir::new(0, -1);
const S: Dir = Dir::new(0, 1);
const W: Dir = Dir::new(-1, 0);
const E: Dir = Dir::new(1, 0);

const NE: Dir = Dir::new(1, -1);
const NW: Dir = Dir::new(-1, -1);
const SE: Dir = Dir::new(1, 1);
const SW: Dir = Dir::new(-1, 1);

const N_SCAN: [Dir; 3] = [NW, N, NE];
const S_SCAN: [Dir; 3] = [SW, S, SE];
const W_SCAN: [Dir; 3] = [NW, W, SW];
const E_SCAN: [Dir; 3] = [NE, E, SE];

const ALL_SCAN: [Dir; 8] = [N, NE, E, S, SE, NW, W, SW];
const SCANS: [([Dir; 3], Dir); 4] = [(N_SCAN, N), (S_SCAN, S), (W_SCAN, W), (E_SCAN, E)];

trait ElveExt {
    fn scan(&self, scan: &[Dir], others: &Elves) -> bool;
    fn proposed_dir(&self, round: usize, others: &HashSet<Elve>) -> Dir;
}

impl ElveExt for Elve {
    fn scan(&self, scan: &[Dir], others: &Elves) -> bool {
        scan.iter().all(|&dir| !others.contains(&(self + dir)))
    }

    #[must_use]
    fn proposed_dir(&self, round: usize, others: &Elves) -> Dir {
        if self.scan(&ALL_SCAN, others) {
            return Vector2::new(0, 0);
        }

        for i in 0..SCANS.len() {
            let (scan, dir) = SCANS[(round + i) % (SCANS.len())];
            if self.scan(&scan, others) {
                return dir;
            }
        }

        // Unspecified in the puzzle, but when no direction is available,
        // we keep standing still.
        Vector2::new(0, 0)
    }
}

trait ElvesExt {
    fn next(&self, round: usize) -> Elves;
    fn parse(input: &str) -> Elves;
    fn edges(&self) -> (i64, i64, i64, i64);
    fn print(&self, round: usize);
}

impl ElvesExt for Elves {
    fn next(&self, round: usize) -> Elves {
        let propsed_poss = self
            .iter()
            .map(|e| (e, e + e.proposed_dir(round, self)))
            .collect::<HashSet<_>>();

        propsed_poss
            .iter()
            .map(|(&old_e, new_e)| {
                if propsed_poss
                    .iter()
                    .any(|(&o_e, n_e)| o_e != old_e && n_e == new_e)
                {
                    old_e
                } else {
                    *new_e
                }
            })
            .collect()
    }

    fn parse(input: &str) -> Elves {
        let mut elves = Elves::new();
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

    // Returns the smallest containing rect in N E S W order
    fn edges(&self) -> (i64, i64, i64, i64) {
        (
            self.iter().map(|e| e.y).min().unwrap(),
            self.iter().map(|e| e.x).max().unwrap(),
            self.iter().map(|e| e.y).max().unwrap(),
            self.iter().map(|e| e.x).min().unwrap(),
        )
    }

    fn print(&self, round: usize) {
        let (min_y, max_x, max_y, min_x) = self.edges();

        print!("\x1b[38;5;29m ┏");
        (min_x..=(max_x)).for_each(|_| print!("━━"));

        println!("━┓\x1b[0m");
        for y in min_y..=max_y {
            print!(" \x1b[38;5;29m┃ ");
            for x in min_x..=max_x {
                if self.contains(&Elve::new(x, y)) {
                    print!("\x1b[93m⬤ \x1b[0m");
                } else {
                    print!("\x1b[38;5;240m∘ \x1b[0m");
                }
            }
            println!("\x1b[38;5;29m┃");
        }

        print!(" ┗━");
        let end = format!(" R-{:0width$} ", round, width = 2);
        (0..=((max_x - min_x) * 2 - end.len() as i64)).for_each(|_| print!("━"));
        print!("\x1b[1;38;5;160m{}\x1b[0m", end);
        println!("\x1b[38;5;29m━┛");

        println!();
    }
}

fn main() {
    let input = challenge_input();
    let mut elves = Elves::parse(&input);
    for round in 0..100 {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        elves.print(round);
        elves = elves.next(round);
        thread::sleep(time::Duration::from_millis(1000));
    }
}
