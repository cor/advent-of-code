use aoc_2022_common::challenge_input;
use std::collections::HashSet;

fn main() {
    let input = challenge_input();
    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| (HashSet::from_iter(a.chars()), HashSet::from_iter(b.chars())))
        .filter_map(|(a, b): (HashSet<char>, HashSet<char>)| a.intersection(&b).next().copied())
        .map(|c| ((c as u32 - 'A' as u32) + 27) % 58)
        .sum()
}
