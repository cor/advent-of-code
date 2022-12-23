use std::collections::HashMap;

use aoc_2022_common::challenge_input;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{i64, line_ending},
    combinator::{iterator, map, opt},
    multi::fold_many0,
    sequence::{separated_pair, terminated},
    IResult,
};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct MonkeyId<'a>(&'a str);

impl<'a> MonkeyId<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        map(take(4usize), Self)(input)
    }
}

#[derive(Debug, Clone)]
enum Monkey<'a> {
    Num(i64),
    Add(MonkeyId<'a>, MonkeyId<'a>),
    Sub(MonkeyId<'a>, MonkeyId<'a>),
    Mul(MonkeyId<'a>, MonkeyId<'a>),
    Div(MonkeyId<'a>, MonkeyId<'a>),
}

impl<'a> Monkey<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            map(i64, Self::Num),
            map(
                separated_pair(MonkeyId::parse, tag(" + "), MonkeyId::parse),
                |(lhs, rhs)| Self::Add(lhs, rhs),
            ),
            map(
                separated_pair(MonkeyId::parse, tag(" - "), MonkeyId::parse),
                |(lhs, rhs)| Self::Sub(lhs, rhs),
            ),
            map(
                separated_pair(MonkeyId::parse, tag(" * "), MonkeyId::parse),
                |(lhs, rhs)| Self::Mul(lhs, rhs),
            ),
            map(
                separated_pair(MonkeyId::parse, tag(" / "), MonkeyId::parse),
                |(lhs, rhs)| Self::Div(lhs, rhs),
            ),
        ))(input)
    }

    pub fn parse_with_id(input: &'a str) -> IResult<&str, (MonkeyId, Self)> {
        separated_pair(MonkeyId::parse, tag(": "), Self::parse)(input)
    }

    pub fn parse_map(input: &'a str) -> IResult<&str, HashMap<MonkeyId, Self>> {
        fold_many0(
            terminated(Self::parse_with_id, opt(line_ending)),
            HashMap::new,
            |mut map, (id, monkey)| {
                map.insert(id, monkey);
                map
            },
        )(input)
    }
}

fn main() {
    let input = challenge_input();
    let (_, monkeys) = Monkey::parse_map(&input).unwrap();
    dbg!(monkeys);
}
