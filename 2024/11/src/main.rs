use aoc_2024_common::challenge_input;
use memoize::memoize;

fn main() {
    let input = challenge_input();
    let parsed: Vec<u64> = input.split(' ').map(|n| n.parse().unwrap()).collect();

    let part_1 = parsed
        .iter()
        .map(|s| count_after_blinks(*s, 25))
        .sum::<u64>();
    println!("{part_1}");
    let part_2 = parsed
        .iter()
        .map(|s| count_after_blinks(*s, 75))
        .sum::<u64>();
    println!("{part_2}");
}

#[memoize(Capacity: 75_000)]
fn count_after_blinks(stone: u64, blinks: u64) -> u64 {
    if blinks == 0 {
        return 1;
    }
    let (left, right) = blink(stone);
    count_after_blinks(left, blinks - 1) + right.map_or(0, |s| count_after_blinks(s, blinks-1))
}

#[inline(always)]
fn blink(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        return (1, None);
    }
    let width = stone.ilog10() + 1;
    if width % 2 == 0 {
        return (stone / 10_u64.pow(width / 2), Some(stone % 10_u64.pow(width / 2)));
    }
    return (stone * 2024, None);
}
