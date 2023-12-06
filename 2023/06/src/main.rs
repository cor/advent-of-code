use aoc_2023_common::challenge_input;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn parse_many(input: &str) -> IResult<&str, Vec<Race>> {
        map(
            separated_pair(
                preceded(preceded(tag("Time:"), space1), separated_list1(space1, u64)),
                line_ending,
                preceded(
                    preceded(tag("Distance:"), space1),
                    separated_list1(space1, u64),
                ),
            ),
            |(times, distances)| {
                times
                    .iter()
                    .zip(distances.iter())
                    .map(|(&time, &distance)| Race { time, distance })
                    .collect::<Vec<_>>()
            },
        )(input)
    }

    fn parse_one(input: &str) -> IResult<&str, Race> {
        map(
            separated_pair(
                preceded(
                    preceded(tag("Time:"), space1),
                    separated_list1(space1, digit1),
                ),
                line_ending,
                preceded(
                    preceded(tag("Distance:"), space1),
                    separated_list1(space1, digit1),
                ),
            ),
            |(times, distances)| {
                let time: u64 = times.join("").parse().unwrap();
                let distance: u64 = distances.join("").parse().unwrap();
                Race { time, distance }
            },
        )(input)
    }

    fn solve(&self) -> (f64, f64) {
        let time = self.time as f64;
        let target_distance = self.distance as f64 + 1.0;

        // Solve quadratic formula
        let d: f64 = time.powi(2) - 4.0 * target_distance;
        ((time - d.sqrt()) / 2.0, (time + d.sqrt()) / 2.0)
    }

    fn winning_options(&self) -> u64 {
        let (fst, snd) = self.solve();
        (snd.floor() - fst.ceil() + 1.0) as u64
    }
}

fn main() {
    let input = challenge_input();
    let races = Race::parse_many(&input).expect("Invalid input").1;
    let part_1: u64 = races.iter().map(Race::winning_options).product();
    println!("{}", part_1);

    let race = Race::parse_one(&input).expect("Invalid input").1;
    let part_2 = race.winning_options();
    println!("{}", part_2);
}
