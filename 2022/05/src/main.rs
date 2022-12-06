use aoc_2022_common::challenge_input;
use regex::Regex;

fn main() {
    let cargo_simulator = CargoSimulator::from(challenge_input().as_str());
    println!("{}", cargo_simulator.part_1());
}

#[derive(Debug)]
struct CargoSimulator {
    pub containers: Vec<Vec<char>>,
    pub instructions: Vec<Instruction>,
}

impl CargoSimulator {
    pub fn simulate(&self) -> Vec<Vec<char>> {
        let mut containers = self.containers.clone();

        for ins in &self.instructions {
            for _ in 0..ins.count {
                let to_move = containers[ins.from].pop().unwrap();
                containers[ins.to].push(to_move);
            }
        }

        containers
    }

    pub fn part_1(&self) -> String {
        self.simulate()
            .iter_mut()
            .filter_map(|l| l.pop())
            .collect::<String>()
    }
}

impl From<&str> for CargoSimulator {
    fn from(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();
        let mut split_iter = lines.split(|l| l.is_empty());
        let containers = split_iter.next().expect("Missing containers").to_vec();
        let instructions = split_iter.next().expect("Missing instructions").to_vec();

        let containers: Vec<Vec<char>> = transpose(
            containers[..containers.len() - 1]
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
        .map(|container| {
            container
                .iter()
                .rev()
                .filter(|&c| *c != ' ')
                .copied()
                .collect()
        })
        .collect();

        let instructions = instructions.iter().map(|&i| Instruction::from(i)).collect();

        CargoSimulator {
            containers,
            instructions,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let capture = re.captures_iter(input).next().expect("Invalid instruction");
        Instruction {
            count: capture[1].parse().unwrap(),
            from: capture[2].parse::<usize>().unwrap() - 1,
            to: capture[3].parse::<usize>().unwrap() - 1,
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
