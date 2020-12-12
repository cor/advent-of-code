use aoc_2020_common::common::load_file;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
    orientation: Direction,
}

impl Position {
    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.x.abs()
    }
}
#[derive(Debug)]
enum Instruction {
    Move(Direction, i64),
    Turn(i64),
    Forward(i64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (operation, number) = s.split_at(1);

        if let Ok(number) = number.parse() {
            match operation {
                "N" => Ok(Instruction::Move(Direction::North, number)),
                "E" => Ok(Instruction::Move(Direction::East, number)),
                "S" => Ok(Instruction::Move(Direction::South, number)),
                "W" => Ok(Instruction::Move(Direction::West, number)),
                "L" => Ok(Instruction::Turn(- number/90)),
                "R" => Ok(Instruction::Turn(number/90)),
                "F" => Ok(Instruction::Forward(number)),
                _ => Err(String::from("Invalid operation in instruction"))
            }
        } else {
            Err(String::from("Invalid number in instruction"))
        }
    }
}

fn main() {
    let input = load_file("./input/1.txt");
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|s| Instruction::from_str(s))
        .filter_map(Result::ok)
        .collect();

    println!("{:?}", instructions);
}
