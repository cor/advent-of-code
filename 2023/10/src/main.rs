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

    fn print_with_path(&self, path: &HashSet<Point>) {
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
                } else {
                    print!("{}", char);
                }
            }
            println!();
        }
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

    // If after 100 cycles we are still expanding, it's the wrong side
    None
}

const START_DIR: Direction = West;

fn main() {
    let input = challenge_input();
    let map = Map::parse(&input);
    let start = map.start_point();
    let mut current_point = start;
    let mut current_dir = START_DIR;

    let mut path_set: HashSet<Point> = HashSet::new();

    path_set.insert(start);
    for i in 0.. {
        current_point = current_point + current_dir;
        path_set.insert(current_point);
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
        // map.print_with_path(&path_set);
    }

    // PART 2
    //------------------------

    let mut current_point = start;
    let mut current_dir = START_DIR;

    let mut left_set: HashSet<Point> = HashSet::new();
    let mut right_set: HashSet<Point> = HashSet::new();

    loop {
        let left = current_point + current_dir.left();
        let right = current_point + current_dir.right();

        if let Some(set) = flood_fill(&path_set, left) {
            left_set.extend(set)
        } else {
            println!("left flooded");
        }

        if let Some(set) = flood_fill(&path_set, right) {
            right_set.extend(set)
        } else {
            println!("right flooded");
        }

        let left = current_point + current_dir + current_dir.left();
        let right = current_point + current_dir + current_dir.right();

        if let Some(set) = flood_fill(&path_set, left) {
            left_set.extend(set)
        } else {
            println!("left flooded");
        }

        if let Some(set) = flood_fill(&path_set, right) {
            right_set.extend(set)
        } else {
            println!("right flooded");
        }

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
            break;
        }
        // map.print_with_path_and_floods(&path_set, &left_set, &right_set);
    }

    map.print_with_path_and_floods(&path_set, &left_set, &right_set);

    // sanity checks
    dbg!(left_set.intersection(&right_set).collect::<HashSet<_>>());
    dbg!(left_set.intersection(&path_set).collect::<HashSet<_>>());
    dbg!(right_set.intersection(&path_set).collect::<HashSet<_>>());

    dbg!(left_set.len());
    dbg!(right_set.len());
    // dbg!(right_set);
    // dbg!(path_set);
}
