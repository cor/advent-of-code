use std::collections::HashSet;

use aoc_2022_common::challenge_input;

fn main() {
    let signal = challenge_input();
    println!("{}", first_unique_sequence(&signal, 4).unwrap());
    println!("{}", first_unique_sequence(&signal, 14).unwrap());
}

fn first_unique_sequence(signal: &str, len: usize) -> Option<usize> {
    signal
        .as_bytes()
        .windows(len)
        .position(|win| win.iter().collect::<HashSet<_>>().len() == len)
        .map(|pos| pos + len)
}
