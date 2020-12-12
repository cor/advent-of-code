use aoc_2020_common::common::load_file;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::str::FromStr;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(usize)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotated_by(&self, n: i64) -> Direction {
        let start: usize = self.clone().into();

        Direction::try_from(((start as i64 + n + 4) % 4) as usize).unwrap()
    }
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
    orientation: Direction,
}

impl Position {
    const START: Position = Position {
        x: 0,
        y: 0,
        orientation: Direction::East,
    };

    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Move(direction, n) => self.move_in_direction(&direction, n),
            Instruction::Turn(n) => self.orientation = self.orientation.rotated_by(n),
            Instruction::Forward(n) => self.move_in_direction(&self.orientation.clone(), n),
        }
    }

    fn move_in_direction(&mut self, direction: &Direction, distance: i64) {
        match direction {
            Direction::North => self.y += distance,
            Direction::East => self.x += distance,
            Direction::South => self.y -= distance,
            Direction::West => self.x -= distance,
        }
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

    let mut position = Position::START;

    for instruction in instructions {
        position.apply_instruction(instruction);
    }

    println!("{:?}", position.manhattan_distance());
}
