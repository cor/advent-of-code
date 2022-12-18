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

#[derive(Debug)]
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

fn main() {
    let input = challenge_input();
    let packets = Packet::parse_pair_list0(&input);
    dbg!(packets);
    // println!("{}", input);
}
