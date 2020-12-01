use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read_numbers<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn find_answer(numbers: Vec<i64>) -> Option<i64> {
    for element1 in numbers.iter() {
        for element2 in numbers.iter() {
            if element1 + element2 == 2020 {
                return Some(element1 * element2);
            }
        }
    }
    return None;
}

fn main() {
    let file = File::open("./input/01.txt").expect("Failed to open file");
    let numbers = read_numbers(file).expect("Failed to parse file");

    match find_answer(numbers) {
        Some(n) => println!("{}", n),
        None => println!("Input doesn't contain solution"),
    }
}
