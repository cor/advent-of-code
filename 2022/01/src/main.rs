use aoc_2022_common::challenge_input;

fn main() {
    let input = challenge_input();
    let lines: Vec<&str> = input.lines().collect();
    let groups: Vec<u64> = lines
        .split(|l| l.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|s| s.parse::<u64>().expect("input contains non-number line"))
                .sum()
        })
        .collect();
    println!("{}", groups.iter().max().expect("no max group"));
}
