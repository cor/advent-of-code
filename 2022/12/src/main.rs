use std::{collections::HashSet, fmt::Display};

use aoc_2022_common::challenge_input;

use derive_more::{Add, AddAssign};
use nalgebra::DMatrix;

// copied form day 8
#[derive(Debug, Eq, PartialEq, Clone, Copy, Add, AddAssign, Hash)]
struct Point(isize, isize);

const DIRECTIONS: [Point; 4] = [Point(0, 1), Point(0, -1), Point(1, 0), Point(-1, 0)];

fn main() {
    let input = challenge_input();
    let map = parse_input(&input);

    let start_index = map.iter().position(|&c| c == MapItem::Start).unwrap();
    let start = Point::from_matrix_index(start_index, map.nrows());

    let part_1 = start.steps_to_end(&map).expect("no part 1 solution");
    println!("{part_1}");

    // Can be made faster by caching visited startpoints or by multithreading
    let part_2 = map
        .iter()
        .enumerate()
        .filter(|(_, &item)| item == MapItem::Start || item == MapItem::Level(1))
        .map(|(i, _)| Point::from_matrix_index(i, map.ncols()))
        .filter_map(|p| p.steps_to_end(&map))
        .min()
        .expect("no part 2 solution");
    println!("{part_2}");
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

impl Point {
    /// [`nalgebra`] probably has a better way to do this, but I couldn't find it in the docs.
    #[allow(clippy::cast_possible_wrap)]
    pub fn from_matrix_index(index: usize, matrix_width: usize) -> Self {
        Point(
            (index / matrix_width) as isize,
            (index % matrix_width) as isize,
        )
    }

    fn steps_to_end(&self, map: &DMatrix<MapItem>) -> Option<usize> {
        let mut steps = Vec::<HashSet<Point>>::new();
        let mut visited = HashSet::<Point>::new();
        visited.insert(*self);
        steps.push(visited.clone());

        for step_number in 1.. {
            let next_steps: HashSet<Point> = steps
                .last()
                .expect("no last steps")
                .iter()
                .flat_map(|s| s.neighbors_to_go_to(map))
                .filter(|s| !visited.contains(s))
                .collect();

            if next_steps.is_empty() {
                // It is impossible to go to the end from this starting position
                // as we have not reached the end yet, but there are no next steps.
                return None;
            }

            if next_steps
                .iter()
                .any(|&s| s.on_map(map) == Some(MapItem::End))
            {
                return Some(step_number);
            }

            visited.extend(&next_steps);
            steps.push(next_steps);
        }
        unreachable!();
    }

    pub fn neighbors_to_go_to(&self, map: &DMatrix<MapItem>) -> Vec<Self> {
        let self_item = self.on_map(map).expect("querying neighbors outside of map");
        DIRECTIONS
            .iter()
            .filter(|&&dir| {
                (*self + dir)
                    .on_map(map)
                    .map_or(false, |neighbor_item| self_item.can_move_to(neighbor_item))
            })
            .map(|&dir| *self + dir)
            .collect()
    }

    pub fn on_map<T: Copy>(&self, map: &DMatrix<T>) -> Option<T> {
        let column: Option<usize> = self.0.try_into().ok();
        let row: Option<usize> = self.1.try_into().ok();
        let upoint = (row, column);

        match upoint {
            (Some(y), Some(x)) => map.get((y, x)).copied(),
            (None | Some(_), None) | (None, Some(_)) => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MapItem {
    Start,
    End,
    Level(u64),
}

impl MapItem {
    pub fn as_level(&self) -> u64 {
        match self {
            MapItem::Start => 1,
            MapItem::End => 26,
            MapItem::Level(l) => *l,
        }
    }
    pub fn can_move_to(&self, other: MapItem) -> bool {
        other.as_level() <= self.as_level() + 1
    }
}

impl Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapItem::Start => write!(f, "S"),
            MapItem::End => write!(f, "E"),
            MapItem::Level(n) => write!(f, "{n}"),
        }
    }
}
