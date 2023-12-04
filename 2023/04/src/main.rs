use aoc_2023_common::challenge_input;

use nom::{
    bytes::complete::tag,
    character::{
        complete::{space1, u32},
        streaming::newline,
    },
    combinator::map,
    multi::separated_list0,
    sequence::separated_pair,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
}

impl Card {
    pub fn parse_many(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list0(newline, Self::parse)(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                preceded(preceded(tag("Card"), space1), u32),
                preceded(tag(":"), space1),
                separated_pair(
                    separated_list0(space1, u32),
                    delimited(space1, tag("|"), space1),
                    separated_list0(space1, u32),
                ),
            ),
            |(id, (winning_numbers, numbers_you_have))| Self {
                id,
                winning_numbers,
                numbers_you_have,
            },
        )(input)
    }

    pub fn win_count(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|&winning_number| self.numbers_you_have.contains(winning_number))
            .count() as u32
    }

    pub fn points(&self) -> u32 {
        match self.win_count() {
            0 => 0,
            n => 2u32.pow(n - 1),
        }
    }

    pub fn points_2(&self, cards: &[Self]) -> u32 {
        (self.id..=(self.id + self.win_count() - 1))
            .map(|new_id| match cards.get(new_id as usize) {
                None => 0,
                Some(card) => card.points_2(cards),
            })
            .sum::<u32>()
            + 1
    }
}

fn main() {
    let input = challenge_input();
    let cards = Card::parse_many(&input).expect("Invalid input").1;
    let points = cards.iter().map(|card| card.points()).sum::<u32>();
    let points_2 = cards.iter().map(|card| card.points_2(&cards)).sum::<u32>();
    println!("{points}");
    println!("{points_2}");
}
