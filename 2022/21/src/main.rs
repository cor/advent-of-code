use std::{collections::HashMap, mem};

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
    let (_, monkeys) = Monkey::parse_map(&input).unwrap();
    let root = MonkeyId("root");
    let part_1 = &monkeys[&root].value(&monkeys);
    println!("{part_1}");

    let (mut human_expr, mut other_expr) = Expr::from_monkeys(&monkeys);
    println!("{:?}", human_expr.contains_human());
    println!("{:?}", other_expr.contains_human());
    if !human_expr.contains_human() {
        mem::swap(&mut human_expr, &mut other_expr);
    }

    while human_expr != Expr::Human {
        (human_expr, other_expr) = simplify_expr(human_expr, other_expr);
    }

    dbg!(other_expr);
}

fn simplify_expr(human: Expr, other: Expr) -> (Expr, Expr) {
    let new_human: Expr;
    let new_other: Expr;

    let other_box = Box::new(other);

    match &human {
        Expr::Human => panic!("attempt to simplify human"),
        Expr::Num(_) => panic!("attempt to simplify number"),
        Expr::Add(lhs, rhs) if lhs.contains_human() => {
            new_human = *lhs.to_owned();
            new_other = Expr::Sub(other_box, rhs.to_owned());
        }
        Expr::Add(lhs, rhs) => {
            new_human = *rhs.to_owned();
            new_other = Expr::Sub(other_box, lhs.to_owned());
        }
        Expr::Sub(lhs, rhs) if lhs.contains_human() => {
            new_human = *lhs.to_owned();
            new_other = Expr::Add(other_box, rhs.to_owned());
        }
        Expr::Sub(lhs, rhs) => {
            new_human = *rhs.to_owned();
            new_other = Expr::Sub(lhs.to_owned(), other_box);
        }
        Expr::Mul(lhs, rhs) if lhs.contains_human() => {
            new_human = *lhs.to_owned();
            new_other = Expr::Div(other_box, rhs.to_owned());
        }
        Expr::Mul(lhs, rhs) => {
            new_human = *rhs.to_owned();
            new_other = Expr::Div(other_box, lhs.to_owned());
        }
        Expr::Div(lhs, rhs) if lhs.contains_human() => {
            new_human = *lhs.to_owned();
            new_other = Expr::Mul(other_box, rhs.to_owned());
        }
        Expr::Div(lhs, rhs) => {
            new_human = *rhs.to_owned();
            new_other = Expr::Div(lhs.to_owned(), other_box);
        }
    }

    (new_human, new_other)
}

/// I could've reused Monkey, but would rather have
/// a recursive structure than a HashMap for part 2
#[derive(Debug, Clone, Eq, PartialEq)]
enum Expr {
    Human,
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn from_monkeys(monkeys: &HashMap<MonkeyId, Monkey>) -> (Expr, Expr) {
        let root = MonkeyId("root");
        let Monkey::Add(lhs, rhs) = &monkeys[&root].clone() else {
            panic!("Invalid rooot monkey for part 2");
        };
        let left_expr = Expr::from_monkey(&monkeys[lhs], monkeys);
        let right_expr = Expr::from_monkey(&monkeys[rhs], monkeys);

        (left_expr, right_expr)
    }

    fn from_monkey(monkey: &Monkey, monkeys: &HashMap<MonkeyId, Monkey>) -> Expr {
        match monkey {
            Monkey::Num(n) => Expr::Num(*n),
            Monkey::Add(lhs, rhs) => Expr::Add(
                Box::new(Expr::human_or_monkey(lhs, monkeys)),
                Box::new(Expr::human_or_monkey(rhs, monkeys)),
            ),
            Monkey::Sub(lhs, rhs) => Expr::Sub(
                Box::new(Expr::human_or_monkey(lhs, monkeys)),
                Box::new(Expr::human_or_monkey(rhs, monkeys)),
            ),
            Monkey::Mul(lhs, rhs) => Expr::Mul(
                Box::new(Expr::human_or_monkey(lhs, monkeys)),
                Box::new(Expr::human_or_monkey(rhs, monkeys)),
            ),
            Monkey::Div(lhs, rhs) => Expr::Div(
                Box::new(Expr::human_or_monkey(lhs, monkeys)),
                Box::new(Expr::human_or_monkey(rhs, monkeys)),
            ),
        }
    }
    fn human_or_monkey(input: &MonkeyId, monkeys: &HashMap<MonkeyId, Monkey>) -> Expr {
        match input {
            MonkeyId("humn") => Expr::Human,
            id => Expr::from_monkey(&monkeys[id], monkeys),
        }
    }

    fn contains_human(&self) -> bool {
        match self {
            Expr::Human => true,
            Expr::Num(_) => false,
            Expr::Add(lhs, rhs)
            | Expr::Sub(lhs, rhs)
            | Expr::Mul(lhs, rhs)
            | Expr::Div(lhs, rhs) => lhs.contains_human() || rhs.contains_human(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn simplify_add() {
        let human_expr = Expr::Add(Box::new(Expr::Human), Box::new(Expr::Num(4)));
        let other_expr = Expr::Num(2);

        let (new_human_expr, new_other_expr) = simplify_expr(human_expr, other_expr);

        assert_eq!(new_human_expr, Expr::Human);
        assert_eq!(
            new_other_expr,
            Expr::Sub(Box::new(Expr::Num(2)), Box::new(Expr::Num(4)))
        );
    }
}
