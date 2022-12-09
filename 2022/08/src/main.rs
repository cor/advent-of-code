use std::{convert::TryInto, ops};

use aoc_2022_common::challenge_input;
use nalgebra::DMatrix;

#[derive(Debug, Eq, PartialEq)]
struct HeightMap(Vec<Vec<u8>>);

#[derive(Debug, Eq, PartialEq)]
struct VisibilityMap(Vec<Vec<bool>>);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Point(isize, isize);

const DIRECTIONS: [Point; 4] = [Point(0, 1), Point(0, -1), Point(1, 0), Point(-1, 0)];

fn main() {
    let height_map = to_height_map(challenge_input());
    let visibility_map =
        height_map.map_with_location(|row, col, _| Point::from((row, col)).is_visible(&height_map));
    let scenic_map = height_map
        .map_with_location(|row, col, _| Point::from((row, col)).scenic_score(&height_map));

    println!("{}", visibility_map.iter().filter(|v| **v).count());
    println!("{}", scenic_map.iter().max().unwrap());
}

impl Point {
    pub fn is_visible(&self, height_map: &DMatrix<u32>) -> bool {
        let self_height = self.on_map(height_map).expect("out of bounds");

        'directions: for dir in DIRECTIONS {
            let mut current_pos = *self;
            loop {
                current_pos = current_pos + dir;
                match current_pos.on_map(height_map) {
                    Some(h) if h >= self_height => continue 'directions,
                    Some(_) => {} // tree is lower than self
                    None => return true,
                }
            }
        }
        false
    }

    pub fn scenic_score(&self, height_map: &DMatrix<u32>) -> u32 {
        let self_height = self.on_map(height_map).expect("out of bounds");

        DIRECTIONS
            .iter()
            .map(|&dir| {
                let mut score = 0;
                let mut current_pos = *self + dir;
                while let Some(h) = current_pos.on_map(height_map) {
                    score += 1;

                    if h >= self_height {
                        break;
                    }

                    current_pos = current_pos + dir;
                }

                score
            })
            .product()
    }

    pub fn on_map<T: Copy>(&self, map: &DMatrix<T>) -> Option<T> {
        let column: Option<usize> = self.1.try_into().ok();
        let row: Option<usize> = self.0.try_into().ok();
        let upoint = (row, column);

        match upoint {
            (Some(y), Some(x)) => map.get((y, x)).copied(),
            (None, None) | (None, Some(_)) | (Some(_), None) => None,
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from(point: (usize, usize)) -> Self {
        Point(point.0 as isize, point.1 as isize)
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn to_height_map(input: String) -> DMatrix<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let height_data: Vec<u32> = lines
        .iter()
        .flat_map(|l| {
            l.as_bytes()
                .iter()
                .map(|b| (*b - 48) as u32)
                .collect::<Vec<_>>()
        })
        .collect();

    DMatrix::from_row_slice(rows, cols, &height_data)
}
