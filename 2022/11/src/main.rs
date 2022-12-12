use aoc_2022_common::challenge_input;

#[cfg(test)]
pub mod tests;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u64,
    character::{complete::newline, streaming::char},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = challenge_input();
    let (_, mut monkeys) = Monkey::parse_many(&input).expect("Invalid input!");

    let total_multiplier: u64 = monkeys.iter().map(|m| m.test).product();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop() {
                item = match monkeys[i].operation {
                    Op::Add(n) => item + n,
                    Op::Times(n) => item * n,
                    Op::Square => item * item,
                };

                // prevent ourselves from going crazy
                item %= total_multiplier;
                // item /= 3;

                let monkey = monkeys[i].clone();
                if item % monkey.test == 0 {
                    monkeys[monkey.targets.0].items.push(item)
                } else {
                    monkeys[monkey.targets.1].items.push(item)
                }

                monkeys[i].inspected += 1;
            }
        }
    }

    let mut inspection_scores: Vec<u64> = monkeys.iter().map(|m| m.inspected).collect();
    inspection_scores.sort();
    inspection_scores.reverse();
    let monkey_business = inspection_scores[0] * inspection_scores[1];

    println!("{}", monkey_business);
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Monkey {
    pub items: Vec<u64>,
    pub operation: Op,
    pub test: u64,
    pub targets: (usize, usize),
    pub inspected: u64,
}

impl Monkey {
    pub fn parse(input: &str) -> IResult<&str, Monkey> {
        let (s, (items, operation, test, targets)) = preceded(
            delimited(tag("Monkey "), u64, tag(":\n")),
            tuple((parse_starting_items, Op::parse, parse_test, parse_targets)),
        )(input)?;

        Ok((
            s,
            Monkey {
                items,
                operation,
                test,
                targets: (targets.0 as usize, targets.1 as usize),
                inspected: 0,
            },
        ))
    }

    pub fn parse_many(input: &str) -> IResult<&str, Vec<Monkey>> {
        separated_list0(tag("\n\n"), Monkey::parse)(input)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Op {
    Add(u64),
    Times(u64),
    Square,
}

fn parse_starting_items(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(
        tag("  Starting items: "),
        separated_list0(tag(", "), u64),
        char('\n'),
    )(input)
}

impl Op {
    pub fn parse(input: &str) -> IResult<&str, Op> {
        delimited(
            tag("  Operation: new = "),
            alt((
                map(tag("old * old"), |_| Op::Square),
                map(preceded(tag("old * "), u64), Op::Times),
                map(preceded(tag("old + "), u64), Op::Add),
            )),
            newline,
        )(input)
    }
}

fn parse_test(input: &str) -> IResult<&str, u64> {
    delimited(tag("  Test: divisible by "), u64, newline)(input)
}

fn parse_targets(input: &str) -> IResult<&str, (u64, u64)> {
    preceded(
        tag("    If true: throw to monkey "),
        separated_pair(u64, tag("\n    If false: throw to monkey "), u64),
    )(input)
}
