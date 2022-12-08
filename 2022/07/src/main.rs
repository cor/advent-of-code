use std::collections::HashMap;

use aoc_2022_common::challenge_input;

#[derive(Debug, Eq, PartialEq)]
enum Node {
    File(u64),
    Directory(HashMap<String, Node>),
}

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
    let mut fs = Node::Directory(HashMap::new());
    let commands = parse_input(challenge_input());

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
                        current_node = map.get_mut(&dir.to_owned()).unwrap();
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

    dbg!(fs);
}

/// TODO: make less ugly
fn parse_input(input: String) -> Vec<Command> {
    let split = input.split("\n$ ").collect::<Vec<_>>();

    let mut parsed_input = Vec::new();

    for item in &split[1..] {
        let lines = item.lines().collect::<Vec<_>>();

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
