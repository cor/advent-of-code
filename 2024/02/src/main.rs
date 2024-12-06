use aoc_2024_common::challenge_input;
use std::cmp::Ordering::*;

fn main() {
    let input = challenge_input();

    let parsed: Vec<Vec<isize>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect()
        })
        .collect();

    let part_1 = parsed
        .iter()
        .filter_map(|report| match report[0].cmp(&report[1]) {
            Less => report.iter().skip(1).try_fold(report[0], |acc, &n| {
                (1..=3).contains(&(n - acc)).then_some(n)
            }),
            Equal => None,
            Greater => report.iter().skip(1).try_fold(report[0], |acc, &n| {
                (1..=3).contains(&-(n - acc)).then_some(n)
            }),
        })
        .count();
    println!("{part_1}");
}
