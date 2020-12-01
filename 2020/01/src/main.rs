use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

#[macro_use]
extern crate itertools;

fn read_numbers<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn find_answer(numbers: Vec<i64>) -> Option<i64> {
    for (i, j) in iproduct!(&numbers, &numbers) {
        if i + j == 2020 {
            return Some(i * j);
        }
    }
    return None;
}

fn find_answer_part2(numbers: impl Iterator<Item = i64> + Copy) -> Option<i64> {
    for (i, j, k) in iproduct!(numbers, numbers, numbers) {
        if i + j + k == 2020 {
            return Some(i * j);
        }
    }
    return None;
}

fn main() {
    let file = File::open("./input/01.txt").expect("Failed to open file");
    let numbers = read_numbers(file).expect("Failed to parse file");

    match find_answer_part2(numbers.into_iter()) {
        Some(n) => println!("{}", n),
        None => println!("Input doesn't contain solution"),
    }
}
