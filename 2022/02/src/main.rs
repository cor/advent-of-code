use aoc_2022_common::challenge_input;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum Outcome {
    Victory = 6,
    Draw = 3,
    Defeat = 0,
}

#[derive(Debug)]
struct Turn(Move, Move);

#[derive(Debug)]
struct Turn2(Move, Outcome);

#[derive(Debug)]
struct Game(Vec<Turn>);

#[derive(Debug)]
struct Game2(Vec<Turn2>);

fn main() {
    let input = challenge_input();
    let game = Game::from(input.as_ref());
    let game2 = Game2::from(input.as_ref());

    println!("{}", game.0.iter().map(Turn::score).sum::<u64>());
    println!("{}", game2.0.iter().map(Turn2::score).sum::<u64>());
}

impl Turn {
    pub fn outcome(&self) -> Outcome {
        use Move::{Paper, Rock, Scissors};
        use Outcome::{Defeat, Draw, Victory};

        match self {
            Turn(a, b) if a == b => Draw,
            Turn(Rock, Paper) | Turn(Paper, Scissors) | Turn(Scissors, Rock) => Victory,
            _ => Defeat,
        }
    }
    pub fn score(&self) -> u64 {
        self.1 as u64 + self.outcome() as u64
    }
}

impl Turn2 {
    pub fn score(&self) -> u64 {
        use Move::{Paper, Rock, Scissors};
        use Outcome::{Defeat, Draw, Victory};

        match self {
            Turn2(m, Draw) => Draw as u64 + (*m as u64),
            Turn2(m, Victory) => {
                Victory as u64
                    + (match m {
                        Rock => Paper,
                        Paper => Scissors,
                        Scissors => Rock,
                    } as u64)
            }
            Turn2(m, Defeat) => {
                Defeat as u64
                    + (match m {
                        Rock => Scissors,
                        Paper => Rock,
                        Scissors => Paper,
                    } as u64)
            }
        }
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        Game(value.lines().into_iter().map(Turn::from).collect())
    }
}

impl From<&str> for Game2 {
    fn from(value: &str) -> Self {
        Game2(value.lines().into_iter().map(Turn2::from).collect())
    }
}

impl From<&str> for Turn {
    fn from(value: &str) -> Self {
        let (move1, move2) = value.split_once(' ').unwrap();
        Turn(move1.into(), move2.into())
    }
}

impl From<&str> for Turn2 {
    fn from(value: &str) -> Self {
        let (mov, outcome) = value.split_once(' ').unwrap();
        Turn2(mov.into(), outcome.into())
    }
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            c => panic!("Cannot convert {c} to Move"),
        }
    }
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Self::Defeat,
            "Y" => Self::Draw,
            "Z" => Self::Victory,
            c => panic!("Cannot convert {c} to Move"),
        }
    }
}
