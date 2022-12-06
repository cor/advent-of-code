use std::collections::HashSet;

use aoc_2022_common::challenge_input;

fn main() {
    let signal = challenge_input();
    println!("{}", first_unique_sequence(&signal, 4).unwrap());
    println!("{}", first_unique_sequence(&signal, 14).unwrap());
}

fn first_unique_sequence(signal: &str, len: usize) -> Option<usize> {
    signal
        .chars()
        .collect::<Vec<_>>()
        .windows(len)
        .enumerate()
        .find(|(_, window)| HashSet::<&char>::from_iter(window.iter()).len() == len)
        .map(|(i, _)| i + len)
}
