use aoc_2022_common::challenge_input;

fn main() {
    let callories: Vec<Vec<u64>> = parse_input(&challenge_input());

    println!("{}", part_1(&callories).expect("no groups in input"));
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .split(|l| l.is_empty())
        .map(|g| {
            g.iter()
                .map(|s| s.parse::<u64>().expect("non-number line"))
                .collect()
        })
        .collect()
}

fn part_1(callories: &[Vec<u64>]) -> Option<u64> {
    callories.iter().map(|group| group.iter().sum()).max()
}
