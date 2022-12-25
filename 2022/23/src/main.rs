use rayon::prelude::*;
use std::{collections::HashSet, ops::Add};

use aoc_2022_common::challenge_input;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point2 {
    x: i16,
    y: i16,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Dir {
    N,
    S,
    W,
    E,
    NE,
    NW,
    SE,
    SW,
    STAY,
}

use Dir::{E, N, NE, NW, S, SE, STAY, SW, W};

impl Add<Dir> for &Point2 {
    type Output = Point2;

    fn add(self, rhs: Dir) -> Self::Output {
        let mut added = *self;
        match rhs {
            N => added.y -= 1,
            S => added.y += 1,
            W => added.x -= 1,
            E => added.x += 1,
            NE => {
                added.y -= 1;
                added.x += 1;
            }
            NW => {
                added.y -= 1;
                added.x -= 1;
            }
            SE => {
                added.y += 1;
                added.x += 1;
            }
            SW => {
                added.y += 1;
                added.x -= 1;
            }
            STAY => {}
        }

        added
    }
}

type Elve = Point2;
type Elves = HashSet<Elve>;

const MAIN_DIRS: [Dir; 4] = [N, S, W, E];
const SCANS: [(u8, Dir); 4] = [
    (0b0000_0111, N),
    (0b0111_0000, S),
    (0b1100_0001, W),
    (0b0001_1100, E),
];

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
        let around_scan = ((others.contains(&(self + NW)) as u8) << 0)
            + ((others.contains(&(self + N)) as u8) << 1)
            + ((others.contains(&(self + NE)) as u8) << 2)
            + ((others.contains(&(self + E)) as u8) << 3)
            + ((others.contains(&(self + SE)) as u8) << 4)
            + ((others.contains(&(self + S)) as u8) << 5)
            + ((others.contains(&(self + SW)) as u8) << 6)
            + ((others.contains(&(self + W)) as u8) << 7);

        if around_scan == 0 {
            return STAY;
        }

        for i in 0..SCANS.len() {
            let (scan, dir) = SCANS[(round + i) % (SCANS.len())];
            if around_scan & scan == 0 {
                return dir;
            }
        }

        STAY
    }

    #[inline(always)]
    fn next(&self, round: usize, others: &Elves) -> Elve {
        let prop_dir = self.proposed_dir(round, others);

        if prop_dir == STAY {
            return *self;
        }

        let test = |candidate: Elve, dir: Dir| {
            others.contains(&candidate) && candidate.proposed_dir(round, others) == dir
        };

        for main_dir in MAIN_DIRS {
            if main_dir == opposite_dir(prop_dir) {
                continue;
            }

            // Another elve also wants to go to our spot, so we won't go there.
            if test(&(self + prop_dir) + main_dir, opposite_dir(main_dir)) {
                return *self;
            }
        }
        self + prop_dir
    }
}

trait ElvesExt {
    fn next(&self, round: usize) -> Elves;
    fn parse(input: &str) -> Elves;
    fn edges(&self) -> (i16, i16, i16, i16);
    fn print(&self, round: usize, end_min_y: i16, end_min_x: i16);
    fn part_1(&self) -> i16;
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
                        elves.insert(Elve {
                            x: x as i16,
                            y: y as i16,
                        });
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
    fn edges(&self) -> (i16, i16, i16, i16) {
        (
            self.iter().map(|e| e.y).min().unwrap(),
            self.iter().map(|e| e.x).max().unwrap(),
            self.iter().map(|e| e.y).max().unwrap(),
            self.iter().map(|e| e.x).min().unwrap(),
        )
    }

    fn print(&self, round: usize, end_min_y: i16, end_min_x: i16) {
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
                if self.contains(&Elve { x, y }) {
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
        (0..=((max_x - min_x) * 2 - end.len() as i16)).for_each(|_| print!("━"));
        print!("\x1b[1;38;5;160m{}\x1b[0m", end);
        println!("\x1b[38;5;29m━┛\x1b[0m");
    }

    fn part_1(&self) -> i16 {
        let (min_y, max_x, max_y, min_x) = self.edges();
        (max_x - min_x + 1) * (max_y - min_y + 1) - self.len() as i16
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
