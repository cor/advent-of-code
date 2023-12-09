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
        let instructions = instructions
            .chars()
            .map(Instruction::parse)
            .collect::<Vec<_>>();
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
}

impl Instruction {
    fn parse(input: char) -> Self {
        match input {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid input"),
        }
    }
}

fn main() {
    let input = challenge_input();
    let map = Map::parse(&input);
    dbg!(map);
    // println!("{input}");
}
