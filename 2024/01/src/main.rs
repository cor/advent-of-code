use aoc_2024_common::challenge_input;

fn main() {
    let input = challenge_input();
    let (mut left, mut right): (Vec<isize>, Vec<isize>) = input
        .lines()
        .map(|l| {
            let (l, r) = l.split_once("   ").unwrap();
            (l.parse::<isize>().unwrap(), r.parse::<isize>().unwrap())
        })
        .unzip();
    left.sort();
    right.sort();

    let part_1: usize = left
        .iter()
        .zip(right.iter())
        .map(|(&x, &y)| x.abs_diff(y))
        .sum();

    let part_2: isize = left
        .iter()
        .map(|i| i * right.iter().filter(|j| &i == j).count() as isize)
        .sum();

    println!("{part_1}");
    println!("{part_2}");
}
