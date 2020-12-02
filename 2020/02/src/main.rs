use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn read_lines<R: Read>(io: R) -> Result<Vec<String>, Error> {
    BufReader::new(io).lines().collect()
}

#[derive(Debug)]
struct PasswordTest<'a> {
    lower: u64,
    upper: u64,
    character: char,
    password: &'a str,
}

fn main() {
    let file = File::open("./input/01.txt").expect("Failed to open file");
    let lines = read_lines(file).expect("Incorrect input");

    // Parse input into PasswordTests
    let tests: Vec<PasswordTest> = lines.iter()
        .map(|line| line.split(": ").collect())
        .map(|line : Vec<&str> | {
            let requirements: Vec<&str> = line[0].split(' ').collect();
            let range: Vec<&str> = requirements[0].split('-').collect();
            let character = requirements[1].chars().nth(0)
                .expect("Invalid char in input");

            PasswordTest {
                lower: range[0].parse().expect("Invalid lower bound in input"),
                upper: range[1].parse().expect("Invalid upper bound in input"),
                character,
                password: line[1],
            }
        })
        .collect();

    // Check how many PasswordTests pass
    let part1 = tests.iter().filter(|test| {
        let char_count: u64 = test.password.matches(test.character).count() as u64;
        char_count >= test.lower && char_count <= test.upper
    }).count();

    // Check how many PasswordTests pass
    let part2 = tests.iter().filter(|test| {
        let chars = test.password.as_bytes();
        let target = test.character as u8;
        let lower = (test.lower - 1) as usize;
        let upper = (test.upper - 1) as usize;

        (chars[lower] == target) ^ (chars[upper] == target)
    }).count();

    println!("{}", part2);
}
