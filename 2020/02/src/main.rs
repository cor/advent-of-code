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

impl PasswordTest<'_> {
    fn part1(&self) -> bool {
        let char_count = self.password.matches(self.character).count() as u64;

        (self.lower..(self.upper + 1)).contains(&char_count)
    }

    fn part2(&self) -> bool {
        let chars = self.password.as_bytes();
        let target = self.character as u8;
        let lower = (self.lower - 1) as usize;
        let upper = (self.upper - 1) as usize;

        (chars[lower] == target) ^ (chars[upper] == target)
    }
}

fn main() {
    let file = File::open("./input/1.txt").expect("Failed to open file");
    let lines = read_lines(file).expect("Incorrect input");

    // Parse input into PasswordTests
    let tests: Vec<PasswordTest> = lines.iter()
        .map(|line| line.split(": ").collect())
        .map(|line : Vec<&str> | {
            let requirements: Vec<&str> = line[0].split(' ').collect();
            let range: Vec<&str> = requirements[0].split('-').collect();

            PasswordTest {
                lower: range[0].parse().expect("Invalid lower bound in input"),
                upper: range[1].parse().expect("Invalid upper bound in input"),
                character: requirements[1].chars().nth(0).expect("Invalid char in input"),
                password: line[1],
            }
        })
        .collect();

    // Check how many PasswordTests pass
    let part1 = tests.iter().filter(PasswordTest::part1).count();
    let part2 = tests.iter().filter(PasswordTest::part2).count();

    println!("{}", part1);
    println!("{}", part2);
}