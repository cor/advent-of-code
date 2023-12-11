use colored::Colorize;
use std::{collections::HashSet, fmt::Display, ops::Add};

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

    fn right(&self) -> Self {
        match &self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn left(&self) -> Self {
        match &self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}

use Direction::*;
use PipeType::*;
use Tile::*;

struct Map {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
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

impl Point {
    fn neighbors(&self) -> [Self; 4] {
        [*self + North, *self + East, *self + South, *self + West]
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

    fn print_with_path_and_floods(
        &self,
        path: &HashSet<Point>,
        left_flood: &HashSet<Point>,
        right_flood: &HashSet<Point>,
    ) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let char = match tile {
                    Pipe(EastWest) => '━',
                    Pipe(NorthSouth) => '┃',
                    Pipe(SouthEast) => '┏',
                    Pipe(SouthWest) => '┓',
                    Pipe(NorthEast) => '┗',
                    Pipe(NorthWest) => '┛',
                    Start => 'S',
                    Ground => '.',
                };

                let point = Point {
                    x: x as i16,
                    y: y as i16,
                };

                if path.contains(&point) {
                    print!("{}", char.to_string().green());
                } else if left_flood.contains(&point) {
                    print!("{}", char.to_string().red());
                } else if right_flood.contains(&point) {
                    print!("{}", char.to_string().blue());
                } else {
                    print!("{}", char);
                }
            }
            println!();
        }
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

fn flood_fill(path: &HashSet<Point>, start: Point) -> Option<HashSet<Point>> {
    let mut flood = HashSet::new();
    let mut latest = HashSet::new();

    if path.contains(&start) {
        // We're attempting to flood on the path, return an empty flood
        return Some(HashSet::new());
    }

    flood.insert(start);
    latest.insert(start);

    for _ in 0..50 {
        if latest.is_empty() {
            // we have flooded the entire area
            return Some(flood);
        }

        let new: HashSet<Point> = latest
            .iter()
            .flat_map(|point| point.neighbors())
            .filter(|point| !flood.contains(point) && !path.contains(point))
            .collect();

        flood.extend(&new);
        latest = new;
    }

    // If after 50 cycles we are still expanding, it's the wrong side
    None
}

// Hardcoded because I didn't bother to check what the start direction is algorithmically.
const START_DIR: Direction = North;

fn main() {
    let input = challenge_input();
    let map = Map::parse(&input);
    let start = map.start_point();

    // PART 1: Determine the path length and create the path set
    // ---------------------------------------------------------

    let mut current_point = start;
    let mut current_dir = START_DIR;
    let mut path_set: HashSet<Point> = HashSet::new();

    path_set.insert(start);
    loop {
        current_point = current_point + current_dir;
        path_set.insert(current_point);
        match map.get(&current_point) {
            Pipe(pipe_type) => {
                let (dir1, dir2) = pipe_type.directions();
                current_dir = if current_dir.inverse() == dir1 {
                    dir2
                } else {
                    dir1
                };
            }
            Start => break,
            Ground => panic!("hit the ground while traversing path"),
        }
    }

    let part_1 = (path_set.len() + 1) / 2;

    // PART 2: Flood both sides to determine surface area within the loop
    // ------------------------------------------------------------------

    let mut current_point = start;
    let mut current_dir = START_DIR;
    let mut left_floods: HashSet<Point> = HashSet::new();
    let mut right_floods: HashSet<Point> = HashSet::new();

    enum OverflowSide {
        Left,
        Right,
    }
    let mut overflow_side: Option<OverflowSide> = None;

    loop {
        let left = current_point + current_dir.left();
        let right = current_point + current_dir.right();
        let left_next = current_point + current_dir + current_dir.left();
        let right_next = current_point + current_dir + current_dir.right();

        match flood_fill(&path_set, left) {
            Some(flood) => left_floods.extend(flood),
            None => overflow_side = Some(OverflowSide::Left),
        }

        match flood_fill(&path_set, right) {
            Some(flood) => right_floods.extend(flood),
            None => overflow_side = Some(OverflowSide::Right),
        }

        match flood_fill(&path_set, left_next) {
            Some(flood) => left_floods.extend(flood),
            None => overflow_side = Some(OverflowSide::Left),
        }

        match flood_fill(&path_set, right_next) {
            Some(flood) => right_floods.extend(flood),
            None => overflow_side = Some(OverflowSide::Right),
        }

        current_point = current_point + current_dir;

        match map.get(&current_point) {
            Pipe(pipe_type) => {
                let (dir1, dir2) = pipe_type.directions();
                current_dir = if current_dir.inverse() == dir1 {
                    dir2
                } else {
                    dir1
                };
            }
            Start => break,
            Ground => panic!("hit the ground while traversing path"),
        }
    }

    map.print_with_path_and_floods(&path_set, &left_floods, &right_floods);

    println!("{}", part_1);
    match overflow_side {
        Some(OverflowSide::Left) => println!("{}", right_floods.len()),
        Some(OverflowSide::Right) => println!("{}", left_floods.len()),
        None => panic!("Neither side overflowed, can't determine inside of loop"),
    }
}
