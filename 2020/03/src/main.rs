use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn read_lines<R: Read>(io: R) -> Result<Vec<String>, Error> {
    BufReader::new(io).lines().collect()
}
#[derive(Debug)]
struct World {
    width: usize,
    height: usize,
    source: Vec<Square>,
}

impl World {
    fn square_at(&self, x: usize, y: usize) -> &Square {
        let x = x % self.width; // repeat to the right
        &self.source[x + y * self.width]
    }
}

#[derive(Debug)]
enum Square {
    Open,
    Tree,
}

impl World {
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

    println!("{:#?}", world.square_at(36, 5));
}
