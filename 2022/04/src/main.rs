use aoc_2022_common::challenge_input;

fn main() {
    let input = challenge_input();
    println!("{}", day_1(&input));
}

fn day_1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| l.split_once(','))
        .map(|(a, b)| (Range::from(a), Range::from(b)))
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count()
}

struct Range(u64, u64);

impl From<&str> for Range {
    fn from(input: &str) -> Self {
        let (left, right) = input.split_once('-').unwrap();
        Range(left.parse().unwrap(), right.parse().unwrap())
    }
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
}
