use aoc_2024_common::challenge_input;
use memoize::memoize;

fn main() {
    let input = challenge_input();
    let parsed: Vec<i64> = input.split(' ').map(|n| n.parse().unwrap()).collect();

    let part_1 = parsed
        .iter()
        .map(|s| count_after_blinks(*s, 25))
        .sum::<i64>();
    println!("{part_1}");
    let part_2 = parsed
        .iter()
        .map(|s| count_after_blinks(*s, 75))
        .sum::<i64>();
    println!("{part_2}");
}

#[memoize]
fn count_after_blinks(stone: i64, blinks: i64) -> i64 {
    match blinks {
        0 => 1,
        n => {
            let stones = blink(stone);
            stones.iter().map(|s| count_after_blinks(*s, n - 1)).sum()
        }
    }
}

fn blink(stone: i64) -> Vec<i64> {
    if stone == 0 {
        return vec![1];
    }
    let width = stone.ilog10() + 1;
    if width % 2 == 0 {
        return vec![stone / 10_i64.pow(width / 2), stone % 10_i64.pow(width / 2)];
    }
    return vec![stone * 2024];
}
