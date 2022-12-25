use core::time;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    thread,
};

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

const MAIN_DIRS: [Dir; 4] = [N, S, W, E];
const ALL_SCAN: [Dir; 8] = [N, NE, E, S, SE, NW, W, SW];
const SCANS: [([Dir; 3], Dir); 4] = [(N_SCAN, N), (S_SCAN, S), (W_SCAN, W), (E_SCAN, E)];

#[inline(always)]
fn opposite_dir(dir: Dir) -> Dir {
    if dir == N {
        S
    } else if dir == S {
        N
    } else if dir == E {
        W
    } else if dir == W {
        E
    } else {
        panic!("attempt to get opposite of non NSWE dir");
    }
}

trait ElveExt {
    fn scan(&self, scan: &[Dir], others: &Elves) -> bool;
    fn proposed_dir(&self, round: usize, others: &Elves) -> Dir;
    fn next(&self, round: usize, others: &Elves) -> Elve;
}

impl ElveExt for Elve {
    fn scan(&self, scan: &[Dir], others: &Elves) -> bool {
        scan.iter().all(|&dir| !others.contains(&(self + dir)))
    }

    #[must_use]
    #[inline(always)]
    fn proposed_dir(&self, round: usize, others: &Elves) -> Dir {
        if self.scan(&ALL_SCAN, others) {
            return Dir::default();
        }

        for i in 0..SCANS.len() {
            let (scan, dir) = SCANS[(round + i) % (SCANS.len())];
            if self.scan(&scan, others) {
                return dir;
            }
        }

        Dir::default()
    }

    #[inline(always)]
    fn next(&self, round: usize, others: &Elves) -> Elve {
        let prop_dir = self.proposed_dir(round, others);

        if prop_dir == Dir::default() {
            return *self;
        }

        let test = |candidate: Dir, dir: Dir| {
            let candidate_point = self + candidate;
            others.contains(&candidate_point) && candidate_point.proposed_dir(round, others) == dir
        };

        for main_dir in MAIN_DIRS {
            if main_dir == opposite_dir(prop_dir) {
                continue;
            }

            // Another elve also wants to go to our spot, so we won't go there.
            if test(prop_dir + main_dir, opposite_dir(main_dir)) {
                return *self;
            }
        }
        self + prop_dir
    }
}

trait ElvesExt {
    fn next(&self, round: usize) -> Elves;
    fn parse(input: &str) -> Elves;
    fn edges(&self) -> (i64, i64, i64, i64);
    fn print(&self, round: usize, end_min_y: i64, end_min_x: i64);
    fn part_1(&self) -> i64;
}

impl ElvesExt for Elves {
    #[inline(always)]
    fn next(&self, round: usize) -> Elves {
        self.par_iter().map(|elve| elve.next(round, self)).collect()
    }

    fn parse(input: &str) -> Elves {
        let mut elves = Elves::with_capacity(5_000);
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
    #[inline(always)]
    fn edges(&self) -> (i64, i64, i64, i64) {
        (
            self.iter().map(|e| e.y).min().unwrap(),
            self.iter().map(|e| e.x).max().unwrap(),
            self.iter().map(|e| e.y).max().unwrap(),
            self.iter().map(|e| e.x).min().unwrap(),
        )
    }

    fn print(&self, round: usize, end_min_y: i64, end_min_x: i64) {
        let (min_y, max_x, max_y, min_x) = self.edges();

        let extra_x_space = 4;
        let extra_y_space = 3;

        let y_correct_range = 0..((min_y - end_min_y).abs() + extra_y_space);
        let x_correct_range = 0..((min_x - end_min_x).abs() + extra_x_space);
        // ensure that the visual remains aligned when the bounds expand
        y_correct_range.for_each(|_| println!());

        x_correct_range.clone().for_each(|_| print!("  "));
        print!("\x1b[38;5;29m┏");
        (min_x..=(max_x)).for_each(|_| print!("━━"));

        println!("━┓\x1b[0m");
        for y in min_y..=max_y {
            x_correct_range.clone().for_each(|_| print!("  "));
            print!("\x1b[38;5;29m┃ ");
            for x in min_x..=max_x {
                if self.contains(&Elve::new(x, y)) {
                    print!("\x1b[93m⬤ \x1b[0m");
                } else if x == 0 && y == 0 {
                    print!("\x1b[38;5;246m∘ \x1b[0m");
                } else {
                    print!("\x1b[38;5;240m∘ \x1b[0m");
                }
            }
            println!("\x1b[38;5;29m┃");
        }

        x_correct_range.for_each(|_| print!("  "));
        print!("┗━");
        let end = format!(" R-{:0width$} ", round, width = 3);
        (0..=((max_x - min_x) * 2 - end.len() as i64)).for_each(|_| print!("━"));
        print!("\x1b[1;38;5;160m{}\x1b[0m", end);
        println!("\x1b[38;5;29m━┛\x1b[0m");
    }

    fn part_1(&self) -> i64 {
        let (min_y, max_x, max_y, min_x) = self.edges();
        (max_x - min_x + 1) * (max_y - min_y + 1) - self.len() as i64
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    let input = challenge_input();
    let mut elves = Elves::parse(&input);
    let mut part_1 = None;
    let mut part_2 = None;

    let mut smallest_x = 0;
    let mut smallest_y = 0;

    // clear_screen();
    // println!();
    // println!();
    // println!();
    // println!("     *** Merry Christmas! ***");

    for round in 0.. {
        if round == 11 {
            part_1 = Some(elves.part_1());
        }
        let next_elves = elves.next(round);
        if next_elves == elves {
            part_2 = Some(round + 1);
            break;
        }
        // next_elves.print(round, -10, -10);

        // update the smallest_x and smallest_y we've encountered
        elves = elves.next(round);
        // let (min_y, _, _, min_x) = elves.edges();
        // smallest_x = smallest_x.min(min_x);
        // smallest_y = smallest_y.min(min_y);
    }

    // reset
    // elves = Elves::parse(&input);
    // for round in 0.. {
    //     // clear the screen
    //     clear_screen();
    //     elves.print(round, smallest_y, smallest_x);
    //     let next_elves = elves.next(round);
    //     if next_elves == elves {
    //         break;
    //     }
    //     elves = elves.next(round);

    //     thread::sleep(time::Duration::from_millis(50));
    // }

    // println!();
    // println!();
    // println!();
    // println!();

    println!(
        "          Part 1: \x1b[1;38;5;160m{}\x1b[0m",
        part_1.unwrap()
    );
    println!(
        "          Part 2: \x1b[1;38;5;160m{}\x1b[0m",
        part_2.unwrap()
    );

    // println!();
    // println!();
    // println!();
    // println!();
}
