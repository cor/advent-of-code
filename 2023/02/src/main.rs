use aoc_2023_common::challenge_input;
use nom::{
    bytes::complete::tag,
    character::{
        complete::{alphanumeric1, space1, u32},
        streaming::newline,
    },
    combinator::map,
    multi::separated_list0,
    sequence::preceded,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<(u32, u32, u32)>,
}

impl Game {
    pub fn parse_many(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list0(newline, Self::parse)(input)
    }
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                preceded(tag("Game "), u32),
                tag(": "),
                separated_list0(
                    tag("; "),
                    separated_list0(tag(", "), separated_pair(u32, space1, alphanumeric1)),
                ),
            ),
            |(id, sets)| {
                let sets = sets
                    .iter()
                    .map(|set| {
                        set.iter()
                            .fold((0, 0, 0), |(r, g, b), (count, color)| match *color {
                                "red" => (r + count, g, b),
                                "green" => (r, g + count, b),
                                "blue" => (r, g, b + count),
                                _ => (r, g, b),
                            })
                    })
                    .collect();
                Game { id, sets }
            },
        )(input)
    }
}

fn main() {
    let input = challenge_input();
    let games = Game::parse_many(&input).expect("invalid input").1;
    let part_1: u32 = games
        .iter()
        .filter(|game| {
            game.sets
                .iter()
                .all(|(r, g, b)| *r <= 12 && *g <= 13 && *b <= 14)
        })
        .map(|game| game.id)
        .sum();

    let part_2: u32 = games
        .iter()
        .map(|game| {
            let (r, g, b) = game.sets.iter().fold((0, 0, 0), |(ar, ag, ab), (r, g, b)| {
                (ar.max(*r), ag.max(*g), ab.max(*b))
            });
            r * g * b
        })
        .sum();
    println!("{part_1}");
    println!("{part_2}");
}
