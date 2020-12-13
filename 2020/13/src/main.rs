use aoc_2020_common::common::load_file;
use std::str::FromStr;
use crate::Bus::Unavailable;
use std::num::ParseIntError;
use ring_algorithm::chinese_remainder_theorem;


/// Part 1, done with iterators

#[derive(Debug)]
enum Bus {
    Unavailable,
    ID(u64),
}

/// For use with functions on Option such as `filter_map`
impl Bus {
    fn available(&self) -> Option<u64> {
        match self {
            Bus::ID(n) => Some(*n),
            Bus::Unavailable => None
        }
    }
}

/// Overly complex, but I wanted to try out making a custom Iterator
#[derive(Debug)]
struct StopIterator {
    bus_id: u64,
    index: u64,
}

impl Iterator for StopIterator {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.index += 1;
        Some(self.index * self.bus_id)
    }
}

impl FromStr for Bus {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Unavailable),
            s => {
                match s.parse::<u64>() {
                    Ok(n) => Ok(Bus::ID(n)),
                    Err(e) => Err(e),
                }
            }
        }
    }
}


fn part_1(input: &str) -> u64 {
    let mut lines = input.lines().into_iter();

    let earliest_time = lines.next().unwrap().parse::<u64>().unwrap();

    // Parse input
    let busses: Vec<Bus> = lines.next()
        .unwrap()
        .split(',')
        .map(Bus::from_str)
        .filter_map(Result::ok)
        .collect();

    // Create an StopIterator for every Bus that is available
    let mut stop_iters: Vec<StopIterator> = busses
        .iter()
        .filter_map(Bus::available)
        .map(|id| StopIterator {
            bus_id: id,
            index: 0,
        })
        .collect();


    let (bus_id, departure_time) = stop_iters
        .iter_mut()
        .map(| s| {
            loop {
                let stop = s.next().unwrap();
                if stop >= earliest_time {
                    return (s.bus_id, stop)
                }
            }
        })
        .min_by_key(|(_, stop)| stop.clone())
        .unwrap();


    bus_id * (departure_time - earliest_time)
}


/// Part 2


#[derive(Debug)]
struct Bus2 {
    id: i64,
    index: i64,
}

impl Bus2 {
    fn residues_modulii(&self) -> (i64, i64) {
        (self.id - self.index, self.id)
    }
}

fn parse_busses(s: &str) -> Vec<Bus2> {
    let snd_line: &str = s.lines().collect::<Vec<&str>>()[1];

    snd_line
        .split(',')
        .enumerate()
        .filter_map(|(index, s)| {
            match s {
                "x" => None,
                n => Some(Bus2 {
                    index: index as i64,
                    id: n.parse::<i64>().unwrap()
                })
            }
        })
        .collect()
}

fn part_2(input: &str) -> u64 {
    let busses = parse_busses(input);

    let (residues, modulii): (Vec<i64>, Vec<i64>) = busses
        .iter()
        .map(Bus2::residues_modulii)
        .unzip();

    chinese_remainder_theorem(&residues, &modulii).unwrap() as u64
}

fn main() {
    let input = load_file("./input/1.txt");

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}
