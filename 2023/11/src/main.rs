use std::fmt::Display;

use aoc_2023_common::challenge_input;

type Universe = Vec<Vec<bool>>;
struct ExpandedUniverse(Universe);

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

fn parse(input: &str) -> Universe {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b == b'#').collect())
        .collect()
}

fn main() {
    let input = challenge_input();
    let universe = parse(&input);
    let expanded: ExpandedUniverse = universe.into();
    println!("{}", expanded);
    println!("{input}");
}
