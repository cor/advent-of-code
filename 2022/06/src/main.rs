use std::collections::HashSet;

use aoc_2022_common::challenge_input;

fn main() {
    let signal = challenge_input();
    println!("{}", first_unique_sequence(&signal, 4).unwrap());
    println!("{}", first_unique_sequence(&signal, 14).unwrap());
}

fn first_unique_sequence(signal: &str, len: usize) -> Option<usize> {
    let chars: Vec<char> = signal.chars().collect();
    (len..chars.len()).find(|&i| HashSet::<&char>::from_iter(&chars[i - len..i]).len() == len)
}
