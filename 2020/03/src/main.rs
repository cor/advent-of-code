use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::thread::current;

fn read_lines<R: Read>(io: R) -> Result<Vec<String>, Error> {
    BufReader::new(io).lines().collect()
}
#[derive(Debug)]
struct World {
    width: usize,
    height: usize,
    source: Vec<Square>,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn start() -> Point {
        Point {
            x: 0,
            y: 0,
        }
    }

    fn add_slope(&self, slope: &Slope) -> Point {
        Point {
            x: self.x + slope.x,
            y: self.y + slope.y,
        }
    }
}

type Slope = Point;

#[derive(Debug)]
enum Square {
    Open,
    Tree,
}

impl World {
    fn square_at(&self, point: &Point) -> &Square {
        let x = point.x % self.width; // repeat to the right
        &self.source[x + point.y * self.width]
    }

    fn count_trees_with_slope(&self, slope: &Slope) -> usize {
        let mut tree_count = 0;
        let mut current_point = Point::start();

        while current_point.y < self.height{
            tree_count += match self.square_at(&current_point) {
                Square::Open => 0,
                Square::Tree => 1,
            };
            current_point = current_point.add_slope(slope);
        }

        tree_count
    }

    fn from_lines(lines: &Vec<String>) -> World {
        let width = lines[0].len();
        let height = lines.len();
        let mut source = Vec::with_capacity(width * height);

        for line in lines {
            for square in line.as_bytes() {
                let square = match square {
                    b'.' => Square::Open,
                    b'#' => Square::Tree,
                    _    => panic!("Invalid character in input"),
                };
                source.push(square);
            }
        }

        World {
            width,
            height,
            source,
        }
    }
}


fn main() {
    let file = File::open("./input/1.txt").expect("Failed to open file");
    let lines = read_lines(file).expect("Incorrect input");
    let world = World::from_lines(&lines);

    let slope = Slope { x: 3, y: 1 };
    println!("{:#?}", world.count_trees_with_slope(&slope));
}
