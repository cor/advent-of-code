use aoc_2022_common::challenge_input;

fn main() {
    let ranges = parse_input(&challenge_input());

    println!("{}", day_1(&ranges));
    println!("{}", day_2(&ranges));
}

fn parse_input(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .filter_map(|l| l.split_once(','))
        .map(|(a, b)| (Range::from(a), Range::from(b)))
        .collect()
}

fn day_1(ranges: &[(Range, Range)]) -> usize {
    ranges
        .iter()
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count()
}

fn day_2(ranges: &[(Range, Range)]) -> usize {
    ranges
        .iter()
        .filter(|(a, b)| a.overlaps(b) || b.overlaps(a))
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

    fn overlaps(&self, other: &Range) -> bool {
        (self.0 >= other.0 && self.0 <= other.1) || (self.1 >= other.0 && self.1 <= other.1)
    }
}
