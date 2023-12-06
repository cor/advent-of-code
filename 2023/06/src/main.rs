use aoc_2023_common::challenge_input;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u32},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    sequence::separated_pair,
    IResult,
};

// d = duration
// x = press_duration
// o = old_record
//
// s = (d-x)*x
//   = dx - x^2
// o = dx - x^2
// -o + dx - x^2 = 0
//
// a = -1
// b = d
// c = -o
// D = d^2 - 4
// D = d^2 - 4 * -o * -1
//   = d^2 - 4o

// x = -d -sqrt(D)
#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn parse_many(input: &str) -> IResult<&str, Vec<Race>> {
        map(
            separated_pair(
                preceded(preceded(tag("Time:"), space1), separated_list1(space1, u32)),
                line_ending,
                preceded(
                    preceded(tag("Distance:"), space1),
                    separated_list1(space1, u32),
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

    fn solve(&self) -> (f64, f64) {
        let duration = self.time as f64;
        let old_record = self.distance as f64;
        let target_record = old_record + 1.0;
        let d: f64 = duration.powi(2) - 4.0 * target_record;

        let fst: f64 = (duration - d.sqrt()) / 2.0;
        let snd: f64 = (duration + d.sqrt()) / 2.0;
        (fst, snd)
    }

    fn winning_options(&self) -> u32 {
        let (fst, snd) = self.solve();
        (snd.floor() - fst.ceil() + 1.0) as u32
    }
}

fn main() {
    let input = challenge_input();
    let races = Race::parse_many(&input).expect("Invalid input").1;
    let part_1: u32 = races.iter().map(|race| race.winning_options()).product();
    println!("{:?}", part_1);

    // let solution = solve(7.0, 9.0);
    // let winning = winning_options(solution);
    // dbg!(solution, winning);
    // let solution = solve(15.0, 40.0);
    // let winning = winning_options(solution);
    // dbg!(solution, winning);
    // let solution = solve(30.0, 200.0);
    // let winning = winning_options(solution);
    // dbg!(solution, winning);
}

fn solve(duration: f64, old_record: f64) -> (f64, f64) {
    let target_record = old_record + 1.0;
    let d: f64 = duration.powi(2) - 4.0 * target_record;

    let fst: f64 = (duration - d.sqrt()) / 2.0;
    let snd: f64 = (duration + d.sqrt()) / 2.0;
    (fst, snd)
}

fn winning_options(solution: (f64, f64)) -> u64 {
    let (fst, snd) = solution;
    (snd.floor() - fst.ceil() + 1.0) as u64
}
