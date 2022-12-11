use aoc_2022_common::challenge_input;

fn main() {
    let calories = parse_input(&challenge_input());

    println!("{}", part_1(&calories));
    println!("{}", part_2(calories));
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .split(|l| l.is_empty())
        .map(|g| g.iter().flat_map(|s| s.parse::<u64>()).sum())
        .collect()
}

fn part_1(calories: &[u64]) -> &u64 {
    calories.iter().max().expect("no groups in input")
}

fn part_2(mut calories: Vec<u64>) -> u64 {
    calories.sort_unstable();
    calories.reverse();
    calories.truncate(3);
    calories.iter().sum()
}
