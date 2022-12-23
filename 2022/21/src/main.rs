use std::collections::HashMap;

use aoc_2022_common::challenge_input;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{i64, line_ending},
    combinator::{map, opt, success},
    multi::fold_many0,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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

type MonkeyOp<'a> = fn(MonkeyId<'a>, MonkeyId<'a>) -> Monkey<'a>;

impl<'a> Monkey<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            map(i64, Self::Num),
            map(
                tuple((
                    MonkeyId::parse,
                    alt((
                        preceded(tag(" + "), success(Self::Add as MonkeyOp)),
                        preceded(tag(" - "), success(Self::Sub as MonkeyOp)),
                        preceded(tag(" * "), success(Self::Mul as MonkeyOp)),
                        preceded(tag(" / "), success(Self::Div as MonkeyOp)),
                    )),
                    MonkeyId::parse,
                )),
                |(lhs, op, rhs)| op(lhs, rhs),
            ),
        ))(input)
    }

    pub fn parse_with_id(input: &'a str) -> IResult<&str, (MonkeyId, Self)> {
        separated_pair(MonkeyId::parse, tag(": "), Self::parse)(input)
    }

    pub fn parse_map(input: &'a str) -> IResult<&str, HashMap<MonkeyId, Self>> {
        fold_many0(
            terminated(Self::parse_with_id, opt(line_ending)),
            || HashMap::with_capacity(3000),
            |mut map, (id, monkey)| {
                map.insert(id, monkey);
                map
            },
        )(input)
    }

    pub fn value(&self, others: &HashMap<MonkeyId, Monkey>) -> i64 {
        match self {
            Monkey::Num(n) => *n,
            Monkey::Add(lhs, rhs) => others[lhs].value(others) + others[rhs].value(others),
            Monkey::Sub(lhs, rhs) => others[lhs].value(others) - others[rhs].value(others),
            Monkey::Mul(lhs, rhs) => others[lhs].value(others) * others[rhs].value(others),
            Monkey::Div(lhs, rhs) => others[lhs].value(others) / others[rhs].value(others),
        }
    }
}

fn main() {
    let input = challenge_input();
    let (_, mut monkeys) = Monkey::parse_map(&input).unwrap();
    let root = MonkeyId("root");
    let part_1 = &monkeys[&root].value(&monkeys);
    println!("{part_1}");

    let Monkey::Add(lhs, rhs) = &monkeys[&root].clone() else {
        panic!("Invalid rooot monkey for part 2");
    };

    let humn = MonkeyId("humn");

    for i in 0..i64::MAX {
        monkeys.insert(humn, Monkey::Num(i));

        if monkeys[lhs].value(&monkeys) == monkeys[rhs].value(&monkeys) {
            println!("Gottem: {i}");
        }

        if i % 100_000 == 0 {
            println!("{i}");
        }
    }
}
