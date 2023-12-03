use std::{collections::HashMap, str::FromStr};

use aoc_2023_common::challenge_input;

#[derive(Debug)]
struct Schematic {
    fields: Vec<Vec<char>>,
    width: i32,
    height: i32,
}

impl FromStr for Schematic {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        let height = fields.len() as i32;
        let width = fields.first().unwrap().len() as i32;
        Ok(Schematic {
            fields,
            width,
            height,
        })
    }
}

impl Schematic {
    fn get_field(&self, index: (i32, i32)) -> Option<char> {
        if index.0 >= 0 && index.0 < self.width && index.1 >= 0 && index.1 < self.height {
            Some(self.fields[index.1 as usize][index.0 as usize])
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PartNumber {
    number: usize,
    x: (usize, usize),
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum PartType {
    Invalid,
    Part,
    Gear((i32, i32)),
}

const NEIGHBORS_DELTAS: [(i32, i32); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

impl PartNumber {
    fn typ(&self, schematic: &Schematic) -> PartType {
        for x in self.x.0..=self.x.1 {
            for (dx, dy) in NEIGHBORS_DELTAS {
                let neighbor = (x as i32 + dx, self.y as i32 + dy);
                match schematic.get_field(neighbor) {
                    Some('*') => return PartType::Gear(neighbor),
                    Some('0'..='9' | '.') | None => {}
                    _ => return PartType::Part,
                }
            }
        }
        PartType::Invalid
    }
}

fn find_part_numbers(schematic: &Schematic) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = Vec::new();

    for (y, line) in schematic.fields.iter().enumerate() {
        let mut char_iter = line.iter().enumerate();
        while let Some((x0, &char)) = char_iter.next() {
            if char.is_ascii_digit() {
                let mut number_digits = vec![char];
                let mut x1 = x0;
                while let Some((char_index_inner, &char_inner @ '0'..='9')) = char_iter.next() {
                    number_digits.push(char_inner);
                    x1 = char_index_inner;
                }
                let number = number_digits.iter().collect::<String>().parse().unwrap();
                part_numbers.push(PartNumber {
                    number,
                    x: (x0, x1),
                    y,
                });
            }
        }
    }
    part_numbers
}

fn main() {
    let input = challenge_input();
    let schematic = Schematic::from_str(&input).unwrap();
    let part_numbers = find_part_numbers(&schematic);
    let part_1: usize = part_numbers
        .iter()
        .filter(|part_number| !matches!(part_number.typ(&schematic), PartType::Invalid))
        .map(|part| part.number)
        .sum();

    println!("{part_1}");

    let mut gears: HashMap<(i32, i32), Vec<PartNumber>> = HashMap::new();
    for part in part_numbers {
        if let PartType::Gear(field) = part.typ(&schematic) {
            gears.entry(field).or_default().push(part);
        };
    }

    let part_2: usize = gears
        .iter()
        .filter(|(_, gears)| gears.len() == 2)
        .map(|(_, gears)| gears.iter().map(|g| g.number).product::<usize>())
        .sum();

    println!("{part_2}");
}
