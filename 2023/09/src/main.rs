use aoc_2023_common::challenge_input;

fn deltas(sequence: &[i64]) -> Vec<i64> {
    sequence
        .windows(2)
        .filter_map(|window| match window {
            [left, right] => Some(right - left),
            _ => None,
        })
        .collect()
}

fn recursive_deltas(sequence: &[i64]) -> Vec<Vec<i64>> {
    let mut sequences = vec![sequence.to_vec().clone()];
    while !sequences.last().unwrap().iter().all(|&n| n == 0) {
        let next = deltas(sequences.last().unwrap());
        sequences.push(next);
    }
    sequences.pop(); // we don't care about [0, 0, ..]
    sequences
}

fn extrapolate(sequence: &[i64]) -> i64 {
    let mut deltas = recursive_deltas(sequence);

    let len = deltas.len();
    let smallest_delta = deltas[len - 1][0];
    deltas[len - 1].push(smallest_delta);

    for i in (0..len - 1).rev() {
        let next = deltas[i].last().unwrap() + deltas[i + 1].last().unwrap();
        deltas[i].push(next);
    }

    *deltas[0].last().unwrap()
}

fn main() {
    let input = challenge_input();
    let mut sequences: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect()
        })
        .collect();

    let part_1: i64 = sequences.iter().map(|seq| extrapolate(seq)).sum();
    println!("{}", part_1);

    sequences.iter_mut().for_each(|seq| seq.reverse());
    let part_2: i64 = sequences.iter().map(|seq| extrapolate(seq)).sum();

    println!("{}", part_2);
}
