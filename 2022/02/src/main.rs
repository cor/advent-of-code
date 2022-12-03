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
    let game = Game::from(challenge_input().as_ref());
    println!("{}", part_1(&game));

    let game2 = Game2::from(challenge_input().as_ref());
    println!("{}", part_2(&game2));
}

fn part_1(game: &Game) -> u64 {
    game.0.iter().map(|t| t.score()).sum()
}

fn part_2(game: &Game2) -> u64 {
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
        let mut chars = value.chars();
        let left = Move::from(&chars.next().unwrap());
        chars.next();
        let right = Move::from(&chars.next().unwrap());
        Turn(left, right)
    }
}

impl From<&str> for Turn2 {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        let left = Move::from(&chars.next().unwrap());
        chars.next();
        let right = Outcome::from(&chars.next().unwrap());
        Turn2(left, right)
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

impl From<&char> for Outcome {
    fn from(value: &char) -> Self {
        match value {
            'X' => Self::Defeat,
            'Y' => Self::Draw,
            'Z' => Self::Victory,
            c => panic!("Cannot convert {c} to Move"),
        }
    }
}
