use aoc_2022_common::challenge_input;
use regex::Regex;

fn main() {
    let crate_mover = CrateMover::from(challenge_input().as_str());
    println!("{}", crate_mover.part_1());
    println!("{}", crate_mover.part_2());
}

#[derive(Debug)]
struct CrateMover {
    pub crates: Vec<Vec<char>>,
    pub moves: Vec<Move>,
}

impl CrateMover {
    pub fn simulate_9000(&self) -> Vec<Vec<char>> {
        let mut crates = self.crates.clone();

        for &Move { count, from, to } in &self.moves {
            for _ in 0..count {
                let to_move = crates[from].pop().unwrap();
                crates[to].push(to_move);
            }
        }

        crates
    }

    pub fn simulate_9001(&self) -> Vec<Vec<char>> {
        let mut crates = self.crates.clone();

        for &Move { count, from, to } in &self.moves {
            let drain_from = crates[from].len() - count;
            let mut drained_crates: Vec<char> = crates[from].drain(drain_from..).collect();
            crates[to].append(&mut drained_crates);
        }

        crates
    }

    pub fn top_crates(mut crates: Vec<Vec<char>>) -> String {
        crates.iter_mut().filter_map(|l| l.pop()).collect()
    }

    pub fn part_1(&self) -> String {
        Self::top_crates(self.simulate_9000())
    }

    pub fn part_2(&self) -> String {
        Self::top_crates(self.simulate_9001())
    }
}

impl From<&str> for CrateMover {
    fn from(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();
        let mut split_iter = lines.split(|l| l.is_empty());
        let crates = split_iter.next().expect("No crates").to_vec();
        let moves = split_iter.next().expect("No moves").to_vec();

        let crates: Vec<Vec<char>> = transpose(
            crates[..crates.len() - 1]
                .iter()
                .map(|l| {
                    l.chars()
                        .collect::<Vec<_>>()
                        .chunks(4)
                        .map(|c| c[1])
                        .collect()
                })
                .collect(),
        )
        .iter()
        .map(|crat| crat.iter().rev().filter(|&c| *c != ' ').copied().collect())
        .collect();

        let moves = moves.iter().map(|&i| Move::from(i)).collect();

        CrateMover { crates, moves }
    }
}

#[derive(Debug)]
struct Move {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let capture = re.captures_iter(input).next().expect("Invalid move");
        Move {
            count: capture[1].parse().expect("Invalid move"),
            from: capture[2].parse::<usize>().expect("Invalid move") - 1,
            to: capture[3].parse::<usize>().expect("Invalid move") - 1,
        }
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
