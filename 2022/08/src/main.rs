use std::{convert::TryInto, ops};

use aoc_2022_common::challenge_input;
use nalgebra::DMatrix;

#[derive(Debug, Eq, PartialEq)]
struct HeightMap(Vec<Vec<u8>>);

#[derive(Debug, Eq, PartialEq)]
struct VisibilityMap(Vec<Vec<bool>>);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Point(isize, isize);

fn main() {
    let height_map = to_height_map(challenge_input());
    let visibility_map =
        height_map.map_with_location(|row, col, _| Point::from((row, col)).is_visible(&height_map));

    println!("{height_map}");
    println!("{visibility_map}");

    // println!("{}"height_map);
    // println!(visiblity_map.display());

    // let height = height_map.0.len();
    // let width = height_map.0[0].len();

    // // let height_map = HeightMap::from(challenge_input().as_ref());
    // // let mut visibility_map = VisibilityMap::from(&height_map);

    // // TODO: Cleanup by abstracting over scan direction

    // for y in 0..height {
    //     let mut current_height = None;
    //     for x in 0..width {
    //         let new_height = Some(height_map.0[y][x]);
    //         if new_height > current_height {
    //             visibility_map.0[y][x] = true;
    //             current_height = new_height;
    //         }
    //     }
    // }

    // for y in 0..height {
    //     let mut current_height = None;
    //     for x in (0..width).rev() {
    //         let new_height = Some(height_map.0[y][x]);
    //         if new_height > current_height {
    //             visibility_map.0[y][x] = true;
    //             current_height = new_height;
    //         }
    //     }
    // }

    // for x in 0..width {
    //     let mut current_height = None;
    //     for y in 0..height {
    //         let new_height = Some(height_map.0[y][x]);
    //         if new_height > current_height {
    //             visibility_map.0[y][x] = true;
    //             current_height = new_height;
    //         }
    //     }
    // }

    // for x in 0..width {
    //     let mut current_height = None;
    //     for y in (0..height).rev() {
    //         let new_height = Some(height_map.0[y][x]);
    //         if new_height > current_height {
    //             visibility_map.0[y][x] = true;
    //             current_height = new_height;
    //         }
    //     }
    // }

    // println!("{}", visibility_map.count());
}

impl Point {
    pub fn is_visible(&self, map: &DMatrix<u8>) -> bool {
        let directions = [Point(0, 1), Point(0, -1), Point(1, 0), Point(-1, 0)];
        directions
            .iter()
            .filter_map(|p| (*self + *p).on_map(map))
            .count()
            == 4
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

fn to_height_map(input: String) -> DMatrix<u8> {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let height_data: Vec<u8> = lines
        .iter()
        .flat_map(|l| l.as_bytes().iter().map(|b| *b - 48).collect::<Vec<_>>())
        .collect();

    DMatrix::from_row_slice(rows, cols, &height_data)
}
