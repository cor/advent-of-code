use std::fmt;
use std::fmt::Display;

use aoc_2024_common::challenge_input;
use nalgebra::DMatrix;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Map {
    player: (usize, usize),
    tiles: DMatrix<Tile>,
}

impl Map {
    fn push_right(&mut self) {
        let (y, x) = self.player;
        let point_right = (y, x + 1);

        match self.tiles[point_right] {
            Wall => return,
            Empty => {
                self.player = point_right;
                return;
            }
            Box => (),
        };

        let mut current_point = point_right;
        loop {
            current_point.1 += 1;
            match self.tiles[current_point] {
                Wall => return, // hit a wall, no box/player moving
                Empty => {
                    // empty, "move" the boxes by swapping point_right and current_point
                    self.tiles[current_point] = Box;
                    self.tiles[point_right] = Empty;
                    self.player = point_right;
                    return;
                }
                Box => continue, // we keep looking for wall/empty
            }
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tiles)?;
        write!(f, "x: {}, y: {}", self.player.0, self.player.1)
    }
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
                        player = (y, x);
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
    let mut map: Map = map_str.into();
    println!("{map}");
    map.push_right();
    println!("{map}");
    map.push_right();
    println!("{map}");
    map.push_right();
    println!("{map}");
    map.push_right();
    println!("{map}");
}
