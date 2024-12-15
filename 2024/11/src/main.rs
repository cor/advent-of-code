use aoc_2024_common::challenge_input;
use memoize::memoize;

fn main() {
    let input = challenge_input();
    let parsed: Vec<i64> = input.split(' ').map(|n| n.parse().unwrap()).collect();

    let part_1 = parsed.iter().map(|s| count_after_blinks(*s, 25)).sum::<i64>();
    println!("{part_1}");
    let part_2 = parsed.iter().map(|s| count_after_blinks(*s, 75)).sum::<i64>();
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

#[memoize]
fn blink(stone: i64) -> Vec<i64> {
    let stone_string = stone.to_string();
    let stone_len = stone_string.len();
    match stone {
        0 => vec![1],
        _ if stone_len % 2 == 0 => {
            let (left, right) = stone_string.split_at(stone_len/2);
            vec![left.parse().unwrap(), right.parse().unwrap()]
        },
        n => vec![n * 2024]
    }
}
