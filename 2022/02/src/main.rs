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
struct Game(Vec<Turn>);

fn main() {
    let game = Game::from(challenge_input().as_ref());

    println!("{}", part_1(&game));
}

fn part_1(game: &Game) -> u64 {
    game.0.iter().map(|t| t.score()).sum()
}

impl Turn {
    pub fn outcome(&self) -> Outcome {
        use Move::{Paper, Rock, Scissors};
        use Outcome::{Defeat, Draw, Victory};
        match self {
            Turn(a, b) if a == b => Draw,
            Turn(Rock, Paper) => Victory,
            Turn(Paper, Scissors) => Victory,
            Turn(Scissors, Rock) => Victory,
            _ => Defeat,
        }
    }
    pub fn score(&self) -> u64 {
        self.1 as u64 + self.outcome() as u64
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        Game(value.lines().into_iter().map(Turn::from).collect())
    }
}

impl From<&str> for Turn {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        let left = Move::from(&chars.next().unwrap());
        chars.next();
        let right = Move::from(&chars.next().unwrap());
        Turn(left, right)
    }
}

impl From<&char> for Move {
    fn from(value: &char) -> Self {
        match value {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            c => panic!("Cannot convert {c} to Move"),
        }
    }
}
