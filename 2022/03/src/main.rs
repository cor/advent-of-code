use aoc_2022_common::challenge_input;
use std::collections::HashSet;

type CharSet = HashSet<char>;

fn main() {
    let input = challenge_input();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| (to_charset(a), to_charset(b)))
        .filter_map(|(a, b)| a.intersection(&b).next().copied())
        .map(to_priority)
        .sum()
}

fn part_2(input: &str) -> u64 {
    input
        .lines()
        .map(to_charset)
        .collect::<Vec<_>>()
        .chunks(3)
        .filter_map(|chunk| match chunk {
            [a, b, c] => a
                .intersection(b)
                .copied()
                .collect::<CharSet>()
                .intersection(c)
                .next()
                .copied(),
            _ => panic!("Invalid chunks in input"),
        })
        .map(to_priority)
        .sum()
}

fn to_priority(c: char) -> u64 {
    ((c as u64 - 'A' as u64) + 27) % 58
}

fn to_charset(s: &str) -> CharSet {
    s.chars().collect()
}
