use aoc_2022_common::challenge_input;

fn main() {
    let calories = parse_input(&challenge_input());

    println!("{}", part_1(&calories).expect("no groups in input"));
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .split(|l| l.is_empty())
        .map(|g| g.iter().map(|s| s.parse::<u64>().unwrap()).collect())
        .collect()
}

fn part_1(calories: &[Vec<u64>]) -> Option<u64> {
    calories.iter().map(|group| group.iter().sum()).max()
}
