use aoc_2024_common::challenge_input;
use std::cmp::Ordering::*;

fn is_safe(report: &&Vec<isize>) -> bool {
    match report[0].cmp(&report[1]) {
        Less => report.iter().skip(1).try_fold(report[0], |acc, &n| {
            (1..=3).contains(&(n - acc)).then_some(n)
        }),
        Equal => None,
        Greater => report.iter().skip(1).try_fold(report[0], |acc, &n| {
            (1..=3).contains(&-(n - acc)).then_some(n)
        }),
    }
    .is_some()
}
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

    let part_1 = parsed.iter().filter(is_safe).count();

    let part_2 = parsed
        .iter()
        .filter(|&report| {
            (0..report.len()).any(|to_skip| {
                let mut report = report.clone();
                report.remove(to_skip);
                is_safe(&&report)
            })
        })
        .count();

    println!("{part_1}");
    println!("{part_2}");
}
