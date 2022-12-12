#[cfg(test)]
pub mod tests;
use aoc_2022_common::challenge_input;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u64,
    character::{complete::line_ending, streaming::char},
    combinator::{map, success},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = challenge_input();
    let (_, monkeys) = Monkey::parse_many(&input).expect("Invalid monkey(s) in input!");

    println!("{}", Monkey::business(monkeys.clone(), true));
    println!("{}", Monkey::business(monkeys, false));
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Monkey {
    pub items: Vec<u64>,
    pub operation: Operation,
    pub test: u64,
    pub targets: (usize, usize),
    pub inspected: u64,
}

impl Monkey {
    pub fn parse(input: &str) -> IResult<&str, Monkey> {
        let (s, (items, operation, test, targets)) = preceded(
            delimited(tag("Monkey "), u64, tag(":\n")),
            tuple((
                parse_starting_items,
                Operation::parse,
                parse_test,
                parse_targets,
            )),
        )(input)?;

        #[allow(clippy::cast_possible_truncation)] // We know that the input wont truncate
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

    pub fn business(mut monkeys: Vec<Monkey>, part_1: bool) -> u64 {
        let total_multiplier: u64 = monkeys.iter().map(|m| m.test).product();

        let rounds = if part_1 { 20 } else { 10_000 };

        for _ in 0..rounds {
            for i in 0..monkeys.len() {
                while let Some(mut item) = monkeys[i].items.pop() {
                    item = monkeys[i].operation.apply(item);

                    // prevent ourselves from going crazy
                    if part_1 {
                        item /= 3;
                    } else {
                        item %= total_multiplier;
                    }

                    let target = if item % monkeys[i].test == 0 {
                        monkeys[i].targets.0
                    } else {
                        monkeys[i].targets.1
                    };

                    monkeys[target].items.push(item);
                    monkeys[i].inspected += 1;
                }
            }
        }

        let mut inspection_scores: Vec<u64> = monkeys.iter().map(|m| m.inspected).collect();
        inspection_scores.sort_unstable();
        inspection_scores.reverse();
        inspection_scores[0] * inspection_scores[1]
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Operation {
    Add(u64),
    Times(u64),
    Square,
}

impl Operation {
    pub fn parse(input: &str) -> IResult<&str, Operation> {
        delimited(
            tag("  Operation: new = "),
            alt((
                preceded(tag("old * old"), success(Operation::Square)),
                map(preceded(tag("old * "), u64), Operation::Times),
                map(preceded(tag("old + "), u64), Operation::Add),
            )),
            line_ending,
        )(input)
    }

    pub fn apply(&self, to: u64) -> u64 {
        match self {
            Operation::Add(n) => to + n,
            Operation::Times(n) => to * n,
            Operation::Square => to * to,
        }
    }
}

fn parse_starting_items(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(
        tag("  Starting items: "),
        separated_list0(tag(", "), u64),
        char('\n'),
    )(input)
}

fn parse_test(input: &str) -> IResult<&str, u64> {
    delimited(tag("  Test: divisible by "), u64, line_ending)(input)
}

fn parse_targets(input: &str) -> IResult<&str, (u64, u64)> {
    preceded(
        tag("    If true: throw to monkey "),
        separated_pair(u64, tag("\n    If false: throw to monkey "), u64),
    )(input)
}
