use aoc_2023_common::challenge_input;

fn first_digit(line: &str) -> Option<char> {
    line.chars().find(|c| c.is_numeric())
}

fn last_digit(line: &str) -> Option<char> {
    line.chars().rev().find(|c| c.is_numeric())
}

fn calibration_value(line: &str) -> Option<usize> {
    let first = first_digit(line)?;
    let last = last_digit(line)?;
    [first, last].iter().collect::<String>().parse().ok()
}

fn main() {
    let input = challenge_input();
    let part_1: usize = input.lines().filter_map(calibration_value).sum();
    println!("{part_1}");
}
