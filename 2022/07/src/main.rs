use std::{cmp, collections::HashMap};

use aoc_2022_common::challenge_input;

/// We want to convert [`Command`]s to this structure
#[derive(Debug, Eq, PartialEq)]
enum Node {
    File(u64),
    Directory(HashMap<String, Node>),
}

impl Node {
    pub fn total_size(&self) -> u64 {
        match self {
            Node::File(size) => *size,
            Node::Directory(nodes) => nodes.iter().map(|(_, node)| node.total_size()).sum(),
        }
    }

    pub fn part_1(&self) -> u64 {
        match self {
            Node::File(_) => 0,
            Node::Directory(nodes) => {
                (if self.total_size() <= 100_000 {
                    self.total_size()
                } else {
                    0
                }) + nodes.iter().map(|(_, node)| node.part_1()).sum::<u64>()
            }
        }
    }

    pub fn part_2(&self, minimum_size: u64) -> Option<u64> {
        match self {
            Node::File(_) => None,
            Node::Directory(nodes) => {
                let total_self_size = self.total_size();
                if total_self_size >= minimum_size {
                    let smallest_child = nodes
                        .iter()
                        .filter_map(|(_, node)| node.part_2(minimum_size))
                        .min();

                    return match smallest_child {
                        Some(size) => Some(cmp::min(size, total_self_size)),
                        None => Some(total_self_size),
                    };
                }
                None
            }
        }
    }
}

/// Should be converted to [`Node`]s
#[derive(Debug, Eq, PartialEq)]
enum Command {
    Cd(CdLocation),
    Ls(Vec<LsOutput>),
}

#[derive(Debug, Eq, PartialEq)]
enum LsOutput {
    File(u64, String),
    Directory(String),
}

#[derive(Debug, Eq, PartialEq)]
enum CdLocation {
    Up,
    Directory(String),
}

fn main() {
    let commands = parse_input(&challenge_input());
    let fs = commands_to_fs(commands);

    println!("{}", fs.part_1());

    let minimum_folder_size = 30_000_000 - (70_000_000 - fs.total_size());

    println!("{}", fs.part_2(minimum_folder_size).unwrap());
}

/// TODO: make less ugly
fn parse_input(input: &str) -> Vec<Command> {
    let serialized_commands = input.split("\n$ ").collect::<Vec<_>>();

    let mut parsed_input = Vec::new();

    for serialized_command in &serialized_commands[1..] {
        let lines = serialized_command.lines().collect::<Vec<_>>();

        parsed_input.push(if lines[0] == "ls" {
            Command::Ls(
                lines[1..]
                    .iter()
                    .map(|l| {
                        let split = l.split(' ').collect::<Vec<_>>();
                        if split[0] == "dir" {
                            LsOutput::Directory(split[1].to_owned())
                        } else {
                            LsOutput::File(
                                split[0].parse().expect("invalid file size"),
                                split[1].to_owned(),
                            )
                        }
                    })
                    .collect(),
            )
        } else {
            Command::Cd(
                match lines[0].split(' ').nth(1).expect("invalid cd command") {
                    ".." => CdLocation::Up,
                    dir => CdLocation::Directory(dir.to_owned()),
                },
            )
        });
    }

    parsed_input
}

/// TODO: make less ugly
fn commands_to_fs(commands: Vec<Command>) -> Node {
    let mut fs = Node::Directory(HashMap::new());

    let mut current_path: Vec<String> = Vec::new();
    for command in commands {
        match command {
            Command::Cd(CdLocation::Up) => {
                current_path.pop();
            }
            Command::Cd(CdLocation::Directory(str)) => {
                current_path.push(str);
            }
            Command::Ls(output) => {
                let current_path_clone = current_path.clone();
                let mut current_node = &mut fs;

                for dir in current_path_clone {
                    if let Node::Directory(map) = current_node {
                        current_node = map.get_mut(&dir.clone()).unwrap();
                    }
                }

                let Node::Directory(map) = current_node else { panic!(); };

                for node in output {
                    match node {
                        LsOutput::File(size, name) => {
                            map.insert(name, Node::File(size));
                        }
                        LsOutput::Directory(name) => {
                            map.insert(name, Node::Directory(HashMap::new()));
                        }
                    }
                }
            }
        }
    }

    fs
}
