use aoc_2022_common::challenge_input;
use std::collections::HashSet;

fn main() {
    let input = challenge_input();
    let splits = input
        .lines()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(a, b)| (HashSet::from_iter(a.chars()), HashSet::from_iter(b.chars())))
        .collect::<Vec<(HashSet<char>, HashSet<char>)>>();
    dbg!(splits);
}
