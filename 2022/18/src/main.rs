use aoc_2022_common::challenge_input;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    /// Panics if input is invalid
    pub fn parse(input: &str) -> Self {
        let (x, yz) = input.split_once(',').unwrap();
        let (y, z) = yz.split_once(',').unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        }
    }

    pub fn parse_list0(input: &str) -> Vec<Self> {
        input.lines().map(Self::parse).collect()
    }
}

fn main() {
    let input = challenge_input();
    let points = Point::parse_list0(&input);
    dbg!(points);
}
