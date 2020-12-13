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

    fn from_str(s: &str) -> PasswordTest {
        let line: Vec<&str> = s.split(": ").collect();
        let requirements: Vec<&str> = line[0].split(' ').collect();
        let range: Vec<&str> = requirements[0].split('-').collect();

        PasswordTest {
            lower: range[0].parse().expect("Invalid lower bound in input"),
            upper: range[1].parse().expect("Invalid upper bound in input"),
            character: requirements[1].chars().next().expect("Invalid char in input"),
            password: line[1],
        }
    }
}

fn main() {
    // Parse input into PasswordTests
    let file = File::open("./input/1.txt").expect("Failed to open file");
    let lines = read_lines(file).expect("Incorrect input");
    let tests = lines.iter().map(|s| PasswordTest::from_str(s));

    // Check how many PasswordTests pass
    // NOTE: cannot write the filter call as `.filter(PasswordTest::part1)` because of this known issue:
    // https://users.rust-lang.org/t/explanation-for-difference-between-filter-func-and-filter-x-func-x/14945/5
    let answer1 = tests.clone().filter(|test| test.part1()).count();
    let answer2 = tests.filter(|test| test.part2()).count();

    println!("{}", answer1);
    println!("{}", answer2);
}
