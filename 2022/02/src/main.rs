use aoc_2022_common::challenge_input;

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Victory,
    Draw,
    Defeat,
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
        match self {
            Turn(Move::Rock, Move::Paper) => Outcome::Victory,
            Turn(Move::Rock, Move::Scissors) => Outcome::Defeat,
            Turn(Move::Paper, Move::Rock) => Outcome::Defeat,
            Turn(Move::Paper, Move::Scissors) => Outcome::Victory,
            Turn(Move::Scissors, Move::Paper) => Outcome::Defeat,
            Turn(Move::Scissors, Move::Rock) => Outcome::Victory,
            _ => Outcome::Draw,
        }
    }
    pub fn score(&self) -> u64 {
        (match self {
            Turn(_, Move::Rock) => 1,
            Turn(_, Move::Paper) => 2,
            Turn(_, Move::Scissors) => 3,
        }) + match self.outcome() {
            Outcome::Victory => 6,
            Outcome::Draw => 3,
            Outcome::Defeat => 0,
        }
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
        let left = Move::from(&chars.next().expect("unexpected character in input"));
        chars.next();
        let right = Move::from(&chars.next().expect("Unexpected character in input"));
        Turn(left, right)
    }
}

impl From<&char> for Move {
    fn from(value: &char) -> Self {
        match value {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            c => panic!("Cannot convert {c} to Move"),
        }
    }
}
