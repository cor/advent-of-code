use std::{collections::HashSet, fmt::Display};

use aoc_2023_common::challenge_input;

type Universe = Vec<Vec<bool>>;
struct ExpandedUniverse<const Expansion: usize>(Universe);
type Galaxies = HashSet<(usize, usize)>;

impl<const expansion: usize> From<Universe> for ExpandedUniverse<expansion> {
    fn from(original: Universe) -> Self {
        let mut expanded = Vec::new();

        // expand rows
        for row in original {
            expanded.push(row.clone());
            if row.iter().all(|b| !b) {
                for _ in 0..expansion {
                    expanded.push(row.clone());
                }
            }
        }

        // expand columns
        let mut i = 0;
        while expanded[0].get(i).is_some() {
            if expanded.iter().all(|row| !row[i]) {
                for _ in 0..expansion {
                    for row in &mut expanded {
                        row.insert(i, false);
                    }
                    i += 1
                }
            }
            i += 1
        }

        Self(expanded)
    }
}

impl<const N: usize> From<ExpandedUniverse<N>> for Galaxies {
    fn from(universe: ExpandedUniverse<N>) -> Self {
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

impl<const N: usize> Display for ExpandedUniverse<N> {
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
    let expanded_1: ExpandedUniverse<1> = universe.clone().into();
    let galaxies_1: Galaxies = expanded_1.into();
    let part_1 = distances(galaxies_1);

    let expanded_2: ExpandedUniverse<1_000_000> = universe.into();
    let galaxies_2: Galaxies = expanded_2.into();
    let part_2 = distances(galaxies_2);

    println!("{part_1}");
    println!("{part_2}");
}
