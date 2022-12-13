use std::fmt::Display;

use aoc_2022_common::challenge_input;

use nalgebra::DMatrix;

fn main() {
    let input = challenge_input();
    let matrix = parse_input(&input);
    println!("{}", matrix);
}

fn parse_input(input: &str) -> DMatrix<MapItem> {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let height_data: Vec<MapItem> = lines
        .iter()
        .flat_map(|l| {
            l.as_bytes().iter().map(|&b| match b {
                83 => MapItem::Start,
                69 => MapItem::End,
                n => MapItem::Level(u64::from(n - 96)),
            })
        })
        .collect();

    DMatrix::from_row_slice(rows, cols, &height_data)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MapItem {
    Start,
    End,
    Level(u64),
}

impl Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapItem::Start => write!(f, "S"),
            MapItem::End => write!(f, "E"),
            MapItem::Level(n) => write!(f, "{}", n),
        }
    }
}
