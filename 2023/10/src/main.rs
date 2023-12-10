use std::fmt::Display;

use aoc_2023_common::challenge_input;

enum Tile {
    Start,
    Ground,
    Pipe(Pipe),
}
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}
use Pipe::*;
use Tile::*;

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|b| match b {
                        b'-' => Pipe(EastWest),
                        b'|' => Pipe(NorthSouth),
                        b'F' => Pipe(SouthEast),
                        b'7' => Pipe(SouthWest),
                        b'L' => Pipe(NorthEast),
                        b'J' => Pipe(NorthWest),
                        b'S' => Start,
                        _ => Ground,
                    })
                    .collect()
            })
            .collect();

        Self { tiles }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                let char = match tile {
                    Pipe(EastWest) => '━',
                    Pipe(NorthSouth) => '┃',
                    Pipe(SouthEast) => '┏',
                    Pipe(SouthWest) => '┓',
                    Pipe(NorthEast) => '┗',
                    Pipe(NorthWest) => '┛',
                    Start => 'S',
                    Ground => ' ',
                };
                write!(f, "{}", char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let input = challenge_input();
    let map = Map::parse(&input);
    println!("{}", map);
}
