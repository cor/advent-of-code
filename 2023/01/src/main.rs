use aoc_2023_common::challenge_input;

const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

enum Direction {
    Left,
    Right,
}

use Direction::{Left, Right};

fn first_digit_1(line: &str, direction: Direction) -> Option<u32> {
    let mut chars = line.chars();
    match direction {
        Left => chars.find(|c| c.is_numeric()),
        Right => chars.rfind(|c| c.is_numeric()),
    }
    .and_then(|c| c.to_digit(10))
}

fn first_digit_2(line: &str, direction: Direction) -> Option<u32> {
    let digit_word_index = DIGIT_WORDS
        .map(|w| match direction {
            Left => line.find(w),
            Right => line.rfind(w),
        })
        .iter()
        .enumerate()
        .fold((0, None), |(digit_l, index_l), (digit_r, index_r)| {
            match (index_l, index_r) {
                (None, None) => (0, None),
                (None, Some(index_r)) => (digit_r, Some(*index_r)),
                (Some(index_l), None) => (digit_l, Some(index_l)),
                (Some(index_l), Some(index_r)) => match direction {
                    Left if index_l > *index_r => (digit_r, Some(*index_r)),
                    Right if index_l < *index_r => (digit_r, Some(*index_r)),
                    _ => (digit_l, Some(index_l)),
                },
            }
        });

    let digit_word_value = digit_word_index.0 as u32 + 1; // offset const indices
    let digit_word_index = digit_word_index.1;

    let digit_number_index = match direction {
        Left => line.chars().enumerate().find(|(_, c)| c.is_numeric()),
        Right => {
            let number_index = line.chars().enumerate().collect::<Vec<_>>();
            number_index
                .iter()
                .rev()
                .cloned()
                .find(|(_, c)| c.is_numeric())
        }
    };

    match (digit_word_index, digit_number_index) {
        (None, None) => None,
        (None, Some((_, char))) => char.to_digit(10),
        (Some(_), None) => Some(digit_word_value),
        (Some(index_word), Some((index_number, char))) => match direction {
            Left if index_word < index_number => Some(digit_word_value),
            Right if index_word > index_number => Some(digit_word_value),
            _ => char.to_digit(10),
        },
    }
}

fn calibration_value_1(line: &str) -> Option<u32> {
    let left = first_digit_1(line, Left)?;
    let right = first_digit_1(line, Right)?;
    Some(left * 10 + right)
}

fn calibration_value_2(line: &str) -> Option<u32> {
    let left = first_digit_2(line, Left)?;
    let right = first_digit_2(line, Right)?;
    Some(left * 10 + right)
}

fn main() {
    let input = challenge_input();
    let lines = input.lines().collect::<Vec<_>>();
    let part_1: u32 = lines
        .iter()
        .filter_map(|line| calibration_value_1(line))
        .sum();
    let part_2: u32 = lines
        .iter()
        .filter_map(|line| calibration_value_2(line))
        .sum();
    println!("{part_1}");
    println!("{part_2}");
}
