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
    fn push(&mut self, direction: (isize, isize)) {
        let player_neighbor = (
            (self.player.0 as isize + direction.0) as usize,
            (self.player.1 as isize + direction.1) as usize,
        );

        match self.tiles[player_neighbor] {
            Wall => return,
            Empty => {
                self.player = player_neighbor;
                return;
            }
            Box => (),
        };

        let mut current_point = player_neighbor;
        loop {
            current_point = (
                (current_point.0 as isize + direction.0) as usize,
                (current_point.1 as isize + direction.1) as usize,
            );
            match self.tiles[current_point] {
                Wall => return, // hit a wall, no box/player moving
                Empty => {
                    // empty, "move" the boxes by swapping point_right and current_point
                    self.tiles[current_point] = Box;
                    self.tiles[player_neighbor] = Empty;
                    self.player = player_neighbor;
                    return;
                }
                Box => continue, // we keep looking for wall/empty
            }
        }
    }

    fn gps_coordinate_sum(&self) -> usize {
        self.tiles
            .map_with_location(|row, col, tile| match tile {
                Box => row * 100 + col,
                _ => 0,
            })
            .sum()
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
    let (map_str, instructions) = input.split_once("\n\n").unwrap();
    let mut map: Map = map_str.into();

    for instr in instructions.chars() {
        match instr {
            '>' => map.push((0, 1)),
            '^' => map.push((-1, 0)),
            '<' => map.push((0, -1)),
            'v' => map.push((1, 0)),
            '\n' => (),
            _ => println!("end of simulation"),
        }
    }
    println!("{}", map.gps_coordinate_sum())
}
