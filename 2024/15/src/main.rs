use std::fmt;
use std::fmt::Display;

use aoc_2024_common::challenge_input;
use nalgebra::DMatrix;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Map {
    player: (usize, usize),
    tiles: DMatrix<Tile>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut tiles = Vec::new();
        let mut player = (0, 0);
        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    '#' => tiles.push(Wall),
                    '.' => tiles.push(Empty),
                    'O' => tiles.push(Box),
                    '@' => {
                        player = (x, y);
                        tiles.push(Empty)
                    }
                    c => panic!("Invalid item {c} on map"),
                }
            }
            // could be more efficient
            width = line.len();
            height = y + 1;
        }

        let tiles = DMatrix::from_row_slice(height, width, &tiles);
        println!("{tiles}");
        Self { tiles, player }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Box,
}
impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Wall => write!(f, "#"),
            Empty => write!(f, "."),
            Box => write!(f, "O"),
        }
    }
}

use Tile::*;

fn main() {
    let input = challenge_input();
    println!("{input}");
    let (map_str, instructions) = input.split_once("\n\n").unwrap();
    let map: Map = map_str.into();

    // let map = input
    //     .lines()
    //     .map(|l| {
    //         l.chars()
    //             .map(|c| match c {
    //                 '#' => Wall,
    //                 '.' => Empty,
    //                 'O' => Box,
    //                 '@' => Player,
    //                 _ => panic!("Invalid item on map"),
    //             })
    //             .collect::<Vec<_>>()
    //     })
    //     .collect::<Vec<_>>();

    // let row_count = map.len();
    // let column_count = map[0].len();
    // let map = map.into_iter().flatten().collect::<Vec<_>>();
    // let map = DMatrix::<Tile>::from_vec(row_count, column_count, map);
    // dbg!(map);
    // dbg!(instructions);
}
