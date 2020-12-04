use std::fs::File;
use std::io::{Read};
use std::ops::{AddAssign, Index};
use std::str::FromStr;

fn load_file(path: &str) -> String {
    let mut input = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut input).expect("Unable to read string");

    input
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}
type Slope = Point;

impl Point {
    const START: Point = Point { x: 0, y: 0 };
}

impl AddAssign<&Slope> for Point {
    fn add_assign(&mut self, slope: &Slope) {
        *self = Point {
            x: self.x + slope.x,
            y: self.y + slope.y,
        };
    }
}

#[derive(Debug)]
enum Square {
    Open,
    Tree,
}

#[derive(Debug)]
struct World {
    width: usize,
    height: usize,
    source: Vec<Square>,
}

impl World {
    fn point_in_bounds(&self, point: &Point) -> bool {
        point.y < self.height
    }

    fn count_trees_with(&self, slope: &Slope) -> usize {
        let mut current_point = Point::START;
        let mut tree_count = 0;

        while self.point_in_bounds(&current_point) {
            tree_count += match self[&current_point] {
                Square::Open => 0,
                Square::Tree => 1,
            };
            current_point += slope;
        }

        tree_count
    }
}

impl FromStr for World {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let width = lines[0].len();
        let height = lines.len();
        let mut source = Vec::with_capacity(width * height);

        for line in lines {
            for square in line.chars() {
                let square = match square {
                    '.' => Square::Open,
                    '#' => Square::Tree,
                    _ => panic!("Invalid character in input"),
                };
                source.push(square);
            }
        }

        Ok(World {
            width,
            height,
            source,
        })
    }
}

impl Index<&Point> for World {
    type Output = Square;

    fn index(&self, index: &Point) -> &Self::Output {
        &self.source[index.x % self.width + index.y * self.width]
    }
}

fn main() {
    let input = load_file("./input/1.txt");
    let world = World::from_str(&input).expect("Couldn't parse world.");

    // Part 1
    let answer1 = world.count_trees_with(&Slope { x: 3, y: 1 });
    println!("{:#?}", answer1);

    // Part 2
    let slopes = [
        Slope { x: 1, y: 1 },
        Slope { x: 3, y: 1 },
        Slope { x: 5, y: 1 },
        Slope { x: 7, y: 1 },
        Slope { x: 1, y: 2 },
    ];

    let answer2 = slopes
        .iter()
        .map(|slope| world.count_trees_with(&slope))
        .product::<usize>();
    println!("{:#?}", answer2);
}
