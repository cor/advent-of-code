use aoc_2020_common::common::load_file;
use std::str::FromStr;
use crate::Bus::Unavailable;
use std::num::ParseIntError;


#[derive(Debug)]
enum Bus {
    Unavailable,
    ID(u64),
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

fn main() {
    let input = load_file("./input/1.txt");
    let mut lines = input.lines().into_iter();
    println!("{}", input);

    let departure_time = lines.next().unwrap().parse::<u64>();

    let busses: Vec<Bus> = lines.next()
        .unwrap()
        .split(',')
        .map(Bus::from_str)
        .filter_map(Result::ok)
        .collect();

    dbg!(busses);
}
