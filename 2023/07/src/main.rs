use aoc_2023_common::challenge_input;

mod part1;
mod part2;

fn main() {
    let input = challenge_input();
    let mut hands = input.lines().map(part1::Hand::parse).collect::<Vec<_>>();
    let mut hands_2 = input.lines().map(part2::Hand::parse).collect::<Vec<_>>();
    hands.sort_unstable();
    hands_2.sort_unstable();

    let part_1: u64 = hands
        .iter()
        .zip(1..)
        .map(|(hand, score)| score * hand.bid)
        .sum();

    let part_2: u64 = hands_2
        .iter()
        .zip(1..)
        .map(|(hand, score)| score * hand.bid)
        .sum();

    println!("{part_1}");
    println!("{part_2}");
}
