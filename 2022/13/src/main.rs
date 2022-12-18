use std::cmp::Ordering;

use aoc_2022_common::challenge_input;
use nom::{
    branch::alt,
    character::complete::i64,
    character::{complete::line_ending, streaming::char},
    combinator::map,
    multi::{count, separated_list0},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Integer(i64),
}

impl Packet {
    pub fn parse(input: &str) -> IResult<&str, Packet> {
        alt((
            map(i64, Self::Integer),
            delimited(
                char('['),
                map(separated_list0(char(','), Self::parse), Self::List),
                char(']'),
            ),
        ))(input)
    }

    pub fn parse_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
        separated_pair(Self::parse, line_ending, Self::parse)(input)
    }

    pub fn parse_pair_list0(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
        separated_list0(count(line_ending, 2), Self::parse_pair)(input)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(lhs), Packet::Integer(rhs)) => lhs.cmp(rhs),
            (Packet::List(lhs), Packet::List(rhs)) => {
                for (l, r) in lhs.iter().zip(rhs.iter()) {
                    match l.cmp(r) {
                        Ordering::Equal => {}
                        res => return res,
                    }
                }
                if rhs.len() < lhs.len() {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            (lhs, Packet::Integer(rhs)) => lhs.cmp(&Packet::List(vec![Packet::Integer(*rhs)])),
            (Packet::Integer(lhs), rhs) => Packet::List(vec![Packet::Integer(*lhs)]).cmp(rhs),
        }
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = challenge_input();
    let (_, packets) = Packet::parse_pair_list0(&input).expect("invalid packets in input");
    // dbg!(&packets);
    let res = &packets
        .iter()
        .map(|(lhs, rhs)| lhs.cmp(rhs))
        .enumerate()
        .map(|(index, ord)| (index + 1, ord))
        .filter(|(_, ord)| *ord != Ordering::Greater)
        .map(|(index, _)| index)
        .sum::<usize>();

    println!("{}", res);
}
