use std::ops::Add;

use aoc_2022_common::challenge_input;

fn main() {
    let input = parse_input(&challenge_input());
    dbg!(input);
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Point(isize, isize);

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl From<&str> for Point {
    fn from(str: &str) -> Self {
        match str {
            "R" => Point(1, 0),
            "U" => Point(0, 1),
            "L" => Point(-1, 0),
            "D" => Point(0, -1),
            _ => panic!("invalid char in output"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .flat_map(|line| {
            let (dir, count) = line.split_once(' ').unwrap();
            let count = count.parse::<i32>().unwrap();
            (0..count).map(|_| Point::from(dir)).collect::<Vec<Point>>()
        })
        .collect()
}
