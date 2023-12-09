use std::collections::HashMap;

use aoc_2023_common::challenge_input;

type Node = [char; 3];
const TARGET_NODE: Node = ['Z', 'Z', 'Z'];

trait Parse {
    fn parse(input: &str) -> Self;
}

impl Parse for Node {
    fn parse(input: &str) -> Self {
        input
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .expect("invalid input")
    }
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Instruction>,
    network: HashMap<Node, (Node, Node)>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let (instructions, network) = input.split_once("\n\n").expect("Invalid input");
        let instructions = instructions.chars().map(Instruction::parse).collect();
        let network = network
            .lines()
            .map(|line| {
                let (from, to) = line.split_once(" = ").expect("invalid input");
                let from = Node::parse(from);
                let (to_left, to_right) = to.split_once(", ").expect("invalid input");
                let to_left = Node::parse(&to_left[1..]);
                let to_right = Node::parse(&to_right[..3]);

                (from, (to_left, to_right))
            })
            .collect();

        Self {
            instructions,
            network,
        }
    }

    fn step_count(&self) -> usize {
        let mut node: Node = ['A', 'A', 'A'];

        for step_count in 0.. {
            let instruction = &self.instructions[step_count % self.instructions.len()];
            let (left, right) = self.network.get(&node).expect("invalid node in network");
            match instruction {
                Instruction::Left => node = *left,
                Instruction::Right => node = *right,
            }
            if node == TARGET_NODE {
                return step_count + 1;
            }
        }

        panic!("QED");
    }

    fn step_count_2(&self) -> usize {
        let mut start_nodes: Vec<&Node> = self
            .network
            .keys()
            .filter(|key| matches!(key, [_, _, 'A']))
            .collect();

        for step_count in 0.. {
            let instruction = &self.instructions[step_count % self.instructions.len()];

            start_nodes = start_nodes
                .iter()
                .map(|&&node| {
                    let (left, right) = self.network.get(&node).expect("invalid node in network");
                    match instruction {
                        Instruction::Left => left,
                        Instruction::Right => right,
                    }
                })
                .collect();
            if start_nodes.iter().all(|node| matches!(node, [_, _, 'Z'])) {
                return step_count + 1;
            }
        }

        panic!("QED");
    }
}

impl Instruction {
    fn parse(input: char) -> Self {
        match input {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("invalid input"),
        }
    }
}

fn main() {
    let input = challenge_input();
    let map = Map::parse(&input);
    println!("{}", map.step_count());
    println!("{}", map.step_count_2());
}
