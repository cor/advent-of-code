use std::cmp::Ordering;

use aoc_2022_common::challenge_input;
use nom::{
    branch::alt,
    character::complete::u64,
    character::{complete::line_ending, streaming::char},
    combinator::map,
    multi::{count, many0, separated_list0},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Integer(u64),
}

impl Packet {
    pub fn parse(input: &str) -> IResult<&str, Packet> {
        alt((
            map(u64, Self::Integer),
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

    pub fn parse_list0(input: &str) -> IResult<&str, Vec<Packet>> {
        separated_list0(line_ending, preceded(many0(line_ending), Self::parse))(input)
    }

    pub fn new_divider(number: u64) -> Self {
        Packet::List(vec![Packet::List(vec![Packet::Integer(number)])])
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(lhs), Packet::Integer(rhs)) => lhs.cmp(rhs),
            (Packet::List(lhs), Packet::List(rhs)) => {
                for i in 0..(usize::max(lhs.len(), rhs.len())) {
                    match (lhs.get(i), rhs.get(i)) {
                        (Some(left), Some(right)) => match left.cmp(right) {
                            Ordering::Equal => continue,
                            ord => return ord,
                        },
                        (None, _) => return Ordering::Less,
                        (_, None) => return Ordering::Greater,
                    }
                }
                Ordering::Equal
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

#[must_use]
pub fn part_1(input: &str) -> usize {
    Packet::parse_pair_list0(input)
        .expect("invalid packets in input")
        .1
        .iter()
        .map(|(lhs, rhs)| lhs.cmp(rhs))
        .enumerate()
        .map(|(index, ord)| (index + 1, ord))
        .filter(|(_, ord)| *ord != Ordering::Greater)
        .map(|(index, _)| index)
        .sum::<usize>()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let (_, mut packets) = Packet::parse_list0(input).expect("invalid packets in input");
    let divider_packet_2 = Packet::new_divider(2);
    let divider_packet_6 = Packet::new_divider(6);
    packets.push(divider_packet_2.clone());
    packets.push(divider_packet_6.clone());
    packets.sort_unstable();

    let divider_2_index = packets.iter().position(|p| *p == divider_packet_2).unwrap() + 1;
    let divider_6_index = packets.iter().position(|p| *p == divider_packet_6).unwrap() + 1;

    divider_2_index * divider_6_index
}

fn main() {
    let input = challenge_input();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}
