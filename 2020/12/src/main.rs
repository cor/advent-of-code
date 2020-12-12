use aoc_2020_common::common::load_file;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use std::str::FromStr;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
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
    wx: i64, // Unused for part 1
    wy: i64, // Unused for part 1
}

impl Position {
    const START: Position = Position {
        x: 0,
        y: 0,
        orientation: Direction::East,
        wx: 10,
        wy: 1,
    };

    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    fn oriented_waypoint(&self) -> (i64, i64) {
        match self.orientation {
            Direction::North => (-self.wy, self.wx),
            Direction::East => (self.wx, self.wy),
            Direction::South => (self.wy, -self.wx),
            Direction::West => (-self.wx, -self.wy),
        }
    }

    fn apply_instruction_1(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move(direction, n) => self.move_in_direction(&direction, *n),
            Instruction::Turn(n) => self.orientation = self.orientation.rotated_by(*n),
            Instruction::Forward(n) => self.move_in_direction(&self.orientation.clone(), *n),
        }
    }

    fn apply_instruction_2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move(direction, n) => self.move_waypoint_in_direction(&direction, *n),
            Instruction::Turn(n) =>  self.orientation = self.orientation.rotated_by(*n),
            Instruction::Forward(n) => self.move_to_waypoint(*n),
        }
    }

    fn move_to_waypoint(&mut self, times: i64) {
        let (dx, dy) = self.oriented_waypoint();
        self.x += dx * times;
        self.y += dy * times;
    }

    fn move_in_direction(&mut self, direction: &Direction, distance: i64) {
        match direction {
            Direction::North => self.y += distance,
            Direction::East => self.x += distance,
            Direction::South => self.y -= distance,
            Direction::West => self.x -= distance,
        }
    }

    fn move_waypoint_in_direction(&mut self, direction: &Direction, distance: i64) {
        match direction {
            Direction::North => self.wy += distance,
            Direction::East => self.wx += distance,
            Direction::South => self.wy -= distance,
            Direction::West => self.wx -= distance,
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

    for instruction in &instructions {
        position.apply_instruction_1(instruction);
    }

    println!("{:?}", position.manhattan_distance());

    let mut position_2 = Position::START;

    for instruction in &instructions {
        println!("{:?}", instruction);
        position_2.apply_instruction_2(instruction);
        println!("{:?}", position_2);
    }

    println!("{:?}", position_2.manhattan_distance());
}
