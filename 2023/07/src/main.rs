use aoc_2023_common::challenge_input;
use itertools::Itertools;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Rank {
    Number(u8),
    T,
    J,
    Q,
    K,
    A,
}
use Rank::*;

impl Rank {
    fn parse(c: char) -> Self {
        match c {
            'A' => A,
            'K' => K,
            'Q' => Q,
            'J' => J,
            'T' => T,
            n if ('2'..='9').contains(&n) => Number(n.to_digit(10).unwrap() as u8),
            _ => panic!("Invalid rank in puzzle"),
        }
    }

    fn tweak_j_value(self) -> Self {
        match self {
            J => Number(1),
            r => r,
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
use HandType::*;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum Game {
    WithoutJokers,
    WithJokers,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    pub cards: [Rank; 5],
    pub bid: u64,
    pub game: Game,
}

impl Hand {
    fn typ(&self) -> HandType {
        let mut groups: Vec<(Rank, usize)> = self
            .cards
            .iter()
            .zip(1..)
            .into_group_map()
            .iter()
            .map(|(k, v)| (**k, v.len()))
            .collect::<Vec<_>>();
        groups.sort_unstable_by(|g0, g1| g1.1.cmp(&g0.1));

        if self.game == Game::WithJokers {
            if groups[0] == (J, 5) {
                return FiveOfAKind;
            }

            for (index, group) in groups.clone().iter().enumerate() {
                if let (J, count) = group {
                    groups.remove(index);
                    groups[0].1 += count;
                    break;
                }
            }
        }

        match groups[..] {
            [(_, 5)] => FiveOfAKind,
            [(_, 4), _] => FourOfAKind,
            [(_, 3), (_, 2)] => FullHouse,
            [(_, 3), _, _] => ThreeOfAKind,
            [(_, 2), (_, 2), _] => TwoPair,
            [(_, 2), _, _, _] => OnePair,
            _ => HighCard,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match self.typ().cmp(&other.typ()) {
            Less => Less,
            Greater => Greater,
            Equal => match self.game {
                Game::WithoutJokers => self.cards.cmp(&other.cards),
                Game::WithJokers => {
                    let self_cards = self.cards.map(Rank::tweak_j_value);
                    let other_cards = other.cards.map(Rank::tweak_j_value);

                    self_cards.cmp(&other_cards)
                }
            },
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    pub fn parse(s: &str, game: Game) -> Self {
        let (cards, bid) = s.split_once(' ').expect("invalid input");

        let cards: [Rank; 5] = cards
            .chars()
            .map(Rank::parse)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let bid = bid.parse().unwrap();
        Self { cards, bid, game }
    }
}

fn total_winnings(hands: &[Hand]) -> u64 {
    hands
        .iter()
        .zip(1..)
        .map(|(hand, score)| score * hand.bid)
        .sum()
}

fn main() {
    let input = challenge_input();
    let mut hands_1 = input
        .lines()
        .map(|line| Hand::parse(line, Game::WithoutJokers))
        .collect::<Vec<_>>();
    let mut hands_2 = input
        .lines()
        .map(|line| Hand::parse(line, Game::WithJokers))
        .collect::<Vec<_>>();
    hands_1.sort_unstable();
    hands_2.sort_unstable();

    println!("{}", total_winnings(&hands_1));
    println!("{}", total_winnings(&hands_2));
}
