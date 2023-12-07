use aoc_2023_common::challenge_input;
mod part1;
fn main() {
    let input = challenge_input();
    let mut hands = input.lines().map(part1::Hand::parse).collect::<Vec<_>>();
    hands.sort_unstable();

    let part_1: u64 = hands
        .iter()
        .zip(1..)
        .map(|(hand, score)| score * hand.bid)
        .sum();

    println!("{part_1}");
}
