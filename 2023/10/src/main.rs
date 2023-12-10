use std::{fmt::Display, ops::Add};

use aoc_2023_common::challenge_input;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Start,
    Ground,
    Pipe(PipeType),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum PipeType {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl PipeType {
    fn directions(&self) -> (Direction, Direction) {
        match self {
            NorthSouth => (North, South),
            EastWest => (East, West),
            NorthEast => (North, East),
            NorthWest => (North, West),
            SouthWest => (South, West),
            SouthEast => (South, East),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn inverse(&self) -> Self {
        match &self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

use Direction::*;
use PipeType::*;
use Tile::*;

struct Map {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: i16,
    y: i16,
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
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

    fn get(&self, &Point { x, y }: &Point) -> Tile {
        if x < 0 || y < 0 {
            return Ground;
        }

        match self
            .tiles
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
        {
            Some(t) => *t,
            None => Ground,
        }
    }

    fn start_point(&self) -> Point {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile == &Start {
                    return Point {
                        x: x as i16,
                        y: y as i16,
                    };
                }
            }
        }
        panic!("no start point");
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
    let start = map.start_point();
    let mut current_point = start;
    let mut current_dir = West;

    for i in 0.. {
        current_point = current_point + current_dir;
        let tile = map.get(&current_point);

        if let Pipe(pt) = tile {
            let (dir1, dir2) = pt.directions();
            current_dir = if current_dir.inverse() == dir1 {
                dir2
            } else {
                dir1
            };
        } else {
            println!("{}", (i + 1) / 2);
            break;
        }
    }
}
