use std::{collections::HashSet, fmt::Display};

use aoc_2023_common::challenge_input;

type Universe = Vec<Vec<bool>>;
struct ExpandedUniverse(Universe);
type Galaxies = HashSet<(usize, usize)>;

impl From<Universe> for ExpandedUniverse {
    fn from(original: Universe) -> Self {
        let mut expanded = Vec::new();

        // expand rows
        for row in original {
            expanded.push(row.clone());
            if row.iter().all(|b| !b) {
                expanded.push(row.clone());
            }
        }

        // expand columns
        let mut i = 0;
        while expanded[0].get(i).is_some() {
            if expanded.iter().all(|row| !row[i]) {
                for row in &mut expanded {
                    row.insert(i, false);
                }
                i += 1
            }
            i += 1
        }

        Self(expanded)
    }
}

impl From<ExpandedUniverse> for Galaxies {
    fn from(universe: ExpandedUniverse) -> Self {
        let mut galaxies = HashSet::new();
        for (y, row) in universe.0.iter().enumerate() {
            for x in 0..row.len() {
                if universe.0[y][x] {
                    galaxies.insert((x, y));
                }
            }
        }
        galaxies
    }
}

impl Display for ExpandedUniverse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for column in row {
                match column {
                    true => write!(f, "#"),
                    false => write!(f, "."),
                }?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

fn manhattan_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn parse(input: &str) -> Universe {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b == b'#').collect())
        .collect()
}

fn distances(galaxies: Galaxies) -> usize {
    galaxies
        .iter()
        .flat_map(|galaxy_a| {
            galaxies
                .iter()
                .map(|galaxy_b| manhattan_distance(galaxy_a, galaxy_b))
                .collect::<Vec<_>>()
        })
        .sum::<usize>()
        / 2
}

fn main() {
    let input = challenge_input();
    let universe = parse(&input);
    let expanded: ExpandedUniverse = universe.into();
    let galaxies: Galaxies = expanded.into();
    let part_1 = distances(galaxies);

    println!("{part_1}");
}
