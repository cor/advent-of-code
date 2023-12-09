use num::integer::lcm;
use rayon::prelude::*;
use std::collections::HashMap;

use aoc_2023_common::challenge_input;

type Node = [char; 3];

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

    fn step_count(&self, start_node: Node) -> usize {
        let mut node: Node = start_node;

        for step_count in 0.. {
            let instruction = &self.instructions[step_count % self.instructions.len()];
            let (left, right) = self.network.get(&node).expect("invalid node in network");
            match instruction {
                Instruction::Left => node = *left,
                Instruction::Right => node = *right,
            }
            if matches!(node, [_, _, 'Z']) {
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
    println!("{}", map.step_count(['A', 'A', 'A']));

    let start_nodes: Vec<&Node> = map
        .network
        .keys()
        .filter(|key| matches!(key, [_, _, 'A']))
        .collect();

    let step_count_2: Vec<usize> = start_nodes
        .par_iter()
        .map(|&&node| map.step_count(node))
        .collect();

    let part_2 = step_count_2.iter().cloned().reduce(lcm).unwrap();
    println!("{}", part_2);
}
