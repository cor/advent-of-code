use aoc_2022_common::challenge_input;
use std::collections::HashSet;

fn main() {
    let input = challenge_input();
    let splits: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| (HashSet::from_iter(a.chars()), HashSet::from_iter(b.chars())))
        .map(|(a, b): (HashSet<char>, HashSet<char>)| a.intersection(&b).copied().collect())
        .collect();

    dbg!(splits);
}
